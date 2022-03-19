""" This module is the start of the deployment for SBOM-API """

from os import system, getenv, path

import aws_cdk as cdk
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_apigateway as apigw
import aws_cdk.aws_ecs as ecs
import aws_cdk.aws_efs as efs
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_s3_notifications as s3n
import aws_cdk.aws_sqs as sqs
from aws_cdk import Duration
from aws_cdk import Stack
from aws_cdk.aws_lambda_event_sources import SqsEventSource
from constructs import Construct

from cyclonedx.constants import SBOM_BUCKET_NAME_EV, DT_QUEUE_URL_EV
from scripts.constants import (
    PUBLIC,
    PRIVATE,
    STACK_ID,
    DT_CONTAINER_ID,
    FARGATE_CLUSTER_ID,
    BUCKET_NAME,
    ENRICHMENT_LAMBDA_NAME,
    EFS_MOUNT_ID,
    INGEST_LAMBDA_NAME,
    PRIVATE_SUBNET_NAME,
    PUBLIC_SUBNET_NAME,
    DT_LAMBDA_NAME,
    DT_TASK_DEF_ID,
    VPC_NAME,
    CIDR,
    DT_SBOM_QUEUE_NAME,
)

# Get the Current Working Directory so we can construct a path to the
# zip file for the Lambdas
cwd = path.dirname(__file__)
code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)


class SBOMApiDeploy(Stack):

    """This class is where the infrastructure to run the application
    is built.  This class inherits from the Stack class, which is part of
    the AWS CDK."""

    def __configure_ingest_func(self, vpc, bucket) -> None:

        """Create the Lambda Function to do the work
        and set permissions on the S3 Bucket"""

        sbom_ingest_func = lambda_.Function(
            self,
            INGEST_LAMBDA_NAME,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.store_handler",
            code=code,
            environment={SBOM_BUCKET_NAME_EV: bucket.bucket_name},
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        bucket.grant_put(sbom_ingest_func)

        lambda_api = apigw.LambdaRestApi(
            self, "sbom_ingest_api", handler=sbom_ingest_func
        )

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")

    def __configure_enrichment_func(self, vpc, bucket, queue) -> None:

        """Create the Lambda Function responsible for listening on the S3 Bucket
        for SBOMs being inserted so they can be inserted into the enrichment process."""

        sbom_enrichment_func = lambda_.Function(
            self,
            ENRICHMENT_LAMBDA_NAME,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.enrichment_entry_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_EV: bucket.bucket_name,
                DT_QUEUE_URL_EV: queue.queue_url,
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        # Bucket rights granted
        bucket.grant_read(sbom_enrichment_func)

        # Grant rights to send messages to the Queue
        queue.grant_send_messages(sbom_enrichment_func)

        # Set up the S3 Bucket to send a notification to the Lambda
        # if someone puts something in the bucket. We really need to
        # think about how we should structure the file names to be
        # identifiable for our purposes #TODO
        destination = s3n.LambdaDestination(sbom_enrichment_func)
        bucket.add_event_notification(s3.EventType.OBJECT_CREATED, destination)

    def __configure_dt_func(self, vpc, queue) -> None:
        """Create the Lambda Function responsible for
        extracting results from DT given an SBOM."""

        sbom_enrichment_func = lambda_.Function(
            self,
            DT_LAMBDA_NAME,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.dt_ingress_handler",
            code=code,
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        event_source = SqsEventSource(queue)
        sbom_enrichment_func.add_event_source(event_source)

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        # Create an endpoint gateway for S3 so we can reach it from the Lambda
        s3_gateway_endpoint = create_gateway_endpoint()

        # Establish a VPC for the lambda to make calls to the services
        vpc = create_vpc(self, s3_gateway_endpoint)

        # create an ecs cluster for running dependency track
        fargate_cluster = ecs.Cluster(self, FARGATE_CLUSTER_ID, vpc=vpc)

        # create an efs mount for maintaining
        dt_mount = efs.FileSystem(self, EFS_MOUNT_ID, vpc=vpc, encrypted=True)

        dt_volume = ecs.Volume(
            name=EFS_MOUNT_ID,
            efs_volume_configuration=ecs.EfsVolumeConfiguration(
                file_system_id=dt_mount.file_system_id
            ),
        )

        dt_volume_mount = ecs.MountPoint(
            container_path="/apiserver",
            source_volume=dt_volume.name,
            read_only=False,
        )

        dt_api_task_definition = ecs.TaskDefinition(
            self,
            DT_TASK_DEF_ID,
            compatibility=ecs.Compatibility.FARGATE,
            cpu="4096",
            memory_mib="8192",
            volumes=[dt_volume],
        )

        container = dt_api_task_definition.add_container(
            DT_CONTAINER_ID,
            image=ecs.ContainerImage.from_registry("dependencytrack/apiserver"),
            logging=ecs.LogDrivers.aws_logs(stream_prefix="dependencyTrackApi"),
            environment={},
            cpu=2048,
            memory_reservation_mib=4096,
        )

        port_mapping = ecs.PortMapping(
            container_port=8080,
            host_port=8080,
            protocol=ecs.Protocol.TCP,
        )

        container.add_port_mappings(port_mapping)

        container.add_mount_points(dt_volume_mount)

        dt_service = ecs.FargateService(
            self,
            "dtService",
            cluster=fargate_cluster,
            task_definition=dt_api_task_definition,
            desired_count=1,
            assign_public_ip=True,
            platform_version=ecs.FargatePlatformVersion.VERSION1_4,
        )

        dt_mount.connections.allow_default_port_from(dt_service)
        dt_service.connections.allow_from(
            ec2.Peer.ipv4("74.134.30.50/32"), ec2.Port.tcp(8080), "Ip Whitelist"
        )

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket(self, BUCKET_NAME)

        queue = sqs.Queue(
            self, DT_SBOM_QUEUE_NAME, fifo=True, content_based_deduplication=True
        )

        self.__configure_ingest_func(vpc, bucket)
        self.__configure_enrichment_func(vpc, bucket, queue)
        self.__configure_dt_func(vpc, queue)


def create_gateway_endpoint() -> ec2.GatewayVpcEndpointOptions:
    """Creates a gateway endpoint"""
    return ec2.GatewayVpcEndpointOptions(service=ec2.GatewayVpcEndpointAwsService.S3)


def create_vpc(stack: Stack, gw_ept: ec2.GatewayVpcEndpointOptions) -> ec2.Vpc:

    """Creates a VPC"""

    private_subnet = ec2.SubnetConfiguration(
        name=PRIVATE_SUBNET_NAME, subnet_type=PRIVATE, cidr_mask=26
    )

    public_subnet = ec2.SubnetConfiguration(
        name=PUBLIC_SUBNET_NAME, subnet_type=PUBLIC, cidr_mask=26
    )

    return ec2.Vpc(
        stack,
        VPC_NAME,
        cidr=CIDR,
        max_azs=2,
        enable_dns_hostnames=True,
        enable_dns_support=True,
        gateway_endpoints={"S3": gw_ept},
        subnet_configuration=[private_subnet, public_subnet],
    )


def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    env = cdk.Environment(
        region=getenv("AWS_REGION"), account=getenv("AWS_ACCOUNT_NUM")
    )

    app = cdk.App()
    SBOMApiDeploy(app, STACK_ID, env=env)
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    system("cdk deploy")

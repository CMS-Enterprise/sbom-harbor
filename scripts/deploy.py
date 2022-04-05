""" This module is the start of the deployment for SBOM-API """

from os import system, getenv, path

import aws_cdk as cdk
import aws_cdk.aws_ssm as ssm
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_ecs as ecs
import aws_cdk.aws_efs as efs
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_s3_notifications as s3n
import aws_cdk.aws_sqs as sqs
import aws_cdk.aws_apigateway as apigwv1
import aws_cdk.aws_elasticloadbalancingv2 as elbv2

from aws_cdk import Duration
from aws_cdk import Stack
from aws_cdk.aws_lambda_event_sources import SqsEventSource
from constructs import Construct

from cyclonedx.constants import (
    ALLOW_DT_PORT_SG,
    DT_API_PORT,
    EMPTY_VALUE,
    FINDINGS_QUEUE_URL_EV,
    LOAD_BALANCER_ID,
    LOAD_BALANCER_LISTENER_ID,
    LOAD_BALANCER_TARGET_ID,
    SBOM_BUCKET_NAME_EV,
    DT_QUEUE_URL_EV,
    DT_API_BASE,
    DT_ROOT_PWD,
    DT_API_KEY,
)

from scripts.constants import (
    DT_DOCKER_ID,
    DT_FARGATE_SVC_NAME,
    DT_INSTALL_LOC,
    ENRICHMENT_EGRESS_LN,
    FINDINGS_QUEUE_NAME,
    PUBLIC,
    PRIVATE,
    STACK_ID,
    DT_CONTAINER_ID,
    FARGATE_CLUSTER_ID,
    BUCKET_NAME,
    SBOM_ENRICHMENT_LN,
    EFS_MOUNT_ID,
    PRISTINE_SBOM_INGEST_LN,
    PRIVATE_SUBNET_NAME,
    PUBLIC_SUBNET_NAME,
    DT_INTERFACE_LN,
    DT_TASK_DEF_ID,
    VPC_NAME,
    CIDR,
    DT_API_INTEGRATION,
    DT_SBOM_QUEUE_NAME,
    DT_REST_API_GATEWAY,
)

# Get the Current Working Directory so we can construct a path to the
# zip file for the Lambdas
cwd = path.dirname(__file__)
code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)


class SBOMApiStack(Stack):

    """This class is where the infrastructure to run the application
    is built.  This class inherits from the Stack class, which is part of
    the AWS CDK."""

    def __create_vpc(self, gw_ept: ec2.GatewayVpcEndpointOptions) -> ec2.Vpc:

        """Creates a VPC"""

        private_subnet = ec2.SubnetConfiguration(
            name=PRIVATE_SUBNET_NAME, subnet_type=PRIVATE, cidr_mask=26
        )

        public_subnet = ec2.SubnetConfiguration(
            name=PUBLIC_SUBNET_NAME, subnet_type=PUBLIC, cidr_mask=26
        )

        return ec2.Vpc(
            self,
            VPC_NAME,
            cidr=CIDR,
            max_azs=2,
            enable_dns_hostnames=True,
            enable_dns_support=True,
            gateway_endpoints={"S3": gw_ept},
            subnet_configuration=[private_subnet, public_subnet],
        )

    def __create_load_balancer(self, vpc):

        security_group = ec2.SecurityGroup(self, "ALB_SG", vpc=vpc)

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(),
            connection=ec2.Port.tcp(DT_API_PORT)
        )

        load_balancer = elbv2.ApplicationLoadBalancer(
            self, LOAD_BALANCER_ID, vpc=vpc,
            internet_facing=False,
            load_balancer_name=LOAD_BALANCER_ID,
            security_group=security_group
        )

        logs_s3_bucket = s3.Bucket(self, "ALB_LOGGING")
        load_balancer.log_access_logs(logs_s3_bucket)

        listener = load_balancer.add_listener(
            LOAD_BALANCER_LISTENER_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=DT_API_PORT,
        )

        listener.add_targets(
            LOAD_BALANCER_TARGET_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=DT_API_PORT,
        )

        return load_balancer, listener

    def __create_dt_fargate_svc(self, vpc, listener):

        # create an ecs cluster for running dependency track
        fargate_cluster = ecs.Cluster(self, FARGATE_CLUSTER_ID, vpc=vpc)

        # create an efs mount for maintaining
        dt_mount = efs.FileSystem(
            self,
            EFS_MOUNT_ID,
            vpc=vpc,
            encrypted=True,
        )

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
            image=ecs.ContainerImage.from_registry(DT_DOCKER_ID),
            logging=ecs.LogDrivers.aws_logs(stream_prefix="dependencyTrackApi"),
            environment={},
            cpu=4096,
            memory_reservation_mib=8192,
        )

        port_mapping = ecs.PortMapping(
            container_port=DT_API_PORT,
            host_port=DT_API_PORT,
            protocol=ecs.Protocol.TCP,
        )

        container.add_port_mappings(port_mapping)
        container.add_mount_points(dt_volume_mount)

        security_group = ec2.SecurityGroup(
            self,
            ALLOW_DT_PORT_SG,
            vpc=vpc
        )

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(),
            connection=ec2.Port.tcp(DT_API_PORT)
        )

        dt_service = ecs.FargateService(
            self,
            DT_FARGATE_SVC_NAME,
            cluster=fargate_cluster,
            task_definition=dt_api_task_definition,
            desired_count=1,
            assign_public_ip=True,
            platform_version=ecs.FargatePlatformVersion.VERSION1_4,
            security_groups=[security_group]
        )

        dt_service.register_load_balancer_targets(ecs.EcsTarget(
                container_name=DT_CONTAINER_ID,
                container_port=DT_API_PORT,
                new_target_group_id="DTTargetGroup",
                listener=ecs.ListenerConfig.application_listener(
                    listener,
                    protocol=elbv2.ApplicationProtocol.HTTP,
                    port=DT_API_PORT,
                ),
            )
        )

        dt_mount.connections.allow_default_port_from(dt_service)
        dt_service.connections.allow_from(
            ec2.Peer.ipv4("74.134.30.50/32"),
            ec2.Port.tcp(DT_API_PORT),
            "Ip Whitelist",
        )

    def __conf_pristine_sbom_ingest_func(self, vpc, bucket) -> None:

        """Create the Lambda Function to do the work
        and set permissions on the S3 Bucket"""

        sbom_ingest_func = lambda_.Function(
            self,
            PRISTINE_SBOM_INGEST_LN,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.pristine_sbom_ingress_handler",
            code=code,
            environment={SBOM_BUCKET_NAME_EV: bucket.bucket_name},
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        bucket.grant_put(sbom_ingest_func)

        lambda_api = apigwv1.LambdaRestApi(
            self, "sbom_ingest_api", handler=sbom_ingest_func
        )

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")

    def __conf_enrichment_ingress_func(self, vpc, bucket, dt_ingress_queue) -> None:

        """Create the Lambda Function responsible for listening on the S3 Bucket
        for SBOMs being inserted so they can be inserted into the enrichment process."""

        sbom_enrichment_ingress_func = lambda_.Function(
            self,
            SBOM_ENRICHMENT_LN,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.enrichment_ingress_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_EV: bucket.bucket_name,
                DT_QUEUE_URL_EV: dt_ingress_queue.queue_url,
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        # Bucket rights granted
        bucket.grant_read(sbom_enrichment_ingress_func)

        # Grant rights to send messages to the Queue
        dt_ingress_queue.grant_send_messages(sbom_enrichment_ingress_func)

        # Set up the S3 Bucket to send a notification to the Lambda
        # if someone puts something in the bucket. We really need to
        # think about how we should structure the file names to be
        # identifiable for our purposes #TODO
        destination = s3n.LambdaDestination(sbom_enrichment_ingress_func)
        bucket.add_event_notification(s3.EventType.OBJECT_CREATED, destination)

    def __conf_dt_interface_func(
            self, vpc, dt_ingress_queue,
            load_balancer, s3_bucket, findings_queue) -> None:

        """Create the Lambda Function responsible for
        extracting results from DT given an SBOM."""

        dt_func_sg = ec2.SecurityGroup(self, "LaunchTemplateSG", vpc=vpc)
        load_balancer.load_balancer_security_groups.append(dt_func_sg)

        fq_dn = load_balancer.load_balancer_dns_name

        dt_interface_function = lambda_.Function(
            self,
            DT_INTERFACE_LN,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.dt_interface_handler",
            code=code,
            environment={
                FINDINGS_QUEUE_URL_EV: findings_queue.queue_url,
                DT_API_BASE: fq_dn,
            },
            timeout=Duration.minutes(1),
            security_groups=[dt_func_sg],
            memory_size=512,

        )

        # Grant rights to send messages to the Queue
        findings_queue.grant_send_messages(dt_interface_function)

        s3_bucket.grant_put(dt_interface_function)
        s3_bucket.grant_read_write(dt_interface_function)

        root_pwd_param = ssm.StringParameter(
            self, DT_ROOT_PWD, string_value=EMPTY_VALUE, parameter_name=DT_ROOT_PWD
        )

        root_pwd_param.grant_read(dt_interface_function)
        root_pwd_param.grant_write(dt_interface_function)

        api_key_param = ssm.StringParameter(
            self, DT_API_KEY, string_value=EMPTY_VALUE, parameter_name=DT_API_KEY
        )

        api_key_param.grant_read(dt_interface_function)
        api_key_param.grant_write(dt_interface_function)

        event_source = SqsEventSource(dt_ingress_queue)
        dt_interface_function.add_event_source(event_source)

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        # Establish a VPC for the lambda to make calls to the services
        vpc = self.__create_vpc(
            ec2.GatewayVpcEndpointOptions(
                service=ec2.GatewayVpcEndpointAwsService.S3
            )
        )

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket(self, BUCKET_NAME)

        dt_ingress_queue = sqs.Queue(
            self,
            DT_SBOM_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5)
        )

        findings_queue = sqs.Queue(
            self,
            FINDINGS_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5)
        )

        lb, listener = self.__create_load_balancer(vpc)
        self.__create_dt_fargate_svc(vpc, listener)
        self.__conf_pristine_sbom_ingest_func(vpc, bucket)
        self.__conf_enrichment_ingress_func(vpc, bucket, dt_ingress_queue)
        self.__conf_dt_interface_func(vpc, dt_ingress_queue, lb, bucket, findings_queue)


def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    env = cdk.Environment(
        region=getenv("AWS_REGION"),
        account=getenv("AWS_ACCOUNT_NUM"),
    )

    app = cdk.App()
    SBOMApiStack(app, STACK_ID, env=env)
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    system("cdk deploy")

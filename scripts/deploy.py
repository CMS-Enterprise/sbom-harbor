""" This module is the start of the deployment for SBOM-API """

from os import system, getenv

import aws_cdk as cdk
import aws_cdk.aws_apigateway as apigw
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_ecs as ecs
import aws_cdk.aws_efs as efs
import aws_cdk.aws_lambda_python_alpha as lambda_
import aws_cdk.aws_s3 as s3
from aws_cdk import Duration
from aws_cdk import Stack
from aws_cdk.aws_lambda import Runtime
from constructs import Construct

BUCKET_NAME = "SBOMBucket"
CIDR = "10.0.0.0/16"
EC2_INSTANCE_NAME = "DependencyTrack"
EC2_SSH_KEY_NAME = "aquia"
EC2_INSTANCE_AMI = "ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20220131"
EC2_INSTANCE_TYPE = "t2.medium"
STACK_ID = "SBOMApiDeploy"
LAMBDA_NAME = "SBOMIngest"
PRIVATE = ec2.SubnetType.PRIVATE_WITH_NAT
PUBLIC = ec2.SubnetType.PUBLIC
REST_API_NAME = "SBOMApi"
PRIVATE_SUBNET_NAME = "SBOMPrivateSubnet"
PUBLIC_SUBNET_NAME = "SBOMPublicSubnet"
VPC_NAME = "SBOMVpc"


class SBOMApiDeploy(Stack):

    """
    This class is where the infrastructure to run the application
    is built.  This class inherits from the Stack class, which is part of
    the AWS CDK.
    """

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        # Create an endpoint gateway for S3 so we can reach it from the Lambda
        s3_gateway_endpoint = create_gateway_endpoint()

        # Establish a VPC for the lambda to make calls to the services
        vpc = create_vpc(self, s3_gateway_endpoint)

        # create an ecs cluster for running dependencytrack
        fargate_cluster = ecs.Cluster(self, "FargateCluster", vpc=vpc)

        # create an efs mount for maintaining
        dt_mount = efs.FileSystem(self, "dtApiStorage", vpc=vpc, encrypted=True)

        dt_volume = ecs.Volume(
            name="dtApiStorage",
            efs_volume_configuration=ecs.EfsVolumeConfiguration(
                file_system_id=dt_mount.file_system_id
            ),
        )

        dt_volume_mount = ecs.MountPoint(
            container_path="/apiserver", source_volume=dt_volume.name, read_only=False
        )

        dt_api_task_definition = ecs.TaskDefinition(
            self,
            "dtTaskDefinition",
            compatibility=ecs.Compatibility.FARGATE,
            cpu="4096",
            memory_mib="8192",
            volumes=[dt_volume],
        )

        container = dt_api_task_definition.add_container(
            "dtContainer",
            image=ecs.ContainerImage.from_registry("dependencytrack/apiserver"),
            logging=ecs.LogDrivers.aws_logs(stream_prefix="dependencyTrackApi"),
            environment={},
            cpu=2048,
            memory_reservation_mib=4096,
        )

        port_mapping = ecs.PortMapping(
            container_port=8080, host_port=8080, protocol=ecs.Protocol.TCP
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

        # Create the EC2 Instance that is going to run dependency track
        # and set permissions on the S3 Bucket
        # dependency_track = create_ec2_instance(self, vpc, security_group)
        # dependency_track.connections.allow_from_any_ipv4(
        #    ec2.Port.tcp(22),
        #    'Allow inbound SSH from anywhere')
        # bucket.grant_put(dependency_track)
        # bucket.grant_read(dependency_track)
        # bucket.grant_delete(dependency_track)

        # Create the Lambda Function to do the work
        # and set permissions on the S3 Bucket
        sbom_ingest_func = lambda_.PythonFunction(
            self,
            LAMBDA_NAME,
            runtime=Runtime.PYTHON_3_8,
            handler="cyclonedx.api.store_handler",
            entry="./cyclonedx",
            index="api.py",
            environment={"SBOM_BUCKET_NAME": bucket.bucket_name},
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        bucket.grant_put(sbom_ingest_func)

        lambda_api = apigw.LambdaRestApi(
            self, "sbom_ingest_api", handler=sbom_ingest_func
        )

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")


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

""" This module is the start of the deployment for SBOM-API """

from os import system, getenv, path

import aws_cdk as cdk
import aws_cdk.aws_ssm as ssm
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_s3_notifications as s3n
import aws_cdk.aws_sqs as sqs
import aws_cdk.aws_elasticloadbalancingv2 as elbv2

from aws_cdk import Duration
from aws_cdk import Stack
from aws_cdk.aws_lambda_event_sources import SqsEventSource
from constructs import Construct

from cyclonedx.constants import (
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
from scripts.DependencyTrackLoadBalancer import DependencyTrackLoadBalancer

from scripts.constants import (
    FINDINGS_QUEUE_NAME,
    PUBLIC,
    PRIVATE,
    STACK_ID,
    BUCKET_NAME,
    SBOM_ENRICHMENT_LN,
    PRIVATE_SUBNET_NAME,
    PUBLIC_SUBNET_NAME,
    DT_INTERFACE_LN,
    VPC_NAME,
    CIDR,
    DT_SBOM_QUEUE_NAME,
)

from scripts.PristineSbomIngressLambda import PristineSbomIngressLambda
from scripts.DependencyTrackFargateInstance import DependencyTrackFargateInstance

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
        bucket = s3.Bucket(
            self,
            BUCKET_NAME,
            removal_policy=cdk.RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )

        dt_ingress_queue = sqs.Queue(
            self,
            DT_SBOM_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5),
        )

        findings_queue = sqs.Queue(
            self,
            FINDINGS_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5),
        )

        dt_lb = DependencyTrackLoadBalancer(
            self,
            vpc=vpc,
        )

        lb_tl = dt_lb.get_lb_target_listener()
        lb_construct = dt_lb.get_lb_construct()

        DependencyTrackFargateInstance(
            self,
            vpc=vpc,
            ecs_target_listener=lb_tl,
        )

        PristineSbomIngressLambda(
            self,
            vpc=vpc,
            code=code,
            s3_bucket=bucket,
        )
        self.__conf_enrichment_ingress_func(vpc, bucket, dt_ingress_queue)
        self.__conf_dt_interface_func(vpc, dt_ingress_queue, lb_construct, bucket, findings_queue)


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

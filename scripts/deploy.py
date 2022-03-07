import aws_cdk as cdk
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_apigateway as apigw
from aws_cdk import Duration
from aws_cdk import Stack
from os import path, system, getenv
from constructs import Construct

BUCKET_NAME = "SBOMBucket"
CIDR = '10.0.0.0/16'
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

        # create a new security group
        security_group = create_security_group(self, vpc)

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket(self, BUCKET_NAME)

        # Create the EC2 Instance that is going to run dependency track
        # and set permissions on the S3 Bucket
        dependency_track = create_ec2_instance(self, vpc, security_group)
        dependency_track.connections.allow_from_any_ipv4(
            ec2.Port.tcp(22),
            'Allow inbound SSH from anywhere')
        bucket.grant_put(dependency_track)
        bucket.grant_read(dependency_track)
        bucket.grant_delete(dependency_track)

        # Create the Lambda Function to do the work
        # and set permissions on the S3 Bucket
        sbom_ingest_func = create_ingest_lambda(self, vpc, bucket)
        bucket.grant_put(sbom_ingest_func)

        # Create an API Gateway with CORS applied so we can hit the
        # endpoint from the command line
        lambda_api = create_api_gateway(self, sbom_ingest_func)

        # Create a POST Endpoint called 'store' to hit.
        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")


def create_gateway_endpoint() -> ec2.GatewayVpcEndpointOptions:

    return ec2.GatewayVpcEndpointOptions(
        service=ec2.GatewayVpcEndpointAwsService.S3
    )


def create_vpc(
        stack: Stack,
        gw_ept: ec2.GatewayVpcEndpointOptions) -> ec2.Vpc:

    private_subnet = ec2.SubnetConfiguration(
        name=PRIVATE_SUBNET_NAME,
        subnet_type=PRIVATE,
        cidr_mask=26
    )

    public_subnet = ec2.SubnetConfiguration(
        name=PUBLIC_SUBNET_NAME,
        subnet_type=PUBLIC,
        cidr_mask=26
    )

    return ec2.Vpc(
            stack, VPC_NAME,
            cidr=CIDR,
            max_azs=2,
            enable_dns_hostnames=True,
            enable_dns_support=True,
            gateway_endpoints={
                "S3": gw_ept
            },
            subnet_configuration=[
                private_subnet,
                public_subnet
            ]
        )


def create_security_group(
        stack: Stack,
        vpc: ec2.Vpc) -> ec2.SecurityGroup:

    return ec2.SecurityGroup(
        stack,
        "Allow_8080",
        allow_all_outbound=True,
        vpc=vpc
    )


def create_ec2_instance(
        stack: Stack, vpc: ec2.Vpc,
        sg: ec2.SecurityGroup) -> ec2.Instance:

    return ec2.Instance(
        stack, "Instance",
        key_name=EC2_SSH_KEY_NAME,
        instance_name=EC2_INSTANCE_NAME,
        instance_type=ec2.InstanceType(EC2_INSTANCE_TYPE),
        machine_image=ec2.MachineImage().lookup(name=EC2_INSTANCE_AMI),
        security_group=sg,
        vpc=vpc
    )


def create_ingest_lambda(
        stack: Stack,
        vpc: ec2.Vpc,
        bucket: s3.Bucket) -> lambda_.Function:

    # Get the Current Working Directory so we can construct a path to the
    # zip file for the Lambda
    cwd = path.dirname(__file__)
    code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)

    return lambda_.Function(
            stack,
            LAMBDA_NAME,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=PRIVATE
            ),
            handler="cyclonedx.api.store_handler",
            code=code,
            environment={"SBOM_BUCKET_NAME": bucket.bucket_name},
            timeout=Duration.seconds(10),
            memory_size=512,
        )


def create_api_gateway(
        stack: Stack,
        sbom_ingest_func: lambda_.Function) -> apigw.LambdaRestApi:

    return apigw.LambdaRestApi(
            stack,
            REST_API_NAME,
            default_cors_preflight_options=apigw.CorsOptions(
                allow_origins=apigw.Cors.ALL_ORIGINS,
                allow_methods=apigw.Cors.ALL_METHODS,
            ),
            handler=sbom_ingest_func,
            proxy=False,
        )


def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    env = cdk.Environment(
        region=getenv("AWS_REGION"),
        account=getenv("AWS_ACCOUNT_NUM")
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

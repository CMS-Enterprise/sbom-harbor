from aws_cdk import (
    aws_ec2 as ec2,
    RemovalPolicy,
)
from constructs import Construct

from deploy.constants import (
    CIDR,
    PRIVATE_SUBNET_NAME,
    PRIVATE,
    PUBLIC_SUBNET_NAME,
    PUBLIC, VPC_ID, VPC_NAME,
)


class SBOMApiVpc(Construct):
    
    """This is the VPC used throughout the application.
    One single VPC for the app."""

    def __init__(
        self,
        scope: Construct,
    ):

        """Creates a VPC for SBOM ingest and enrichment"""

        super().__init__(scope, VPC_NAME)

        private_subnet = ec2.SubnetConfiguration(
            name=PRIVATE_SUBNET_NAME,
            subnet_type=PRIVATE,
            cidr_mask=26,
        )

        # TODO: release elastic IP addresses on teardown
        # see: https://us-east-1.console.aws.amazon.com/ec2/v2/home?region=us-east-1#Addresses:
        public_subnet = ec2.SubnetConfiguration(
            name=PUBLIC_SUBNET_NAME,
            subnet_type=PUBLIC,
            cidr_mask=26,
        )

        self.vpc = ec2.Vpc(
            self,
            id=VPC_ID,
            vpc_name=VPC_NAME,
            cidr=CIDR,
            max_azs=2,
            enable_dns_support=True,
            enable_dns_hostnames=True,
            subnet_configuration=[private_subnet, public_subnet],
            gateway_endpoints={
                "S3": ec2.GatewayVpcEndpointOptions(
                    service=ec2.GatewayVpcEndpointAwsService.S3
                )
            },
        )

        self.vpc.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_vpc(self) -> ec2.Vpc:

        """Returns the underlying VPC to plug into other constructs."""

        return self.vpc

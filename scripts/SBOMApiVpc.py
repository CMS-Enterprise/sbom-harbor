import aws_cdk.aws_ec2 as ec2
from constructs import Construct

from scripts.constants import CIDR, PRIVATE, PRIVATE_SUBNET_NAME, PUBLIC, PUBLIC_SUBNET_NAME, VPC_NAME


class SBOMApiVpc(Construct):

    def __init__(self,  scope: Construct):

        super().__init__(scope, VPC_NAME)

        """Creates a VPC"""

        private_subnet = ec2.SubnetConfiguration(
            name=PRIVATE_SUBNET_NAME,
            subnet_type=PRIVATE,
            cidr_mask=26,
        )

        public_subnet = ec2.SubnetConfiguration(
            name=PUBLIC_SUBNET_NAME,
            subnet_type=PUBLIC,
            cidr_mask=26,
        )

        self.vpc = ec2.Vpc(
            self,
            VPC_NAME,
            cidr=CIDR,
            max_azs=2,
            enable_dns_support=True,
            enable_dns_hostnames=True,
            subnet_configuration=[
                private_subnet,
                public_subnet
            ],
            gateway_endpoints={
                "S3": ec2.GatewayVpcEndpointOptions(
                    service=ec2.GatewayVpcEndpointAwsService.S3
                )
            },
        )

    def get_vpc(self):
        return self.vpc

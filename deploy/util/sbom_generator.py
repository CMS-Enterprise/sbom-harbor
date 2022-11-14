"""
-> Module to house SbomGeneratorLambda
"""
from aws_cdk import Duration, Size
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from deploy.constants import (
    CF_DOMAIN,
    CF_DOMAIN_KEY,
    GH_FETCH_TOKEN,
    GH_FETCH_TOKEN_KEY,
    HARBOR_PASSWORD,
    HARBOR_PASSWORD_KEY,
    HARBOR_USERNAME,
    HARBOR_USERNAME_KEY,
    PRIVATE,
    SBOM_GENERATOR_LN,
)


class SBOMGeneratorLambda(Construct):
    """
    -> Constructs a Lambda to generate SBOMs for either
    -> all public repositories for an organization or
    -> a specific repo for an organization.
    """

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
    ):
        super().__init__(scope, SBOM_GENERATOR_LN)

        self.func = lambda_.Function(
            self,
            SBOM_GENERATOR_LN,
            function_name="SBOMGeneratorLambda",
            runtime=lambda_.Runtime.FROM_IMAGE,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler=lambda_.Handler.FROM_IMAGE,
            code=lambda_.Code.from_asset_image(
                directory="./harbor-sbom-generator",
                file="Dockerfile",
            ),
            environment={
                GH_FETCH_TOKEN_KEY: GH_FETCH_TOKEN,
                CF_DOMAIN_KEY: CF_DOMAIN,
                HARBOR_USERNAME_KEY: HARBOR_USERNAME,
                HARBOR_PASSWORD_KEY: HARBOR_PASSWORD,
            },
            timeout=Duration.seconds(900),
            memory_size=512,
            ephemeral_storage_size=Size.mebibytes(amount=10000),
        )

    def get_lambda_function(self):
        """
        -> Returns the Lambda CDK Type
        """

        return self.func

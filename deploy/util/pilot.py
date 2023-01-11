"""
-> Module to house PilotLambda
"""
from aws_cdk import Size
from aws_cdk import aws_iam as iam
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from deploy.constants import PILOT_LN, STANDARD_LAMBDA_TIMEOUT


class PilotLambda(Construct):
    """Constructs a Lambda that receives Pilot requests from external CI or the importer"""

    def __init__(
        self,
        scope: Construct,
    ):
        super().__init__(scope, PILOT_LN)

        self.pilot_func = lambda_.Function(
            self,
            PILOT_LN,
            function_name=PILOT_LN,
            runtime=lambda_.Runtime.FROM_IMAGE,
            handler=lambda_.Handler.FROM_IMAGE,
            code=lambda_.Code.from_asset_image(
                directory="./harbor-rs",
                file="Dockerfile.pilot",
            ),
            timeout=STANDARD_LAMBDA_TIMEOUT,
            memory_size=512,
            ephemeral_storage_size=Size.mebibytes(amount=5000),
        )

        self.pilot_func.grant_invoke(
            iam.ServicePrincipal("apigateway.amazonaws.com"),
        )

    def get_lambda_function(self):
        """
        -> Get the CDK Lambda Construct
        """

        return self.pilot_func

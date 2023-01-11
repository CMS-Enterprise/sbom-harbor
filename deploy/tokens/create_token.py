"""
-> Constructs SBOMCreateToken Lambda
"""
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from deploy.constants import (
    CREATE_TOKEN_LN,
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
    STANDARD_LAMBDA_TIMEOUT,
)
from deploy.util import DynamoTableManager, create_asset


class SBOMCreateTokenLambda(Construct):

    """Lambda to create an API token"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
    ):

        super().__init__(scope, CREATE_TOKEN_LN)

        self.func = lambda_.Function(
            self,
            CREATE_TOKEN_LN,
            function_name=CREATE_TOKEN_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.tokens.create_token_handler",
            code=create_asset(self),
            timeout=STANDARD_LAMBDA_TIMEOUT,
            memory_size=512,
        )

        table_mgr.grant(self.func)

    def get_lambda_function(self):
        """
        -> Get the actual Lambda Construct
        """
        return self.func

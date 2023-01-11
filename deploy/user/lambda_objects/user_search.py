"""
-> Module to house the User Search Lambda Construct
"""
from aws_cdk import aws_cognito as cognito
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from aws_cdk.aws_iam import Effect, PolicyStatement
from constructs import Construct

from cyclonedx.constants import USER_POOL_CLIENT_ID_KEY, USER_POOL_ID_KEY
from deploy.constants import (
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
    STANDARD_LAMBDA_TIMEOUT,
    USER_SEARCH_LN,
)
from deploy.util import create_asset


class SBOMUserSearchLambda(Construct):

    """Lambda to search for users in Cognito"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        user_pool_client: cognito.UserPoolClient,
        user_pool: cognito.UserPool,
    ):

        super().__init__(scope, USER_SEARCH_LN)

        self.func = lambda_.Function(
            self,
            USER_SEARCH_LN,
            function_name=USER_SEARCH_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.handlers.user_search_handler",
            code=create_asset(self),
            timeout=STANDARD_LAMBDA_TIMEOUT,
            memory_size=512,
            environment={
                USER_POOL_ID_KEY: user_pool.user_pool_id,
                USER_POOL_CLIENT_ID_KEY: user_pool_client.user_pool_client_id,
            },
        )

        self.func.add_to_role_policy(
            PolicyStatement(
                effect=Effect.ALLOW,
                actions=[
                    "cognito-idp:ListUsers",
                ],
                resources=["*"],
            )
        )

    def get_lambda_function(self):

        """
        -> Get the actual Lambda Construct
        """

        return self.func

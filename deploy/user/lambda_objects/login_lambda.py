"""
-> Module to house the SBOMLoginLambda Construct
"""
from aws_cdk import Duration
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_iam as iam
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from deploy.constants import (
    AWS_ACCOUNT_ID,
    ENVIRONMENT,
    LOGIN_LN,
    SBOM_API_PYTHON_RUNTIME,
    USER_POOL_CLIENT_ID_KEY,
    USER_POOL_ID_KEY,
)
from deploy.util import create_asset


class SBOMLoginLambda(Construct):

    """Lambda to manage logging in"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        user_pool_id: str,
        user_pool_client_id: str,
    ):

        super().__init__(scope, LOGIN_LN)

        self.login_func = lambda_.Function(
            self,
            LOGIN_LN,
            function_name=LOGIN_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
            ),
            handler="cyclonedx.handlers.login_handler",
            code=create_asset(self),
            timeout=Duration.seconds(10),
            memory_size=512,
            environment={
                USER_POOL_ID_KEY: user_pool_id,
                USER_POOL_CLIENT_ID_KEY: user_pool_client_id,
                "CDK_DEFAULT_ACCOUNT": AWS_ACCOUNT_ID,
                "ENVIRONMENT": ENVIRONMENT,
            },
        )

        self.login_func.add_to_role_policy(
            iam.PolicyStatement(
                effect=iam.Effect.ALLOW,
                actions=[
                    "cognito-idp:AdminGetUser",
                    "cognito-idp:AdminEnableUser",
                    "cognito-idp:AdminDisableUser",
                    "cognito-idp:AdminInitiateAuth",
                ],
                resources=["*"],
            )
        )

    def get_lambda_function(self):

        """
        -> Get the actual Lambda Construct
        """

        return self.login_func

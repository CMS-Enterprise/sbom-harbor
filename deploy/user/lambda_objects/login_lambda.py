from aws_cdk import (
    aws_ec2 as ec2,
    aws_iam as iam,
    aws_lambda as lambda_,
    Duration,
)
from constructs import Construct

from cyclonedx.constants import (
    USER_POOL_CLIENT_ID_KEY,
    USER_POOL_NAME_KEY,
)
from deploy.constants import (
    LOGIN_LN,
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.util import create_asset


class SBOMLoginLambda(Construct):

    """ Lambda to manage logging in """

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
            function_name="SBOMLoginLambda",
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.user.login_handler",
            code=create_asset(self),
            timeout=Duration.seconds(10),
            memory_size=512,
            environment={
                USER_POOL_NAME_KEY: user_pool_id,
                USER_POOL_CLIENT_ID_KEY: user_pool_client_id
            }
        )

        self.login_func.add_to_role_policy(iam.PolicyStatement(
            effect=iam.Effect.ALLOW,
            actions=[
                'cognito-idp:AdminGetUser',
                'cognito-idp:AdminEnableUser',
                'cognito-idp:AdminDisableUser',
                'cognito-idp:AdminInitiateAuth',
            ],
            resources=[
                f"*"
            ]
        ))

    def get_lambda_function(self):
        return self.login_func

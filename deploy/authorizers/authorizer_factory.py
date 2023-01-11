"""Factory for generating Authorizer Lambdas"""

from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from aws_cdk.aws_iam import Effect, PolicyStatement
from constructs import Construct

from cyclonedx.constants import USER_POOL_CLIENT_ID_KEY, USER_POOL_ID_KEY
from deploy.constants import PRIVATE, SBOM_API_PYTHON_RUNTIME, STANDARD_LAMBDA_TIMEOUT
from deploy.util import create_asset


class AuthorizerLambdaFactory:

    """Factory to generate AuthorizerLambdas"""

    class SBOMJwtAuthorizerLambda(Construct):

        """Lambda to check DynamoDB for a token belonging to the team sending an SBOM"""

        # pylint: disable = R0913
        def __init__(
            self,
            scope: Construct,
            vpc: ec2.Vpc,
            name: str,
            user_pool_id: str,
            user_pool_client_id: str,
        ):

            super().__init__(scope, name)

            self.lambda_func = lambda_.Function(
                self,
                name,
                function_name=name,
                runtime=SBOM_API_PYTHON_RUNTIME,
                vpc=vpc,
                vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
                handler="cyclonedx.handlers.jwt_authorizer_handler",
                code=create_asset(self),
                timeout=STANDARD_LAMBDA_TIMEOUT,
                memory_size=512,
                environment={
                    USER_POOL_ID_KEY: user_pool_id,
                    USER_POOL_CLIENT_ID_KEY: user_pool_client_id,
                },
            )

            self.lambda_func.add_to_role_policy(
                PolicyStatement(
                    effect=Effect.ALLOW,
                    actions=[
                        "cognito-idp:AdminDisableUser",
                        "cognito-idp:AdminEnableUser",
                        "cognito-idp:AdminGetUser",
                        "cognito-idp:ListUsers",
                    ],
                    resources=["*"],
                )
            )

        def get_lambda_function(self):

            """
            -> Get the CDK Lambda Construct
            """

            return self.lambda_func

    def __init__(
        self: "AuthorizerLambdaFactory",
        scope: Construct,
        vpc: ec2.Vpc,
        user_pool_id: str,
        user_pool_client_id: str,
    ):

        """Constructor"""

        self.scope = scope
        self.vpc = vpc
        self.user_pool_id = user_pool_id
        self.user_pool_client_id = user_pool_client_id

    def create(self, lambda_name: str):

        """Create an AuthorizerLambda with the specified name"""

        return AuthorizerLambdaFactory.SBOMJwtAuthorizerLambda(
            self.scope,
            vpc=self.vpc,
            name=lambda_name,
            user_pool_id=self.user_pool_id,
            user_pool_client_id=self.user_pool_client_id,
        )

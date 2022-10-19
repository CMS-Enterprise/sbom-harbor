"""Factory for generating Authorizer Lambdas"""

from aws_cdk import (
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    Duration,
)
from aws_cdk.aws_iam import (
    PolicyStatement,
    Effect,
)
from constructs import Construct

from deploy.constants import (
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.util import create_asset


class AuthorizerLambdaFactory:

    """Factory to generate AuthorizerLambdas"""

    class SBOMJwtAuthorizerLambda(Construct):

        """Lambda to check DynamoDB for a token belonging to the team sending an SBOM"""

        def __init__(
            self,
            scope: Construct,
            *,
            vpc: ec2.Vpc,
            name: str,
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
                timeout=Duration.seconds(10),
                memory_size=512,
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

            """Get the CDK Lambda Construct"""

            return self.lambda_func

    def __init__(
        self: "AuthorizerLambdaFactory",
        scope: Construct,
        vpc: ec2.Vpc,
    ):

        """Constructor"""

        self.scope = scope
        self.vpc = vpc

    def create(self, lambda_name: str):

        """Create an AuthorizerLambda with the specified name"""

        return AuthorizerLambdaFactory.SBOMJwtAuthorizerLambda(
            self.scope,
            vpc=self.vpc,
            name=f"{lambda_name}_Authorizer",
        )

from aws_cdk import (
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    Duration,
)
from constructs import Construct

from deploy.constants import (
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.util import create_asset


class AuthorizerLambdaFactory(object):

    class SBOMJwtAuthorizerLambda(Construct):

        """ Lambda to check DynamoDB for a token belonging to the team sending an SBOM """

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
                handler="cyclonedx.teams.jwt_authorizer_handler",
                code=create_asset(self),
                timeout=Duration.seconds(10),
                memory_size=512,
            )

        def get_lambda_function(self):
            return self.lambda_func

    def __init__(self, scope: Construct, vpc: ec2.Vpc):
        self.scope = scope
        self.vpc = vpc

    def create(self, lambda_name: str):
        return AuthorizerLambdaFactory.SBOMJwtAuthorizerLambda(
            self.scope, vpc=self.vpc, name=f"{lambda_name}_Authorizer")

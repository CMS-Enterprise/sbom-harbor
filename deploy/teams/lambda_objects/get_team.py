from aws_cdk import (
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    Duration,
)
from constructs import Construct

from deploy.constants import (
    GET_TEAM_LN,
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.util import create_asset
from deploy.util import DynamoTableManager


class SBOMGetTeamLambda(Construct):

    """ Lambda to get a team """

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
    ):

        super().__init__(scope, GET_TEAM_LN)

        self.func = lambda_.Function(
            self, GET_TEAM_LN,
            function_name="SBOMGetTeamLambda",
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.teams.get_team_handler",
            code=create_asset(self),
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        table_mgr.grant(self.func)

    def get_lambda_function(self):
        return self.func


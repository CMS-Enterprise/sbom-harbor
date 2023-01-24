from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from deploy.constants import (
    SBOM_API_PYTHON_RUNTIME,
    STANDARD_LAMBDA_TIMEOUT,
    UPDATE_TEAM_LN,
)
from deploy.util import DynamoTableManager, create_asset


class SBOMUpdateTeamLambda(Construct):

    """Lambda to update a team"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
    ):

        super().__init__(scope, UPDATE_TEAM_LN)

        self.func = lambda_.Function(
            self,
            UPDATE_TEAM_LN,
            function_name=UPDATE_TEAM_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
            ),
            handler="cyclonedx.teams.update_team_handler",
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

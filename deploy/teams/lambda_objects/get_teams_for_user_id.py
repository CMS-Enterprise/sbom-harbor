from aws_cdk import Duration
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from deploy.constants import (
    AWS_ACCOUNT_ID,
    ENVIRONMENT,
    GET_TEAMS_FOR_ID_LN,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.util import DynamoTableManager, create_asset


class SBOMGetTeamsForUserIdLambda(Construct):

    """Lambda to get a team"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
    ):

        super().__init__(scope, GET_TEAMS_FOR_ID_LN)

        self.func = lambda_.Function(
            self,
            GET_TEAMS_FOR_ID_LN,
            function_name=GET_TEAMS_FOR_ID_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
            ),
            handler="cyclonedx.teams.get_teams_for_id_handler",
            code=create_asset(self),
            timeout=Duration.seconds(10),
            memory_size=512,
            environment={
                "CDK_DEFAULT_ACCOUNT": AWS_ACCOUNT_ID,
                "ENVIRONMENT": ENVIRONMENT,
            },
        )

        table_mgr.grant(self.func)

    def get_lambda_function(self):
        return self.func
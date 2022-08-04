from constructs import Construct
from aws_cdk import (
    RemovalPolicy,
    aws_dynamodb as dynamodb,
    aws_applicationautoscaling as autoscale,
)

from cyclonedx.constants import (
    TEAM_TABLE_ID,
    TEAM_TABLE_NAME,
)


class SBOMTeamTable(Construct):
    """
    This class is used to create the IAM role for the user pool.
    params:
        scope: Construct
        user_pool: SBOMUserPool
    """

    def __init__(
        self,
        scope: Construct,
    ):
        super().__init__(scope, TEAM_TABLE_ID)

        self.construct = dynamodb.Table(
            self, TEAM_TABLE_ID,
            table_name=TEAM_TABLE_NAME,
            billing_mode=dynamodb.BillingMode.PROVISIONED,
            removal_policy=RemovalPolicy.DESTROY,
            partition_key=dynamodb.Attribute(
                name="Id",
                type=dynamodb.AttributeType.STRING,
            ),
        )

        # Set up scaling
        read_scaling = self.construct.auto_scale_read_capacity(
            min_capacity=1,
            max_capacity=50,
        )

        read_scaling.scale_on_utilization(
            target_utilization_percent=50
        )

        read_scaling.scale_on_schedule(
            "ScaleUpInTheMorning",
            schedule=autoscale.Schedule.cron(
                hour="8",
                minute="0",
            ),
            min_capacity=20
        )

        read_scaling.scale_on_schedule(
            "ScaleDownAtNight",
            schedule=autoscale.Schedule.cron(
                hour="20",
                minute="0",
            ),
            max_capacity=20
        )

    def get_construct(self) -> dynamodb.Table:

        """ Return the underlying CDK Defined L3 Construct """

        return self.construct

"""This Stack is used to set up shared resources
that the other stacks use when deploying the application"""
import aws_cdk
from constructs import Construct
from aws_cdk import (
    aws_s3 as s3,
    aws_dynamodb as dynamodb,
    aws_applicationautoscaling as autoscale,
    RemovalPolicy,
    Stack,
)
from scripts.constants import (
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    SHARED_RESOURCE_STACK_ID,
    TEAM_TABLE_ID,
    TEAM_TABLE_NAME,
)
from scripts.constructs import (
    SBOMApiVpc
)


class SBOMSharedResourceStack(Stack):

    """This Stack is used to set up shared resources
    that the other stacks use when deploying the application"""

    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, SHARED_RESOURCE_STACK_ID, **kwargs)

        # Create the VPC
        vpc = SBOMApiVpc(self)
        self.vpc = vpc.get_vpc()

        # Create the S3 Bucket to put the BOMs in
        self.s3_bucket = s3.Bucket(
            self,
            S3_BUCKET_ID,
            bucket_name=S3_BUCKET_NAME,
            removal_policy=RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )

        table = dynamodb.Table(
            self, TEAM_TABLE_ID,
            table_name=TEAM_TABLE_NAME,
            billing_mode=dynamodb.BillingMode.PROVISIONED,
            removal_policy=aws_cdk.RemovalPolicy.DESTROY,
            partition_key=dynamodb.Attribute(
                name="id",
                type=dynamodb.AttributeType.STRING,
            ),
        )

        # Set up scaling
        read_scaling = table.auto_scale_read_capacity(
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

    def get_vpc(self):

        """Gets the VPC"""

        return self.vpc

    def get_lb(self):

        """Gets the Application Load Balancer"""

        return self.lb

    def get_s3_bucket(self):

        """Gets the S3 Bucket"""

        return self.s3_bucket


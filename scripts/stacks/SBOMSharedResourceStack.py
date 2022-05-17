
"""This Stack is used to set up shared resources
that the other stacks use when deploying the application"""

from constructs import Construct
from aws_cdk import (
    aws_s3 as s3,
    aws_dynamodb as dynamodb,
    RemovalPolicy,
    Stack,
)
from scripts.constants import (
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    SHARED_RESOURCE_STACK_ID,
)
from scripts.constructs import (
    SBOMApiVpc,
    SBOMTeamTable
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

        self.team_table: dynamodb.Table = SBOMTeamTable(self).get_construct()

    def get_vpc(self):

        """Gets the VPC"""

        return self.vpc

    def get_lb(self):

        """Gets the Application Load Balancer"""

        return self.lb

    def get_s3_bucket(self):

        """Gets the S3 Bucket"""

        return self.s3_bucket

    def get_team_table(self) -> dynamodb.Table:

        """ Returns the DynamoDB Team Table construct """

        return self.team_table

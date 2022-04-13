"""This Stack is used to set up shared resources
that the other stacks use when deploying the application"""

import aws_cdk as cdk
import aws_cdk.aws_s3 as s3

from aws_cdk import Stack
from constructs import Construct

from scripts.constructs import SBOMApiVpc

from scripts.constants import (
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    SHARED_RESOURCE_STACK_ID,
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

        self.vpc = SBOMApiVpc(self).get_vpc()

        # Create the S3 Bucket to put the BOMs in
        self.s3_bucket = s3.Bucket(
            self,
            S3_BUCKET_ID,
            bucket_name=S3_BUCKET_NAME,
            removal_policy=cdk.RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )

    def get_vpc(self):

        """Gets the VPC"""

        return self.vpc

    def get_s3_bucket(self):

        """Gets the S3 Bucket"""

        return self.s3_bucket

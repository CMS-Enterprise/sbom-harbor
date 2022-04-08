"""This stack deploys the Ingress Pipeline"""

from os import path

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
from aws_cdk import Stack
from constructs import Construct

from scripts.constants import (
    ENRICHMENT_BUCKET_NAME,
    S3_BUCKET_NAME,
    VPC_ID,
    VPC_NAME,
)

from scripts.constructs import PristineSbomIngressLambda

cwd = path.dirname(__file__)
ingress_code = lambda_.AssetCode.from_asset("%s/../../dist/lambda.zip" % cwd)


class SBOMIngressPiplineStack(Stack):

    """This stack deploys the Ingress Pipeline"""

    def __init__(
        self,
        scope: Construct,
        construct_id: str,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        vpc = ec2.Vpc.from_lookup(self, id=VPC_ID, vpc_name=VPC_NAME)

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket.from_bucket_name(
            self,
            ENRICHMENT_BUCKET_NAME,
            bucket_name=S3_BUCKET_NAME,
        )

        PristineSbomIngressLambda(
            self,
            vpc=vpc,
            code=ingress_code,
            s3_bucket=bucket,
        )

"""This Stack deploys the Enrichment Pipeline"""

from os import path

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_sqs as sqs
from aws_cdk import Duration
from aws_cdk import Stack
from constructs import Construct

from scripts.constants import (
    DT_SBOM_QUEUE_NAME,
    ENRICHMENT_BUCKET_NAME,
    S3_BUCKET_NAME,
    VPC_ID,
    VPC_NAME,
)

from scripts.constructs import DependencyTrackFargateInstance
from scripts.constructs import DependencyTrackInterfaceLambda
from scripts.constructs import DependencyTrackLoadBalancer
from scripts.constructs import EnrichmentIngressLambda

cwd = path.dirname(__file__)
enrichment_code = lambda_.AssetCode.from_asset("%s/../../dist/lambda.zip" % cwd)


class SBOMEnrichmentPiplineStack(Stack):

    """This Stack deploys the Enrichment Pipeline"""

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        dt_ingress_queue = sqs.Queue(
            self,
            DT_SBOM_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5),
        )

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket.from_bucket_name(
            self,
            ENRICHMENT_BUCKET_NAME,
            bucket_name=S3_BUCKET_NAME,
        )

        vpc = ec2.Vpc.from_lookup(self, id=VPC_ID, vpc_name=VPC_NAME)

        dt_lb = DependencyTrackLoadBalancer(
            self,
            vpc=vpc,
        )

        EnrichmentIngressLambda(
            self,
            vpc=vpc,
            code=enrichment_code,
            s3_bucket=bucket,
            output_queue=dt_ingress_queue,
        )

        DependencyTrackInterfaceLambda(
            self,
            vpc=vpc,
            code=enrichment_code,
            s3_bucket=bucket,
            input_queue=dt_ingress_queue,
            load_balancer=dt_lb,
        )

        DependencyTrackFargateInstance(
            self,
            vpc=vpc,
            load_balancer=dt_lb,
        )

"""This Stack deploys the Enrichment Pipeline"""

from os import path
from aws_cdk import (
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    aws_s3 as s3,
    aws_sqs as sqs,
    Duration,
    Stack,
)
from constructs import Construct
from scripts.constants import (
    DT_SBOM_QUEUE_NAME,
    ENRICHMENT_STACK_ID,
    S3_BUCKET_NAME,
)
from scripts.constructs import (
    DependencyTrackFargateInstance,
    DependencyTrackInterfaceLambda,
    DependencyTrackLoadBalancer,
    EnrichmentIngressLambda,
)


class SBOMEnrichmentPiplineStack(Stack):

    """This Stack deploys the Enrichment Pipeline"""

    def __init__(self, scope: Construct, vpc: ec2.Vpc, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, ENRICHMENT_STACK_ID, **kwargs)

        s3_bucket = s3.Bucket.from_bucket_name(
            self, "THIS_CONSTRUCT_ID", S3_BUCKET_NAME
        )

        dt_ingress_queue = sqs.Queue(
            self,
            DT_SBOM_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5),
        )

        dt_lb = DependencyTrackLoadBalancer(
            self,
            vpc=vpc,
        )

        EnrichmentIngressLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
            output_queue=dt_ingress_queue,
        )

        DependencyTrackInterfaceLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
            input_queue=dt_ingress_queue,
            load_balancer=dt_lb,
        )

        DependencyTrackFargateInstance(
            self,
            vpc=vpc,
            load_balancer=dt_lb,
        )

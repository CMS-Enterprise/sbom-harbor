

from os import path

import aws_cdk as cdk
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_sqs as sqs
import aws_cdk.aws_ec2 as ec2

from aws_cdk import Duration
from aws_cdk import Stack
from constructs import Construct

from scripts.constructs import SBOMApiVpc
from scripts.constructs import PristineSbomIngressLambda
from scripts.constructs import DependencyTrackFargateInstance
from scripts.constructs import DependencyTrackLoadBalancer
from scripts.constructs import EnrichmentIngressLambda
from scripts.constructs import DependencyTrackInterfaceLambda

from scripts.constants import (
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    DT_SBOM_QUEUE_NAME,
    ENRICHMENT_BUCKET_NAME,
    VPC_ID,
    VPC_NAME,
)

# Get the Current Working Directory so we can construct a path to the
# zip file for the Lambdas
cwd = path.dirname(__file__)
ingress_code = lambda_.AssetCode.from_asset("%s/../../dist/lambda.zip" % cwd)
enrichment_code = lambda_.AssetCode.from_asset("%s/../../dist/lambda.zip" % cwd)


class SBOMSharedResourceStack(Stack):

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        SBOMApiVpc(self).get_vpc()

        # Create the S3 Bucket to put the BOMs in
        s3.Bucket(
            self,
            S3_BUCKET_ID,
            bucket_name=S3_BUCKET_NAME,
            removal_policy=cdk.RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )


class SBOMIngressPiplineStack(Stack):

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        vpc = ec2.Vpc.from_lookup(
            self,
            id=VPC_ID,
            vpc_name=VPC_NAME
        )

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


class SBOMEnrichmentPiplineStack(Stack):

    """This class is where the infrastructure to run the application
    is built.  This class inherits from the Stack class, which is part of
    the AWS CDK."""

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

        vpc = ec2.Vpc.from_lookup(
            self,
            id=VPC_ID,
            vpc_name=VPC_NAME
        )

        dt_lb = DependencyTrackLoadBalancer(
            self,
            vpc=vpc,
        )

        EnrichmentIngressLambda(
            self,
            vpc=vpc,
            code=enrichment_code,
            s3_bucket=bucket,
            output_queue=dt_ingress_queue
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

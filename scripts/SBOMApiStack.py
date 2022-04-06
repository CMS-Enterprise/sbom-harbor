
from os import path

import aws_cdk as cdk
import aws_cdk.aws_elasticloadbalancingv2 as elbv2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_sqs as sqs

from aws_cdk import Duration
from aws_cdk import Stack
from constructs import Construct

from scripts.DependencyTrackLoadBalancer import DependencyTrackLoadBalancer
from scripts.EnrichmentIngressLambda import EnrichmentIngressLambda
from scripts.DependencyTrackInterfaceLambda import DependencyTrackInterfaceLambda
from scripts.SBOMApiVpc import SBOMApiVpc

from scripts.constants import (
    FINDINGS_QUEUE_NAME,
    BUCKET_NAME,
    DT_SBOM_QUEUE_NAME,
)

from scripts.PristineSbomIngressLambda import PristineSbomIngressLambda
from scripts.DependencyTrackFargateInstance import DependencyTrackFargateInstance

# Get the Current Working Directory so we can construct a path to the
# zip file for the Lambdas
cwd = path.dirname(__file__)
code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)


class SBOMApiStack(Stack):

    """This class is where the infrastructure to run the application
    is built.  This class inherits from the Stack class, which is part of
    the AWS CDK."""

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket(
            self,
            BUCKET_NAME,
            removal_policy=cdk.RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )

        dt_ingress_queue = sqs.Queue(
            self,
            DT_SBOM_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5),
        )

        findings_queue = sqs.Queue(
            self,
            FINDINGS_QUEUE_NAME,
            fifo=True,
            content_based_deduplication=True,
            visibility_timeout=Duration.minutes(5),
        )

        vpc = SBOMApiVpc(self).get_vpc()

        dt_lb = DependencyTrackLoadBalancer(
            self,
            vpc=vpc,
        )

        lb_tl = dt_lb.get_lb_target_listener()
        load_balancer: elbv2.ApplicationLoadBalancer = dt_lb.get_load_balancer()

        DependencyTrackFargateInstance(
            self,
            vpc=vpc,
            ecs_target_listener=lb_tl,
        )

        PristineSbomIngressLambda(
            self,
            vpc=vpc,
            code=code,
            s3_bucket=bucket,
        )

        EnrichmentIngressLambda(
            self,
            vpc=vpc,
            code=code,
            s3_bucket=bucket,
            output_queue=dt_ingress_queue
        )

        DependencyTrackInterfaceLambda(
            self,
            vpc=vpc,
            code=code,
            s3_bucket=bucket,
            dt_ingress_queue=dt_ingress_queue,
            load_balancer=load_balancer,
            findings_queue=findings_queue
        )

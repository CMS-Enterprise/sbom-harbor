"""
-> Module to house EnrichmentIngressLambda
"""
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_events as eventbridge
from aws_cdk import aws_lambda as lambda_
from aws_cdk import aws_s3 as s3
from aws_cdk import aws_s3_notifications as s3n
from constructs import Construct

from cyclonedx.constants import AWS_ACCOUNT_ID, ENVIRONMENT
from deploy.constants import (
    SBOM_API_PYTHON_RUNTIME,
    SBOM_BUCKET_NAME_KEY,
    SBOM_ENRICHMENT_LN,
    STANDARD_LAMBDA_TIMEOUT,
)
from deploy.util import create_asset


class EnrichmentIngressLambda(Construct):

    """Create the Lambda Function responsible for listening on the S3 Bucket
    for SBOMs being inserted so they can be inserted into the enrichment process."""

    def __init__(
        self,
        scope: Construct,
        s3_bucket: s3.IBucket,
        *,
        vpc: ec2.Vpc,
        event_bus: eventbridge.EventBus,
    ):

        super().__init__(scope, SBOM_ENRICHMENT_LN)

        self.func = lambda_.Function(
            self,
            SBOM_ENRICHMENT_LN,
            function_name=SBOM_ENRICHMENT_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
            ),
            handler="cyclonedx.handlers.enrichment_ingress_handler",
            code=create_asset(self),
            environment={
                SBOM_BUCKET_NAME_KEY: s3_bucket.bucket_name,
                "CDK_DEFAULT_ACCOUNT": AWS_ACCOUNT_ID,
                "ENVIRONMENT": ENVIRONMENT,
            },
            timeout=STANDARD_LAMBDA_TIMEOUT,
            memory_size=512,
        )

        # Bucket rights granted
        s3_bucket.grant_read(self.func)

        # Write to EventBridge
        event_bus.grant_put_events_to(self.func)

        # Set up the S3 Bucket to send a notification to the Lambda
        # if someone puts something in the bucket.
        s3_bucket.add_event_notification(
            s3.EventType.OBJECT_CREATED,
            s3n.LambdaDestination(self.func),
            s3.NotificationKeyFilter(
                prefix="sbom",
            ),
        )

    def get_lambda_function(self):

        """
        -> Returns the actual Construct
        """

        return self.func

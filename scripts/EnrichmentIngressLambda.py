from aws_cdk import Duration
from aws_cdk.aws_lambda import AssetCode
from aws_cdk.aws_s3 import Bucket
from constructs import Construct
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_sqs as sqs
import aws_cdk.aws_s3_notifications as s3n
import aws_cdk.aws_s3 as s3

import aws_cdk.aws_lambda as lambda_
from cyclonedx.constants import (
    DT_QUEUE_URL_EV,
    SBOM_BUCKET_NAME_EV,
)
from scripts.constants import (
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
    SBOM_ENRICHMENT_LN,
)


class EnrichmentIngressLambda(Construct):

    """Create the Lambda Function responsible for listening on the S3 Bucket
        for SBOMs being inserted so they can be inserted into the enrichment process."""

    def __init__(self, scope: Construct, *, vpc: ec2.Vpc,
                 code: AssetCode, s3_bucket: Bucket,
                 output_queue: sqs.Queue,):

        super().__init__(scope, SBOM_ENRICHMENT_LN)

        sbom_enrichment_ingress_func = lambda_.Function(
            self,
            SBOM_ENRICHMENT_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.enrichment_ingress_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_EV: s3_bucket.bucket_name,
                DT_QUEUE_URL_EV: output_queue.queue_url,
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        # Bucket rights granted
        s3_bucket.grant_read(sbom_enrichment_ingress_func)

        # Grant rights to send messages to the Queue
        output_queue.grant_send_messages(sbom_enrichment_ingress_func)

        # Set up the S3 Bucket to send a notification to the Lambda
        # if someone puts something in the bucket. We really need to
        # think about how we should structure the file names to be
        # identifiable for our purposes #TODO
        destination = s3n.LambdaDestination(sbom_enrichment_ingress_func)
        s3_bucket.add_event_notification(s3.EventType.OBJECT_CREATED, destination)

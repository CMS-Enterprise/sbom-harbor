"""
-> Module to house SummarizerLambda
"""
from aws_cdk import Duration
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_events as eventbridge
from aws_cdk import aws_lambda as lambda_
from aws_cdk import aws_s3 as i_bucket
from constructs import Construct

from deploy.constants import PRIVATE, SBOM_API_PYTHON_RUNTIME, SUMMARIZER_LN
from deploy.util import create_asset


class SummarizerLambda(Construct):

    """
    -> This Construct creates a Lambda
    -> use to manage Dependency Track operations
    """

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        s3_bucket: i_bucket,
        event_bus: eventbridge.EventBus,
    ):
        super().__init__(scope, SUMMARIZER_LN)

        dt_func_sg = ec2.SecurityGroup(self, "LaunchTemplateSG", vpc=vpc)

        self.func = lambda_.Function(
            self,
            SUMMARIZER_LN,
            function_name=SUMMARIZER_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.handlers.summarizer_handler",
            code=create_asset(self),
            timeout=Duration.minutes(1),
            security_groups=[dt_func_sg],
            memory_size=512,
        )

        event_bus.grant_put_events_to(self.func)
        s3_bucket.grant_put(self.func)
        s3_bucket.grant_read_write(self.func)

    def get_lambda_function(self):

        """
        -> Get the CDK Lambda Construct
        """

        return self.func

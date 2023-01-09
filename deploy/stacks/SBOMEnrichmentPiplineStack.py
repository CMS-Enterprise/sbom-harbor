"""This Stack deploys the Enrichment Pipeline"""

import aws_cdk.aws_stepfunctions as stepfunctions
from aws_cdk import Stack
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_events as eventbridge
from aws_cdk import aws_events_targets as targets
from aws_cdk import aws_s3 as s3
from aws_cdk import aws_stepfunctions_tasks as tasks
from aws_cdk.aws_stepfunctions import Chain, Parallel
from constructs import Construct

from cyclonedx.constants import EVENT_BUS_SOURCE

from deploy.constants import ENRICHMENT_STACK_ID, S3_BUCKET_NAME, environize
from deploy.enrichment import EnrichmentIngressLambda
from deploy.enrichment.dependency_track import (
    DependencyTrackFargateInstance,
    DependencyTrackInterfaceLambda,
    DependencyTrackLoadBalancer,
    SummarizerLambda,
)
from deploy.enrichment.ion_channel import IonChannelInterfaceLambda
from deploy.util import DynamoTableManager


class SBOMEnrichmentPiplineStack(Stack):

    """This Stack deploys the Enrichment Pipeline"""

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
        event_bus: eventbridge.EventBus,
        **kwargs
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, ENRICHMENT_STACK_ID, **kwargs)

        s3_bucket = s3.Bucket.from_bucket_name(self, "DATA_LAKE", S3_BUCKET_NAME)

        dt_lb = DependencyTrackLoadBalancer(
            self,
            vpc=vpc,
        )

        DependencyTrackFargateInstance(
            self,
            vpc=vpc,
            load_balancer=dt_lb,
        )

        EnrichmentIngressLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
            event_bus=event_bus,
        )

        parallel = Parallel(self, "ENRICHMENT_JOBS")

        # Dependency Track Enrichment Source
        dt_lambda = DependencyTrackInterfaceLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
            event_bus=event_bus,
            load_balancer=dt_lb,
        ).get_lambda_function()

        dt_task = tasks.LambdaInvoke(
            self,
            "ENRICHMENT_DT_TASK",
            lambda_function=dt_lambda,
            input_path="$.detail",
            result_path="$.detail.results",
            output_path="$.detail",
        )

        parallel: Parallel = parallel.branch(dt_task)

        # Ion Channel Enrichment Source
        ic_lambda = IonChannelInterfaceLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
            event_bus=event_bus,
        ).get_lambda_function()

        ic_task = tasks.LambdaInvoke(
            self,
            "ENRICHMENT_IC_TASK",
            lambda_function=ic_lambda,
            input_path="$.detail",
            result_path="$.detail.results",
            output_path="$.detail",
        )

        # Uncommenting this will turn Ion Channel Enrichment on.
        parallel: Parallel = parallel.branch(ic_task)

        # Default Enrichment Source
        # default_enrichment_lambda = DefaultEnrichmentInterfaceLambda(
        #     self,
        #     vpc=vpc,
        #     s3_bucket=s3_bucket,
        #     event_bus=event_bus,
        # ).get_lambda_function()
        #
        # default_task = tasks.LambdaInvoke(
        #     self, "ENRICHMENT_DEFAULT_TASK",
        #     lambda_function=default_enrichment_lambda,
        #     input_path="$.detail",
        #     result_path="$.detail.results",
        #     output_path="$.detail",
        # )
        #
        # parallel: Parallel = parallel.branch(default_task)

        # Default Enrichment Source
        summarizer_lambda = SummarizerLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
            event_bus=event_bus,
        ).get_lambda_function()

        table_mgr.grant(summarizer_lambda)

        summarizer_task = tasks.LambdaInvoke(
            self,
            "ENRICHMENT_SUMMARIZER_TASK",
            lambda_function=summarizer_lambda,
        )

        chain: Chain = parallel.next(summarizer_task)

        # Create State Machine
        enrichment_machine = stepfunctions.StateMachine(
            self,
            "ENRICHMENT_STATE_MACHINE",
            state_machine_name=environize("Enrichment", delimiter="_"),
            state_machine_type=stepfunctions.StateMachineType.STANDARD,
            definition=chain,
        )

        eventbridge.Rule(
            self,
            "ENRICHMENT_EVENTBRIDGE_RULE",
            rule_name=environize("Enrichment", delimiter="_"),
            enabled=True,
            event_pattern=eventbridge.EventPattern(
                source=[EVENT_BUS_SOURCE],
            ),
            targets=[
                targets.SfnStateMachine(enrichment_machine),
            ],
            event_bus=event_bus,
        )

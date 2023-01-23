"""
-> Module to house the IonChannelInterfaceLambda Construct
"""
import os

from aws_cdk import Duration
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_events as eventbridge
from aws_cdk import aws_lambda as lambda_
from aws_cdk import aws_s3 as i_bucket
from aws_cdk import aws_ssm as ssm
from constructs import Construct

from cyclonedx.constants import AWS_ACCOUNT_ID, ENVIRONMENT
from deploy.constants import (
    IC_API_BASE,
    IC_API_KEY,
    IC_INTERFACE_LN,
    IC_RULESET_TEAM_ID,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.util import create_asset


class IonChannelInterfaceLambda(Construct):

    """This Construct creates a Lambda
    use to manage Dependency Track operations"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        s3_bucket: i_bucket,
        event_bus: eventbridge.EventBus,
    ):

        super().__init__(scope, IC_INTERFACE_LN)

        dt_func_sg = ec2.SecurityGroup(self, "LaunchTemplateSG", vpc=vpc)

        self.func = lambda_.Function(
            self,
            IC_INTERFACE_LN,
            function_name=IC_INTERFACE_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
            ),
            handler="cyclonedx.handlers.ic_interface_handler",
            environment={
                "CDK_DEFAULT_ACCOUNT": AWS_ACCOUNT_ID,
                "ENVIRONMENT": ENVIRONMENT,
            },
            code=create_asset(self),
            timeout=Duration.minutes(15),
            security_groups=[dt_func_sg],
            memory_size=512,
        )

        event_bus.grant_put_events_to(self.func)
        s3_bucket.grant_put(self.func)
        s3_bucket.grant_read_write(self.func)

        # Ion Channel JWT Needs to be in the 'ION_CHANNEL_TOKEN'
        # Environment Variable
        api_key_param = ssm.StringParameter(
            self,
            IC_API_KEY,
            string_value=os.environ.get("ION_CHANNEL_TOKEN"),
            parameter_name=IC_API_KEY,
        )
        api_key_param.grant_read(self.func)

        # Storing the Ion Channel Host here for consistency.
        ic_base_url_param = ssm.StringParameter(
            self,
            IC_API_BASE,
            string_value="api.ionchannel.io",
            parameter_name=IC_API_BASE,
        )
        ic_base_url_param.grant_read(self.func)

        # Storing the Ion Channel Team ID here for consistency as well.
        ic_team_id_param = ssm.StringParameter(
            self,
            IC_RULESET_TEAM_ID,
            string_value="232a5775-9231-4083-9422-c2333cecb7da",
            parameter_name=IC_RULESET_TEAM_ID,
        )
        ic_team_id_param.grant_read(self.func)

    def get_lambda_function(self):

        """
        -> Getter for the actual construct
        """

        return self.func

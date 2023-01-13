"""
-> Module to house DependencyTrackInterfaceLambda
"""
from aws_cdk import Duration
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_elasticloadbalancingv2 as elbv2
from aws_cdk import aws_events as eventbridge
from aws_cdk import aws_lambda as lambda_
from aws_cdk import aws_s3 as i_bucket
from aws_cdk import aws_ssm as ssm
from constructs import Construct

from cyclonedx.constants import (
    AWS_ACCOUNT_ID_KEY,
    AWS_REGION_KEY,
    DT_API_BASE,
    DT_API_KEY,
    DT_ROOT_PWD,
    EMPTY_VALUE,
    ENVIRONMENT_KEY,
)
from deploy.constants import (
    AWS_ACCOUNT_ID,
    AWS_REGION_SHORT,
    DT_INTERFACE_ID,
    DT_INTERFACE_LN,
    ENVIRONMENT,
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.enrichment.dependency_track import DependencyTrackLoadBalancer
from deploy.util import create_asset


class DependencyTrackInterfaceLambda(Construct):

    """This Construct creates a Lambda
    use to manage Dependency Track operations"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        s3_bucket: i_bucket,
        event_bus: eventbridge.EventBus,
        load_balancer: DependencyTrackLoadBalancer,
    ):

        super().__init__(scope, DT_INTERFACE_LN)

        dt_func_sg = ec2.SecurityGroup(self, "LaunchTemplateSG", vpc=vpc)

        alb: elbv2.ApplicationLoadBalancer = load_balancer.get_load_balancer()
        alb.load_balancer_security_groups.append(dt_func_sg)
        fq_dn = alb.load_balancer_dns_name

        self.func = lambda_.Function(
            self,
            DT_INTERFACE_ID,
            function_name=DT_INTERFACE_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.handlers.dt_interface_handler",
            code=create_asset(self),
            environment={
                DT_API_BASE: fq_dn,
                ENVIRONMENT_KEY: ENVIRONMENT,
                AWS_REGION_KEY: AWS_REGION_SHORT,
                AWS_ACCOUNT_ID_KEY: AWS_ACCOUNT_ID,
            },
            timeout=Duration.minutes(1),
            security_groups=[dt_func_sg],
            memory_size=512,
        )

        event_bus.grant_put_events_to(self.func)
        s3_bucket.grant_put(self.func)
        s3_bucket.grant_read_write(self.func)

        root_pwd_param = ssm.StringParameter(
            self,
            DT_ROOT_PWD,
            string_value=EMPTY_VALUE,
            parameter_name=DT_ROOT_PWD,
        )

        root_pwd_param.grant_read(self.func)
        root_pwd_param.grant_write(self.func)

        api_key_param = ssm.StringParameter(
            self,
            DT_API_KEY,
            string_value=EMPTY_VALUE,
            parameter_name=DT_API_KEY,
        )

        api_key_param.grant_read(self.func)
        api_key_param.grant_write(self.func)

    def get_lambda_function(self):

        """
        -> Getter for the actual construct
        """

        return self.func

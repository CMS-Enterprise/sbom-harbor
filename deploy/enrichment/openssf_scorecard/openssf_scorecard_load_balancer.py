"""
-> Module for OpenSSF ScoreCard Fargate load balancing
"""
from aws_cdk import RemovalPolicy
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_elasticloadbalancingv2 as elbv2
from aws_cdk import aws_s3 as s3
from constructs import Construct

from cyclonedx.constants import (
    OPENSSF_API_PORT,
    OPENSSF_LOAD_BALANCER_ID,
    OPENSSF_LOAD_BALANCER_LISTENER_ID,
    OPENSSF_LOAD_BALANCER_TARGET_ID,
)
from deploy.constants import OPENSSF_LB_ID, OPENSSF_LB_LOGGING_ID, OPENSSF_LB_SG_ID


class OpenssfScorecardLoadBalancer(Construct):
    """
    -> Creates a load balancer for OpenSSFScorecard docker module
    """

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
    ):
        super().__init__(scope, OPENSSF_LB_ID)

        security_group = ec2.SecurityGroup(
            self,
            OPENSSF_LB_SG_ID,
            vpc=vpc,
        )

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(), connection=ec2.Port.tcp(OPENSSF_API_PORT)
        )

        load_balancer = elbv2.ApplicationLoadBalancer(
            self,
            OPENSSF_LOAD_BALANCER_ID,
            vpc=vpc,
            internet_facing=False,
            load_balancer_name=OPENSSF_LOAD_BALANCER_ID,
            security_group=security_group,
        )

        logs_s3_bucket = s3.Bucket(
            self,
            OPENSSF_LB_LOGGING_ID,
            removal_policy=RemovalPolicy.DESTROY,
        )
        load_balancer.log_access_logs(logs_s3_bucket)

        listener = load_balancer.add_listener(
            OPENSSF_LOAD_BALANCER_LISTENER_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=OPENSSF_API_PORT,
        )

        listener.add_targets(
            OPENSSF_LOAD_BALANCER_TARGET_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=OPENSSF_API_PORT,
        )

        self.load_balancer = load_balancer
        self.listener = listener

    def get_lb_target_listener(self) -> elbv2.ApplicationListener:

        """Returns the Target Listener
        which points to OpenSSF Scorecard"""

        return self.listener

    def get_load_balancer(self) -> elbv2.ApplicationLoadBalancer:

        """returns the load balancer
        construct to plug into other constructs"""

        return self.load_balancer

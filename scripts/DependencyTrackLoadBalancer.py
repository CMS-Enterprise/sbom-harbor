import aws_cdk as cdk
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_elasticloadbalancingv2 as elbv2
import aws_cdk.aws_s3 as s3
from constructs import Construct

from cyclonedx.constants import (
    DT_API_PORT,
    LOAD_BALANCER_ID,
    LOAD_BALANCER_LISTENER_ID,
    LOAD_BALANCER_TARGET_ID,
)

from scripts.constants import (
    DT_LB_ID,
    DT_LB_LOGGING_ID, DT_LB_SG_ID,
)


class DependencyTrackLoadBalancer(Construct):

    def __init__(self, scope: Construct, *, vpc: ec2.Vpc):

        super().__init__(scope, DT_LB_ID)

        security_group = ec2.SecurityGroup(
            self,
            DT_LB_SG_ID,
            vpc=vpc,
        )

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(),
            connection=ec2.Port.tcp(DT_API_PORT)
        )

        load_balancer = elbv2.ApplicationLoadBalancer(
            self, LOAD_BALANCER_ID, vpc=vpc,
            internet_facing=False,
            load_balancer_name=LOAD_BALANCER_ID,
            security_group=security_group
        )

        logs_s3_bucket = s3.Bucket(
            self,
            DT_LB_LOGGING_ID,
            removal_policy=cdk.RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )
        load_balancer.log_access_logs(logs_s3_bucket)

        listener = load_balancer.add_listener(
            LOAD_BALANCER_LISTENER_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=DT_API_PORT,
        )

        listener.add_targets(
            LOAD_BALANCER_TARGET_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=DT_API_PORT,
        )

        self.load_balancer = load_balancer
        self.listener = listener

    def get_lb_target_listener(self):
        return self.listener

    def get_lb_construct(self):
        return self.load_balancer

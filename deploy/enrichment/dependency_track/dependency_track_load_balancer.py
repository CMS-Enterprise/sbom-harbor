from aws_cdk import RemovalPolicy
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_elasticloadbalancingv2 as elbv2
from aws_cdk import aws_s3 as s3
from constructs import Construct

from deploy.constants import (
    DT_API_PORT,
    DT_LB_ID,
    DT_LB_LOGGING_ID,
    DT_LB_SG_ID,
    DT_LOAD_BALANCER_ID,
    DT_LOAD_BALANCER_LISTENER_ID,
    DT_LOAD_BALANCER_NAME,
    DT_LOAD_BALANCER_TARGET_ID,
)


class DependencyTrackLoadBalancer(Construct):

    """Creates a load balancer used to make requests
    to the Dependency Track instance running in ECS (Fargate)"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
    ):

        super().__init__(scope, DT_LB_ID)

        security_group = ec2.SecurityGroup(
            self,
            DT_LB_SG_ID,
            vpc=vpc,
        )

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(), connection=ec2.Port.tcp(DT_API_PORT)
        )

        load_balancer = elbv2.ApplicationLoadBalancer(
            self,
            DT_LOAD_BALANCER_ID,
            vpc=vpc,
            internet_facing=False,
            load_balancer_name=DT_LOAD_BALANCER_NAME,
            security_group=security_group,
        )

        logs_s3_bucket = s3.Bucket(
            self,
            DT_LB_LOGGING_ID,
            removal_policy=RemovalPolicy.DESTROY,
        )
        load_balancer.log_access_logs(logs_s3_bucket)

        listener = load_balancer.add_listener(
            DT_LOAD_BALANCER_LISTENER_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=DT_API_PORT,
        )

        listener.add_targets(
            DT_LOAD_BALANCER_TARGET_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=DT_API_PORT,
        )

        self.load_balancer = load_balancer
        self.listener = listener

    def get_lb_target_listener(self) -> elbv2.ApplicationListener:

        """Returns the Target Listener
        which points to Dependency Track"""

        return self.listener

    def get_load_balancer(self) -> elbv2.ApplicationLoadBalancer:

        """returns the load balancer
        construct to plug into other constructs"""

        return self.load_balancer

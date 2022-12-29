"""
-> Creates a fargate instance to manage the installation of the openssf scorecard
-> docker container
"""
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_ecs as ecs
from aws_cdk import aws_efs as efs
from aws_cdk import aws_elasticloadbalancingv2 as elbv2
from constructs import Construct

from cyclonedx.constants import ALLOW_SCORECARD_PORT_SG, OPENSSF_API_PORT
from deploy.constants import (
    OPENSSF_CONTAINER_ID,
    OPENSSF_DOCKER_ID,
    OPENSSF_FARGATE_CLUSTER_ID,
    OPENSSF_FARGATE_SVC_NAME,
    OPENSSF_MOUNT_ID,
    OPENSSF_TASK_DEF_ID,
)
from deploy.enrichment.openssf_scorecard import OpenssfScorecardLoadBalancer


class OpenssfScorecardFargateInstance(Construct):

    """This Construct creates a Fargate
    instance running Openssf Scorecard"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        load_balancer: OpenssfScorecardLoadBalancer,
    ):
        super().__init__(scope, OPENSSF_FARGATE_CLUSTER_ID)

        # create an ecs cluster for running openssf scorecard
        fargate_cluster = ecs.Cluster(self, OPENSSF_FARGATE_CLUSTER_ID, vpc=vpc)

        # create an efs mount for maintaining
        mount = efs.FileSystem(
            self,
            OPENSSF_MOUNT_ID,
            vpc=vpc,
            encrypted=True,
        )

        volume = ecs.Volume(
            name=OPENSSF_MOUNT_ID,
            efs_volume_configuration=ecs.EfsVolumeConfiguration(
                file_system_id=mount.file_system_id
            ),
        )

        volume_mount = ecs.MountPoint(
            container_path="/apiserver",
            source_volume=volume.name,
            read_only=False,
        )

        api_task_definition = ecs.TaskDefinition(
            self,
            OPENSSF_TASK_DEF_ID,
            compatibility=ecs.Compatibility.FARGATE,
            cpu="4096",
            memory_mib="8192",
            volumes=[volume],
        )

        container = api_task_definition.add_container(
            OPENSSF_CONTAINER_ID,
            image=ecs.ContainerImage.from_registry(OPENSSF_DOCKER_ID),
            logging=ecs.LogDrivers.aws_logs(stream_prefix="openssfScorecardApi"),
            environment={},
            cpu=4096,
            memory_reservation_mib=8192,
        )

        port_mapping = ecs.PortMapping(
            container_port=OPENSSF_API_PORT,
            host_port=OPENSSF_API_PORT,
            protocol=ecs.Protocol.TCP,
        )

        container.add_port_mappings(port_mapping)
        container.add_mount_points(volume_mount)

        security_group = ec2.SecurityGroup(self, ALLOW_SCORECARD_PORT_SG, vpc=vpc)

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(), connection=ec2.Port.tcp(OPENSSF_API_PORT)
        )

        service = ecs.FargateService(
            self,
            OPENSSF_FARGATE_SVC_NAME,
            cluster=fargate_cluster,
            task_definition=api_task_definition,
            desired_count=1,
            assign_public_ip=True,
            platform_version=ecs.FargatePlatformVersion.VERSION1_4,
            security_groups=[security_group],
        )

        service.register_load_balancer_targets(
            ecs.EcsTarget(
                container_name=OPENSSF_CONTAINER_ID,
                container_port=OPENSSF_API_PORT,
                new_target_group_id="ScorecardTargetGroup",
                listener=ecs.ListenerConfig.application_listener(
                    load_balancer.get_lb_target_listener(),
                    protocol=elbv2.ApplicationProtocol.HTTP,
                    port=OPENSSF_API_PORT,
                ),
            )
        )

        mount.connections.allow_default_port_from(service)

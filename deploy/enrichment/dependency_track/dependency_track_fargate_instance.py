from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_ecs as ecs
from aws_cdk import aws_efs as efs
from aws_cdk import aws_elasticloadbalancingv2 as elbv2
from constructs import Construct

from cyclonedx.constants import (
    ALLOW_DT_PORT_SG,
    DT_API_PORT,
    DT_CONTAINER_ID,
    DT_DOCKER_ID,
    DT_FARGATE_SVC_NAME,
    DT_TASK_DEF_ID,
    EFS_MOUNT_ID,
    FARGATE_CLUSTER_ID,
    FARGATE_CLUSTER_NAME,
)
from deploy.enrichment.dependency_track import DependencyTrackLoadBalancer


class DependencyTrackFargateInstance(Construct):

    """This Construct creates a Fargate
    instance running Dependency Track"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        load_balancer: DependencyTrackLoadBalancer,
    ):

        super().__init__(scope, FARGATE_CLUSTER_ID)

        # create an ecs cluster for running dependency track
        fargate_cluster = ecs.Cluster(
            self, FARGATE_CLUSTER_ID, cluster_name=FARGATE_CLUSTER_NAME, vpc=vpc
        )

        # create an efs mount for maintaining
        dt_mount = efs.FileSystem(
            self,
            EFS_MOUNT_ID,
            vpc=vpc,
            encrypted=True,
        )

        dt_volume = ecs.Volume(
            name=EFS_MOUNT_ID,
            efs_volume_configuration=ecs.EfsVolumeConfiguration(
                file_system_id=dt_mount.file_system_id
            ),
        )

        dt_volume_mount = ecs.MountPoint(
            container_path="/apiserver",
            source_volume=dt_volume.name,
            read_only=False,
        )

        dt_api_task_definition = ecs.TaskDefinition(
            self,
            DT_TASK_DEF_ID,
            compatibility=ecs.Compatibility.FARGATE,
            cpu="4096",
            memory_mib="8192",
            volumes=[dt_volume],
        )

        container = dt_api_task_definition.add_container(
            DT_CONTAINER_ID,
            image=ecs.ContainerImage.from_registry(DT_DOCKER_ID),
            logging=ecs.LogDrivers.aws_logs(stream_prefix="dependencyTrackApi"),
            environment={},
            cpu=4096,
            memory_reservation_mib=8192,
        )

        port_mapping = ecs.PortMapping(
            container_port=DT_API_PORT,
            host_port=DT_API_PORT,
            protocol=ecs.Protocol.TCP,
        )

        container.add_port_mappings(port_mapping)
        container.add_mount_points(dt_volume_mount)

        security_group = ec2.SecurityGroup(self, ALLOW_DT_PORT_SG, vpc=vpc)

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(), connection=ec2.Port.tcp(DT_API_PORT)
        )

        dt_service = ecs.FargateService(
            self,
            DT_FARGATE_SVC_NAME,
            cluster=fargate_cluster,
            task_definition=dt_api_task_definition,
            desired_count=1,
            assign_public_ip=True,
            platform_version=ecs.FargatePlatformVersion.VERSION1_4,
            security_groups=[security_group],
        )

        dt_service.register_load_balancer_targets(
            ecs.EcsTarget(
                container_name=DT_CONTAINER_ID,
                container_port=DT_API_PORT,
                new_target_group_id="DTTargetGroup",
                listener=ecs.ListenerConfig.application_listener(
                    load_balancer.get_lb_target_listener(),
                    protocol=elbv2.ApplicationProtocol.HTTP,
                    port=DT_API_PORT,
                ),
            )
        )

        dt_mount.connections.allow_default_port_from(dt_service)

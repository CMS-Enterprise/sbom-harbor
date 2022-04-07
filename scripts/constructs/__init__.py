import aws_cdk as cdk
import aws_cdk.aws_apigateway as apigwv1
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_ecs as ecs
import aws_cdk.aws_efs as efs
import aws_cdk.aws_elasticloadbalancingv2 as elbv2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_s3_notifications as s3n
import aws_cdk.aws_sqs as sqs
import aws_cdk.aws_ssm as ssm
from aws_cdk import Duration
from aws_cdk.aws_lambda import AssetCode
from aws_cdk.aws_lambda_event_sources import SqsEventSource
from aws_cdk.aws_s3 import Bucket
from constructs import Construct

from cyclonedx.constants import (
    DT_QUEUE_URL_EV,
    ALLOW_DT_PORT_SG,
    DT_API_BASE,
    DT_API_KEY,
    DT_API_PORT,
    DT_ROOT_PWD,
    EMPTY_VALUE,
    SBOM_BUCKET_NAME_EV,
    FINDINGS_QUEUE_URL_EV,
    LOAD_BALANCER_ID,
    LOAD_BALANCER_LISTENER_ID,
    LOAD_BALANCER_TARGET_ID
)

from scripts.constants import (
    PRISTINE_SBOM_INGRESS_API_ID,
    PRISTINE_SBOM_INGRESS_LN,
    CIDR,
    PRIVATE,
    PRIVATE_SUBNET_NAME,
    PUBLIC,
    PUBLIC_SUBNET_NAME,
    VPC_NAME,
    SBOM_API_PYTHON_RUNTIME,
    DT_CONTAINER_ID,
    DT_DOCKER_ID,
    DT_FARGATE_SVC_NAME,
    DT_INTERFACE_LN,
    DT_LB_ID,
    DT_LB_LOGGING_ID,
    DT_LB_SG_ID,
    DT_TASK_DEF_ID,
    EFS_MOUNT_ID,
    FARGATE_CLUSTER_ID,
    SBOM_ENRICHMENT_LN
)


class SBOMApiVpc(Construct):

    def __init__(self,  scope: Construct):

        super().__init__(scope, VPC_NAME)

        """Creates a VPC"""

        private_subnet = ec2.SubnetConfiguration(
            name=PRIVATE_SUBNET_NAME,
            subnet_type=PRIVATE,
            cidr_mask=26,
        )

        public_subnet = ec2.SubnetConfiguration(
            name=PUBLIC_SUBNET_NAME,
            subnet_type=PUBLIC,
            cidr_mask=26,
        )

        self.vpc = ec2.Vpc(
            self,
            VPC_NAME,
            cidr=CIDR,
            max_azs=2,
            enable_dns_support=True,
            enable_dns_hostnames=True,
            subnet_configuration=[
                private_subnet,
                public_subnet
            ],
            gateway_endpoints={
                "S3": ec2.GatewayVpcEndpointOptions(
                    service=ec2.GatewayVpcEndpointAwsService.S3
                )
            },
        )

    def get_vpc(self):
        return self.vpc


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

    def get_load_balancer(self):
        return self.load_balancer


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


class PristineSbomIngressLambda(Construct):

    def __init__(self, scope: Construct, *, vpc: ec2.Vpc,
                 code: AssetCode, s3_bucket: Bucket,):

        super().__init__(scope, PRISTINE_SBOM_INGRESS_LN)

        sbom_ingest_func = lambda_.Function(
            self,
            PRISTINE_SBOM_INGRESS_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.pristine_sbom_ingress_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_EV: s3_bucket.bucket_name
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        s3_bucket.grant_put(sbom_ingest_func)

        lambda_api = apigwv1.LambdaRestApi(
            self,
            id=PRISTINE_SBOM_INGRESS_API_ID,
            handler=sbom_ingest_func,
        )

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")


class DependencyTrackFargateInstance(Construct):

    def __init__(
            self,
            scope: Construct,
            *,
            vpc: ec2.Vpc,
            load_balancer: DependencyTrackLoadBalancer):

        super().__init__(scope, FARGATE_CLUSTER_ID)

        # create an ecs cluster for running dependency track
        fargate_cluster = ecs.Cluster(self, FARGATE_CLUSTER_ID, vpc=vpc)

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

        security_group = ec2.SecurityGroup(
            self,
            ALLOW_DT_PORT_SG,
            vpc=vpc
        )

        security_group.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(),
            connection=ec2.Port.tcp(DT_API_PORT)
        )

        dt_service = ecs.FargateService(
            self,
            DT_FARGATE_SVC_NAME,
            cluster=fargate_cluster,
            task_definition=dt_api_task_definition,
            desired_count=1,
            assign_public_ip=True,
            platform_version=ecs.FargatePlatformVersion.VERSION1_4,
            security_groups=[security_group]
        )

        dt_service.register_load_balancer_targets(ecs.EcsTarget(
            container_name=DT_CONTAINER_ID,
            container_port=DT_API_PORT,
            new_target_group_id="DTTargetGroup",
            listener=ecs.ListenerConfig.application_listener(
                load_balancer.get_lb_target_listener(),
                protocol=elbv2.ApplicationProtocol.HTTP,
                port=DT_API_PORT,
            ),
        ))

        dt_mount.connections.allow_default_port_from(dt_service)


class DependencyTrackInterfaceLambda(Construct):

    def __init__(self, scope: Construct, *, vpc: ec2.Vpc,
                 code: AssetCode, s3_bucket: Bucket,
                 input_queue: sqs.Queue,
                 output_queue: sqs.Queue,
                 load_balancer: DependencyTrackLoadBalancer):

        super().__init__(scope, DT_INTERFACE_LN)

        """Create the Lambda Function responsible for
        extracting results from DT given an SBOM."""

        dt_func_sg = ec2.SecurityGroup(self, "LaunchTemplateSG", vpc=vpc)

        alb: elbv2.ApplicationLoadBalancer = load_balancer.get_load_balancer()
        alb.load_balancer_security_groups.append(dt_func_sg)
        fq_dn = alb.load_balancer_dns_name

        dt_interface_function = lambda_.Function(
            self,
            DT_INTERFACE_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.dt_interface_handler",
            code=code,
            environment={
                FINDINGS_QUEUE_URL_EV: output_queue.queue_url,
                DT_API_BASE: fq_dn,
            },
            timeout=Duration.minutes(1),
            security_groups=[dt_func_sg],
            memory_size=512,
        )

        # Grant rights to send messages to the Queue
        output_queue.grant_send_messages(dt_interface_function)

        s3_bucket.grant_put(dt_interface_function)
        s3_bucket.grant_read_write(dt_interface_function)

        root_pwd_param = ssm.StringParameter(
            self,
            DT_ROOT_PWD,
            string_value=EMPTY_VALUE,
            parameter_name=DT_ROOT_PWD,
        )

        root_pwd_param.grant_read(dt_interface_function)
        root_pwd_param.grant_write(dt_interface_function)

        api_key_param = ssm.StringParameter(
            self,
            DT_API_KEY,
            string_value=EMPTY_VALUE,
            parameter_name=DT_API_KEY
        )

        api_key_param.grant_read(dt_interface_function)
        api_key_param.grant_write(dt_interface_function)

        event_source = SqsEventSource(input_queue)
        dt_interface_function.add_event_source(event_source)

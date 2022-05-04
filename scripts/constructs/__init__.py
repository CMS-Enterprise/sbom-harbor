""" This module is where all the higher level CDK constructs are stored """

from http import client
from json import load
from aws_cdk import (
    aws_apigateway as apigwv1,
    aws_cognito as cognito,
    aws_ec2 as ec2,
    aws_ecs as ecs,
    aws_efs as efs,
    aws_elasticloadbalancingv2 as elbv2,
    aws_elasticloadbalancingv2_actions as actions,
    aws_iam as iam,
    aws_lambda as lambda_,
    aws_s3 as s3,
    aws_s3_notifications as s3n,
    aws_sqs as sqs,
    aws_ssm as ssm,
    CfnOutput,
    Duration,
    RemovalPolicy,
)
from aws_cdk.aws_lambda import AssetCode
from aws_cdk.aws_lambda_event_sources import SqsEventSource
from aws_cdk.aws_s3 import IBucket
from constructs import Construct
from cyclonedx.constants import (
    ALLOW_DT_PORT_SG,
    APP_LOAD_BALANCER_ID,
    APP_LOAD_BALANCER_LISTENER_ID,
    APP_LOAD_BALANCER_TARGET_ID,
    APP_PORT,
    DT_API_BASE,
    DT_API_KEY,
    DT_API_PORT,
    DT_LOAD_BALANCER_ID,
    DT_LOAD_BALANCER_LISTENER_ID,
    DT_LOAD_BALANCER_TARGET_ID,
    DT_QUEUE_URL_EV,
    DT_ROOT_PWD,
    EMPTY_VALUE,
    SBOM_BUCKET_NAME_KEY,
)
from scripts.constants import (
    API_GW_ID_EXPORT_NAME,
    API_GW_URL_EXPORT_ID,
    APP_LB_ID,
    APP_LB_LOGGING_ID,
    APP_LB_SECURITY_GROUP_ID,
    CIDR,
    COGNITO_DOMAIN_PREFIX,
    COGNITO_POOLS_AUTH_ID,
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
    PRISTINE_SBOM_INGRESS_API_ID,
    PRISTINE_SBOM_INGRESS_LN,
    PRIVATE_SUBNET_NAME,
    PRIVATE,
    PUBLIC_SUBNET_NAME,
    PUBLIC,
    SBOM_API_PYTHON_RUNTIME,
    SBOM_ENRICHMENT_LN,
    USER_POOL_APP_CLIENT_ID,
    USER_POOL_DOMAIN_ID,
    USER_POOL_GROUP_DESCRIPTION,
    USER_POOL_GROUP_ID,
    USER_POOL_GROUP_NAME,
    USER_POOL_ID,
    USER_POOL_NAME,
    USER_ROLE_ID,
    USER_ROLE_NAME,
    VPC_ID,
    VPC_NAME,
)

class SBOMApiVpc(Construct):

    """This is the VPC used throughout the application.
    One single VPC for the app."""

    def __init__(
        self,
        scope: Construct,
    ):

        """Creates a VPC for SBOM ingest and enrichment"""

        super().__init__(scope, VPC_NAME)

        private_subnet = ec2.SubnetConfiguration(
            name=PRIVATE_SUBNET_NAME,
            subnet_type=PRIVATE,
            cidr_mask=26,
        )

        # TODO: release elastic IP addresses on teardown
        # see: https://us-east-1.console.aws.amazon.com/ec2/v2/home?region=us-east-1#Addresses:
        public_subnet = ec2.SubnetConfiguration(
            name=PUBLIC_SUBNET_NAME,
            subnet_type=PUBLIC,
            cidr_mask=26,
        )

        self.vpc = ec2.Vpc(
            self,
            id=VPC_ID,
            vpc_name=VPC_NAME,
            cidr=CIDR,
            max_azs=2,
            enable_dns_support=True,
            enable_dns_hostnames=True,
            subnet_configuration=[private_subnet, public_subnet],
            gateway_endpoints={
                "S3": ec2.GatewayVpcEndpointOptions(
                    service=ec2.GatewayVpcEndpointAwsService.S3
                )
            },
        )

        self.vpc.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_vpc(self) -> ec2.Vpc:

        """Returns the underlying VPC to plug into other constructs."""

        return self.vpc


class SBOMUserPool(Construct):

    """
    This class is used to create the user pool
    used throughout the application.
    """

    def __init__(
        self,
        scope: Construct,
    ):

        super().__init__(scope, USER_POOL_ID)

        self.id = USER_POOL_ID

        self.user_pool = cognito.UserPool(
            self,
            USER_POOL_ID,
            user_pool_name=USER_POOL_NAME,
            account_recovery=cognito.AccountRecovery.EMAIL_ONLY,
            auto_verify=cognito.AutoVerifiedAttrs(
                email=True,
            ),
            custom_attributes={
                "role_name": cognito.StringAttribute(min_len=5, max_len=15, mutable=False),
                "team_id": cognito.StringAttribute(min_len=5, max_len=15, mutable=False),
            },
            self_sign_up_enabled=True,
            sign_in_aliases=cognito.SignInAliases(
                email=True,
                phone=False,
                username=False,
                preferred_username=False,
            ),
            sign_in_case_sensitive=False,
            standard_attributes=cognito.StandardAttributes(
                email=cognito.StandardAttribute(
                    required=True,
                    mutable=False,
                ),
                fullname=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                given_name=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                family_name=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                locale=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                timezone=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
            ),
            password_policy=cognito.PasswordPolicy(
                min_length=8,
                require_symbols=True,
                require_digits=True,
                require_lowercase=True,
                require_uppercase=True,
            ),
            removal_policy=RemovalPolicy.DESTROY,
        )

    def get_cognito_user_pool(self) -> cognito.UserPool:
        return self.user_pool


class SBOMUserRole(Construct):
    """
    This class is used to create the IAM role for the user pool .
    params:
        scope: Construct
        user_pool: SBOMUserPool
    """

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
    ):

        super().__init__(scope, USER_ROLE_ID)

        self.node.add_dependency(user_pool)

        self.user_role = iam.Role(
            self,
            USER_ROLE_ID,
            role_name=USER_ROLE_NAME,
            description='Default role for authenticated users',
            assumed_by=iam.FederatedPrincipal(
                "cognito-identity.amazonaws.com",
                {
                    "StringEquals": {
                        "cognito-identity.amazonaws.com:aud": user_pool.get_cognito_user_pool().user_pool_id,
                    },
                    "ForAnyValue:StringLike": {
                        "cognito-identity.amazonaws.com:amr": "authenticated",
                    },
                },
            ),
            managed_policies=[
                iam.ManagedPolicy.from_aws_managed_policy_name(
                    "service-role/AWSLambdaBasicExecutionRole",
                ),
                iam.ManagedPolicy.from_aws_managed_policy_name(
                    # TODO: remove this when we have a better solution
                    "AmazonS3FullAccess",
                ),
            ],
        )

        self.user_role.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_role(self) -> iam.Role:
        return self.user_role


class SBOMUserPoolGroup(Construct):

    """
    This class is used to create the user pool group.
    params:
        scope: Construct
        user_pool: SBOMUserPool
        user_role: SBOMUserRole
    """

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
        user_role: SBOMUserRole,
    ):

        super().__init__(scope, USER_POOL_GROUP_ID)

        for dep in [user_pool, user_role]:
            self.node.add_dependency(dep)

        self.user_pool_group = cognito.CfnUserPoolGroup(
            self,
            USER_POOL_GROUP_ID,
            description=USER_POOL_GROUP_DESCRIPTION,
            group_name=USER_POOL_GROUP_NAME,
            precedence=1,
            role_arn=user_role.get_cognito_user_role().role_arn,
            user_pool_id=user_pool.get_cognito_user_pool().user_pool_id,
        )

        self.user_pool_group.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_pool_group(self) -> cognito.CfnUserPoolGroup:
        return self.user_pool_group


class SBOMUserPoolClient(Construct):

    """
    This class is used to create the user pool app client.
    params:
        scope: Construct
        user_pool: SBOMUserPool

    """

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
    ):

        super().__init__(scope, USER_POOL_APP_CLIENT_ID)

        self.node.add_dependency(user_pool)

        client_write_attributes = (
            cognito.ClientAttributes()
        ).with_standard_attributes(
            email=True,
            phone_number=True,
            family_name=True,
            fullname=True,
            given_name=True,
            locale=True,
            preferred_username=True,
            timezone=True,
        )

        client_read_attributes = (
            client_write_attributes
        ).with_standard_attributes(
            email_verified=True,
            phone_number_verified=True,
        ).with_custom_attributes(
            "custom:role_name",
            "custom:team_id",
        )

        self.client = cognito.UserPoolClient(
            self,
            USER_POOL_APP_CLIENT_ID,
            user_pool=user_pool.get_cognito_user_pool(),
            auth_flows=cognito.AuthFlow(
                custom=True,
                user_password=True,
                user_srp=True,
            ),
            enable_token_revocation=True,
            prevent_user_existence_errors=True,
            read_attributes=client_read_attributes,
            write_attributes=client_write_attributes,
        )

        cfn_client = self.client.node.default_child
        cfn_client.add_property_override("RefreshTokenValidity", 1)
        cfn_client.add_property_override("SupportedIdentityProviders", ["COGNITO"])

        self.client.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_pool_client(self) -> cognito.UserPoolClient:
        return self.client


class SBOMUserPoolDomain(Construct):

    """This class is used to create the user pool app client domain."""

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
    ):

        super().__init__(scope, USER_POOL_DOMAIN_ID)

        self.node.add_dependency(user_pool)

        self.user_pool_domain = cognito.UserPoolDomain(
            self,
            USER_POOL_DOMAIN_ID,
            user_pool=user_pool.get_cognito_user_pool(),
            cognito_domain=cognito.CognitoDomainOptions(
                domain_prefix=COGNITO_DOMAIN_PREFIX,
            ),
        )

        self.user_pool_domain.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_pool_domain(self) -> cognito.UserPoolDomain:
        return self.user_pool_domain


class ApplicationLoadBalancer(Construct):

    """Creates a load balancer used to make requests
    to the Dependency Track instance running in ECS (Fargate)"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        user_pool: SBOMUserPool,
        user_pool_client: SBOMUserPoolClient,
        user_pool_domain: SBOMUserPoolDomain,
    ):

        super().__init__(scope, APP_LB_ID)

        for dep in [vpc, user_pool, user_pool_client, user_pool_domain]:
            self.node.add_dependency(dep)

        security_group = ec2.SecurityGroup(
            self,
            APP_LB_SECURITY_GROUP_ID,
            vpc=vpc,
        )

        security_group.add_ingress_rule(
            connection=ec2.Port.tcp(APP_PORT),
            peer=ec2.Peer.any_ipv4(),
        )

        self.load_balancer = elbv2.ApplicationLoadBalancer(
            self,
            APP_LOAD_BALANCER_ID,
            vpc=vpc,
            internet_facing=True,
            load_balancer_name=APP_LOAD_BALANCER_ID,
            security_group=security_group,
        )

        self.load_balancer.log_access_logs(
            s3.Bucket(
                self,
                APP_LB_LOGGING_ID,
                auto_delete_objects=True,
                removal_policy=RemovalPolicy.DESTROY,
            )
        )

        self.listener = self.load_balancer.add_listener(
            APP_LOAD_BALANCER_LISTENER_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=APP_PORT,
            default_action=actions.AuthenticateCognitoAction(
                user_pool=user_pool.get_cognito_user_pool(),
                user_pool_client=user_pool_client.get_cognito_user_pool_client(),
                user_pool_domain=user_pool_domain.get_cognito_user_pool_domain(),
                next=elbv2.ListenerAction.fixed_response(200,
                    content_type="text/plain",
                    message_body="Authenticated",
                ),
            ),
        )

        self.listener.add_targets(
            APP_LOAD_BALANCER_TARGET_ID,
            protocol=elbv2.ApplicationProtocol.HTTP,
            port=APP_PORT,
        )

        for construct in self.load_balancer, self.listener:
            construct.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_target_listener(self) -> elbv2.ApplicationListener:

        """Returns the Target Listener
        which points to Dependency Track"""

        return self.listener

    def get_load_balancer(self) -> elbv2.ApplicationLoadBalancer:

        """returns the load balancer
        construct to plug into other constructs"""

        return self.load_balancer


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
            load_balancer_name=DT_LOAD_BALANCER_ID,
            security_group=security_group,
        )

        logs_s3_bucket = s3.Bucket(
            self,
            DT_LB_LOGGING_ID,
            removal_policy=RemovalPolicy.DESTROY,
            auto_delete_objects=True,
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


class EnrichmentIngressLambda(Construct):

    """Create the Lambda Function responsible for listening on the S3 Bucket
    for SBOMs being inserted so they can be inserted into the enrichment process."""

    def __init__(
        self,
        scope: Construct,
        s3_bucket: s3.IBucket,
        *,
        vpc: ec2.Vpc,
        code: AssetCode,
        output_queue: sqs.Queue,
    ):

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
                SBOM_BUCKET_NAME_KEY: s3_bucket.bucket_name,
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
        s3_bucket.add_event_notification(
            s3.EventType.OBJECT_CREATED,
            s3n.LambdaDestination(sbom_enrichment_ingress_func),
        )


class SbomApiLoginLambda(Construct):

    """ Lambda to manage logging in """

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        code: AssetCode,
        s3_bucket: IBucket,
        user_pool: cognito.UserPool
    ):

        super().__init__(scope, PRISTINE_SBOM_INGRESS_LN)

        # TODO Complete
        login_func = lambda_.Function(
            self,
            PRISTINE_SBOM_INGRESS_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PUBLIC),
            handler="cyclonedx.api.login_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_KEY: s3_bucket.bucket_name,
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        pass


class PristineSbomIngressLambda(Construct):

    """Constructs a Lambda that can take
    Pristine SBOMS and puts them in the S3 Bucket"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        code: AssetCode,
        s3_bucket: IBucket,
        user_pool: cognito.UserPool
    ):

        super().__init__(scope, PRISTINE_SBOM_INGRESS_LN)

        auth = apigwv1.CognitoUserPoolsAuthorizer(
            self, COGNITO_POOLS_AUTH_ID,
            cognito_user_pools=[user_pool]
        )

        sbom_ingest_func = lambda_.Function(
            self,
            PRISTINE_SBOM_INGRESS_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.pristine_sbom_ingress_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_KEY: s3_bucket.bucket_name,
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

        CfnOutput(
            self,
            API_GW_URL_EXPORT_ID,
            # TODO: get rest api url dynamically for "value" below
            value=lambda_api.rest_api_id + ".execute-api.us-east-1.amazonaws.com",
            export_name=API_GW_ID_EXPORT_NAME,
            description="",
        )

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method(
            "POST", authorizer=auth,
            authorization_type=apigwv1.AuthorizationType.COGNITO,
        )


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


class DependencyTrackInterfaceLambda(Construct):

    """This Construct creates a Lambda
    use to manage Dependency Track operations"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        code: AssetCode,
        s3_bucket: IBucket,
        input_queue: sqs.Queue,
        load_balancer: DependencyTrackLoadBalancer,
    ):

        super().__init__(scope, DT_INTERFACE_LN)

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
                DT_API_BASE: fq_dn,
            },
            timeout=Duration.minutes(1),
            security_groups=[dt_func_sg],
            memory_size=512,
        )

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
            self, DT_API_KEY, string_value=EMPTY_VALUE, parameter_name=DT_API_KEY
        )

        api_key_param.grant_read(dt_interface_function)
        api_key_param.grant_write(dt_interface_function)

        event_source = SqsEventSource(input_queue)
        dt_interface_function.add_event_source(event_source)

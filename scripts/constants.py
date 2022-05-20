""" This module has constants throughout the build code """

from os import getenv
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_

an = getenv("AWS_ACCOUNT_NUM")

AUTHORIZATION_HEADER = "Authorization"

COGNITO_POOLS_AUTH_ID = "cognito_pools_auth_id"
COGNITO_DOMAIN_PREFIX = f"sbom-web-app-{an}"
USER_POOL_DOMAIN_ID = f"SBOMUserPoolDomain{an}"
USER_POOL_ID = f"SBOMUserPool_ID_{an}"
USER_POOL_NAME = f"SBOMUserPool_{an}"
USER_POOL_GROUP_ID = f"SBOMUserPoolGroup_Admin_ID_{an}"
USER_POOL_GROUP_NAME = f"SBOMUserPoolGroup_Admin_{an}"
USER_POOL_GROUP_DESCRIPTION = "Default group for authenticated administrator users"
USER_POOL_APP_CLIENT_ID = "SBOMUserPool_AppClient"
USER_POOL_APP_CLIENT_NAME = "SBOMUserPool_App"
USER_ROLE_ID = "SBOMUserRole_ID"
USER_ROLE_NAME = "SBOMUserRole"
ADMIN_USER_ID = "sbomadmin"
ADMIN_USER_USERNAME = "sbomadmin@aquia.us"

APP_LB_ID = "AppLoadBalancer"
APP_LB_SECURITY_GROUP_ID = "AppLoadBalancer-SecurityGroup"

SBOM_API_PYTHON_RUNTIME = lambda_.Runtime.PYTHON_3_9

PRISTINE_SBOM_INGRESS_LN = "PristineSBOMIngressLambda"
CREATE_TOKEN_LN = "CreateTokenLambda"
DELETE_TOKEN_LN = "DeleteTokenLambda"
REGISTER_TEAM_LN = "RegisterTeamLambda"
LOGIN_LN = "LoginLambda"
SBOM_ENRICHMENT_LN = "SBOMEnrichmentIngressLambda"
DT_INTERFACE_LN = "DependencyTrackInterfaceLambda"
ENRICHMENT_EGRESS_LN = "EnrichmentEgressLambda"
AUTHORIZER_LN = "JwtTokenAuthorizer"
TOKEN_AUTHORIZER_LN = "APITokenAuthorizer"
API_KEY_AUTHORIZER_LN = "APIKeyAuthorizer"

DT_LB_ID = "DEPENDENCY-TRACK-LOAD-BALANCER"
DT_LB_SG_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-SECURITY-GROUP"
DT_LB_LOGGING_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-LOGGING"

PRISTINE_SBOM_INGRESS_API_ID = "SBOM-API"

DT_DOCKER_ID = "dependencytrack/apiserver"
DT_INSTALL_LOC = "/apiserver"

UI_CONFIG_FILE_NAME = "config.json"
S3_WS_BUCKET_NAME = f"sbom.webapp.bucket.{an}"
S3_WS_BUCKET_ID = f"{S3_WS_BUCKET_NAME}.id"
API_GW_URL_KEY = "apigw_url"

S3_BUCKET_ID = f"sbom.bucket.id.{an}"
S3_BUCKET_NAME = f"sbom.bucket.{an}"
INGRESS_BUCKET_NAME = f"ingress.{S3_BUCKET_NAME}"
ENRICHMENT_BUCKET_NAME = f"enrichment.{S3_BUCKET_NAME}"

UI_DEPLOYMENT_ID = "ui_deployment_id"

VPC_ID = "sbom.vpc.id"
VPC_NAME = "sbom.vpc"

STACK_ID = "SBOMApi"
SHARED_RESOURCE_STACK_ID = f"{STACK_ID}-Shared-Resource"
ENRICHMENT_STACK_ID = f"{STACK_ID}-Enrichment"
INGRESS_STACK_ID = f"{STACK_ID}-Ingress"
WEB_STACK_ID = f"{STACK_ID}-Web"
USER_MANAGEMENT_STACK_ID = f"{STACK_ID}-User-Management"

VPC_TAG_NAME = f"{SHARED_RESOURCE_STACK_ID}/{VPC_NAME}/{VPC_NAME}"

CIDR = "10.0.0.0/16"
EC2_INSTANCE_NAME = "DependencyTrack"
EC2_SSH_KEY_NAME = "aquia"
EC2_INSTANCE_AMI = "ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20220131"
EC2_INSTANCE_TYPE = "t2.medium"
PRIVATE = ec2.SubnetType.PRIVATE_WITH_NAT
PUBLIC = ec2.SubnetType.PUBLIC
REST_API_NAME = "SBOMApi"
PRIVATE_SUBNET_NAME = "SBOMPrivateSubnet"
PUBLIC_SUBNET_NAME = "SBOMPublicSubnet"

EFS_MOUNT_ID = "dtApiStorage"
DT_CONTAINER_ID = "dtContainer"
FARGATE_CLUSTER_ID = "DTFargateCluster"
DT_FARGATE_SVC_NAME = "DTFargateService"
DT_TASK_DEF_ID = "dtTaskDefinition"

DT_SBOM_QUEUE_NAME = "DT_SBOMQueue"
DT_REST_API_GATEWAY = "DT_REST_API_GW"
DT_API_INTEGRATION = "DT_API_INT"

API_GW_ID_EXPORT_NAME = "apigwurl"
API_GW_URL_EXPORT_ID = f"{API_GW_ID_EXPORT_NAME}id"

CLOUDFRONT_DIST_NAME = "sbomapidistribution"


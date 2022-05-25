""" This module has constants throughout the build code """

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
from scripts.environment import Environment

env = Environment()     # load environment variables
CONFIG = env.get_all()  # export env vars as "CONFIG"

# General
SBOM_API_PYTHON_RUNTIME = lambda_.Runtime.PYTHON_3_9

# Cognito
AUTHORIZATION_HEADER = "Authorization"
COGNITO_POOLS_AUTH_ID = "cognito_pools_auth_id"
COGNITO_DOMAIN_PREFIX = f"sbom-web-app-{env.get_aws_account()}"
USER_POOL_DOMAIN_ID = f"SBOMUserPoolDomain{env.get_aws_account()}"
USER_POOL_ID = f"SBOMUserPool_ID_{env.get_aws_account()}"
USER_POOL_NAME = f"SBOMUserPool_{env.get_aws_account()}"
USER_POOL_GROUP_ID = f"SBOMUserPoolGroup_Admin_ID_{env.get_aws_account()}"
USER_POOL_GROUP_NAME = f"SBOMUserPoolGroup_Admin_{env.get_aws_account()}"
USER_POOL_GROUP_DESCRIPTION = "Default group for authenticated administrator users"
USER_POOL_APP_CLIENT_ID = "SBOMUserPool_AppClient"
USER_POOL_APP_CLIENT_NAME = "SBOMUserPool_App"
USER_ROLE_ID = "SBOMUserRole_ID"
USER_ROLE_NAME = "SBOMUserRole"
ADMIN_USER_ID = "sbomadmin"
ADMIN_USER_USERNAME = "sbomadmin@aquia.us"

# Load Balancer
APP_LB_ID = "AppLoadBalancer"
APP_LB_SECURITY_GROUP_ID = "AppLoadBalancer-SecurityGroup"

# Lambdas
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

# Dependency Track
DT_API_INTEGRATION = "DT_API_INT"
DT_CONTAINER_ID = "dtContainer"
DT_DOCKER_ID = "dependencytrack/apiserver"
DT_FARGATE_SVC_NAME = "DTFargateService"
DT_INSTALL_LOC = "/apiserver"
DT_LB_ID = "DEPENDENCY-TRACK-LOAD-BALANCER"
DT_LB_LOGGING_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-LOGGING"
DT_LB_SG_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-SECURITY-GROUP"
DT_REST_API_GATEWAY = "DT_REST_API_GW"
DT_SBOM_QUEUE_NAME = "DT_SBOMQueue"
DT_TASK_DEF_ID = "dtTaskDefinition"
EFS_MOUNT_ID = "dtApiStorage"
FARGATE_CLUSTER_ID = "DTFargateCluster"

# APIs
PRISTINE_SBOM_INGRESS_API_ID = "SBOM-API"
REST_API_NAME = "SBOMApi"

# S3 - SBOMs
S3_BUCKET_ID = f"sbom.bucket.id.{env.get_aws_account()}"
S3_BUCKET_NAME = f"sbom.bucket.{env.get_aws_account()}"
INGRESS_BUCKET_NAME = f"ingress.{S3_BUCKET_NAME}"
ENRICHMENT_BUCKET_NAME = f"enrichment.{S3_BUCKET_NAME}"

# S3 - UI
S3_WS_BUCKET_NAME = f"sbom.webapp.bucket.{env.get_aws_account()}"
S3_WS_BUCKET_ID = f"{S3_WS_BUCKET_NAME}.id"
UI_CONFIG_FILE_NAME = "config.json"
UI_DEPLOYMENT_ID = "ui_deployment_id"

# Stacks
STACK_ID = "SBOMApi"
ENRICHMENT_STACK_ID = f"{STACK_ID}-Enrichment"
INGRESS_STACK_ID = f"{STACK_ID}-Ingress"
SHARED_RESOURCE_STACK_ID = f"{STACK_ID}-Shared-Resource"
USER_MANAGEMENT_STACK_ID = f"{STACK_ID}-User-Management"
WEB_STACK_ID = f"{STACK_ID}-Web"

# VPC
VPC_ID = "sbom.vpc.id"
VPC_NAME = "sbom.vpc"
VPC_TAG_NAME = f"{SHARED_RESOURCE_STACK_ID}/{VPC_NAME}/{VPC_NAME}"

# EC2
CIDR = "10.0.0.0/16"
EC2_INSTANCE_AMI = "ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20220131"
EC2_INSTANCE_NAME = "DependencyTrack"
EC2_INSTANCE_TYPE = "t2.medium"
EC2_SSH_KEY_NAME = "aquia"

# Subnets
PRIVATE = ec2.SubnetType.PRIVATE_WITH_NAT
PRIVATE_SUBNET_NAME = "SBOMPrivateSubnet"
PUBLIC = ec2.SubnetType.PUBLIC
PUBLIC_SUBNET_NAME = "SBOMPublicSubnet"

# API Gateway
API_GW_ID_EXPORT_NAME = "apigwurl"
API_GW_URL_EXPORT_ID = f"{API_GW_ID_EXPORT_NAME}id"
API_GW_URL_KEY = "apigw_url"

# Cloudfront
CLOUDFRONT_DIST_NAME = "sbomapidistribution"
CLOUDFRONT_BUCKET_NAME = f"cloudfront.logging.bucket.{env.get_aws_account()}"

""" This module has constants throughout the build code """
from os import getenv

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
from boto3 import session
from dotenv import load_dotenv

# load environment variables from .env file into os.environ
load_dotenv(".env")

ENVIRONMENT = getenv("ENVIRONMENT") or getenv("AWS_VAULT") or "sandbox"
awsSession = session.Session()
regionCodes = {
    "us-east-1": "use1",
    "us-east-2": "use2",
    "us-west-1": "usw1",
    "us-west-2": "usw2",
}

# General
SBOM_API_PYTHON_RUNTIME = lambda_.Runtime.PYTHON_3_9

# AWS
AWS_ACCOUNT_ID = awsSession.client("sts").get_caller_identity().get("Account")
AWS_REGION = awsSession.region_name
AWS_REGION_SHORT = regionCodes.get(AWS_REGION)

# Cognito
AUTHORIZATION_HEADER = "Authorization"
USER_POOL_ID = "CognitoUserPool"
USER_POOL_NAME = f"{ENVIRONMENT}-HarborUsers-{AWS_REGION_SHORT}"
USER_POOL_GROUP_ID = "AdminsUserPoolGroup"
USER_POOL_GROUP_NAME = f"{ENVIRONMENT}-Admins-{AWS_REGION_SHORT}"
USER_POOL_GROUP_DESCRIPTION = "Default group for authenticated administrator users"
USER_POOL_APP_CLIENT_ID = "UserPoolAppClient"
USER_POOL_APP_CLIENT_NAME = f"{ENVIRONMENT}-Harbor-{AWS_REGION_SHORT}"
USER_ROLE_ID = "CognitoUserRole"
USER_ROLE_NAME = f"{ENVIRONMENT}-CognitoUser-{AWS_REGION_SHORT}"

# Load Balancer
APP_LB_ID = "AppLoadBalancer"
APP_LB_SECURITY_GROUP_ID = f"{ENVIRONMENT}-AppLoadBalancer-{AWS_REGION_SHORT}"

# Lambdas
SBOM_INGRESS_LN = "SBOMIngress"
CREATE_TOKEN_LN = "CreateToken"
DELETE_TOKEN_LN = "DeleteToken"
REGISTER_TEAM_LN = "RegisterTeam"
UPDATE_TEAM_LN = "UpdateTeam"
GET_TEAM_LN = "GetTeam"
GET_TEAMS_FOR_ID_LN = "GetTeamsForId"
USER_SEARCH_LN = "UserSearch"
LOGIN_LN = "Login"
SBOM_ENRICHMENT_LN = "SBOMEnrichmentIngress"
DT_INTERFACE_LN = "DependencyTrackInterface"
IC_INTERFACE_LN = "IonChannelInterface"
DEFAULT_INTERFACE_LN = "DefaultEnrichmentInterface"
ENRICHMENT_EGRESS_LN = "EnrichmentEgress"
AUTHORIZER_LN = "JwtTokenAuthorizer"
TOKEN_AUTHORIZER_LN = "APITokenAuthorizer"
API_KEY_AUTHORIZER_LN = "APIKeyAuthorizer"
SUMMARIZER_LN = "Summarizer"
SBOM_GENERATOR_LN = "SBOMGenerator"

# External bucket integration point
EXTERNAL_BUCKET_NAME = "sbom-harbor-summary-share-test"

# Dependency Track
DT_API_INTEGRATION = "DT_API_INT"
DT_CONTAINER_ID = "dtContainer"
DT_DOCKER_ID = "dependencytrack/apiserver"
DT_FARGATE_SVC_NAME = f"{ENVIRONMENT}-DTFargateService-{AWS_REGION_SHORT}"
DT_INSTALL_LOC = "/apiserver"
DT_LB_ID = "DEPENDENCY-TRACK-LOAD-BALANCER"
DT_LB_LOGGING_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-LOGGING"
DT_LB_SG_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-SECURITY-GROUP"
DT_REST_API_GATEWAY = "DT_REST_API_GW"
DT_SBOM_QUEUE_NAME = f"{ENVIRONMENT}-DT_SBOMQueue-{AWS_REGION_SHORT}"
DT_TASK_DEF_ID = "dtTaskDefinition"
EFS_MOUNT_ID = "dtApiStorage"
FARGATE_CLUSTER_ID = "DTFargateCluster"

# S3 - SBOMs
S3_BUCKET_ID = "UploadsEnrichmentBucket"
S3_BUCKET_NAME = (
    f"{ENVIRONMENT}-harbor-sbom-uploads-enrichment-{AWS_ACCOUNT_ID}-{AWS_REGION_SHORT}"
)

# S3 - UI
S3_WS_BUCKET_NAME = (
    f"{ENVIRONMENT}-harbor-web-assets-{AWS_ACCOUNT_ID}-{AWS_REGION_SHORT}"
)
S3_WS_BUCKET_ID = "WebAssetsBucket"
UI_DEPLOYMENT_ID = "UiDeployment"


# Stacks
STACK_ID_PREFIX = f"{ENVIRONMENT}-harbor"
ENRICHMENT_STACK_ID = f"{STACK_ID_PREFIX}-enrichment-{AWS_REGION_SHORT}"
API_STACK_ID = f"{STACK_ID_PREFIX}-backend-{AWS_REGION_SHORT}"
SHARED_RESOURCE_STACK_ID = f"{STACK_ID_PREFIX}-shared-resources-{AWS_REGION_SHORT}"
USER_MANAGEMENT_STACK_ID = f"{STACK_ID_PREFIX}-user-management-{AWS_REGION_SHORT}"
WEB_STACK_ID = f"{STACK_ID_PREFIX}-frontend-{AWS_REGION_SHORT}"
SBOM_GENERATOR_STACK_ID = f"{STACK_ID_PREFIX}-sbom-generator-{AWS_REGION_SHORT}"

# VPC
VPC_ID = "HarborNetworkVpc"
VPC_NAME = f"{ENVIRONMENT}-HarborNetwork-{AWS_REGION_SHORT}"
VPC_TAG_NAME = f"{SHARED_RESOURCE_STACK_ID}/{VPC_NAME}/{VPC_NAME}"

# EC2
CIDR = "10.0.0.0/16"

# Subnets
PRIVATE = ec2.SubnetType.PRIVATE_WITH_EGRESS
PRIVATE_SUBNET_NAME = "Private"
PUBLIC = ec2.SubnetType.PUBLIC
PUBLIC_SUBNET_NAME = "Public"

# API Gateway
API_GW_ID_EXPORT_NAME = "apigwurl"

# Cloudfront
CLOUDFRONT_DIST_ID = "CloudFrontDistribution"

# GitHub
# SBOM Generator
GH_FETCH_TOKEN_KEY = "GH_FETCH_TOKEN"
GH_FETCH_TOKEN = getenv("GH_FETCH_TOKEN")
CF_DOMAIN_KEY = "CF_DOMAIN"
CF_DOMAIN = getenv("CF_DOMAIN")
HARBOR_USERNAME_KEY = "HARBOR_USERNAME"
HARBOR_PASSWORD_KEY = "HARBOR_PASSWORD"
HARBOR_USERNAME = getenv("HARBOR_USERNAME")
HARBOR_PASSWORD = getenv("HARBOR_PASSWORD")

""" This module has constants throughout the build code """
from os import getenv, path

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_

regionCodes = {
    "us-east-1": "use1",
    "us-east-2": "use2",
    "us-west-1": "usw1",
    "us-west-2": "usw2",
}

# General
SBOM_API_PYTHON_RUNTIME = lambda_.Runtime.PYTHON_3_9
PYTHON_LOGGING_CONFIG = path.join(
    path.dirname(path.dirname(__file__)), "python_logging.conf"
)

# AWS
AWS_PROFILE = getenv("AWS_PROFILE")
AWS_ACCOUNT_ID = getenv("CDK_DEFAULT_ACCOUNT")
AWS_REGION = getenv("CDK_DEFAULT_REGION")
AWS_REGION_SHORT = regionCodes.get(AWS_REGION)
ENVIRONMENT = getenv("ENVIRONMENT") or "sandbox"

# CMS
CMS_PERMISSION_BOUNDARY_ARN = (
    f"arn:aws:iam::{AWS_ACCOUNT_ID}:policy/cms-cloud-admin/"
    f"ct-ado-poweruser-permissions-boundary-policy"
)
CMS_ROLE_PATH = "/delegatedadmin/developer/"

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
PILOT_LN = "Pilot"

# External bucket integration point
EXTERNAL_BUCKET_NAME = "dev-harbor-sbom-summary-share-557147098836-use1"

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
S3_WS_BUCKET_ID = "WebAssetsBucket"
S3_WS_BUCKET_NAME = (
    f"{ENVIRONMENT}-harbor-web-assets-{AWS_ACCOUNT_ID}-{AWS_REGION_SHORT}"
)

# Stacks
STACK_ID_PREFIX = f"{ENVIRONMENT}-harbor"
ENRICHMENT_STACK_ID = f"{STACK_ID_PREFIX}-enrichment-{AWS_REGION_SHORT}"
API_STACK_ID = f"{STACK_ID_PREFIX}-backend-{AWS_REGION_SHORT}"
SHARED_RESOURCE_STACK_ID = f"{STACK_ID_PREFIX}-shared-resources-{AWS_REGION_SHORT}"
USER_MANAGEMENT_STACK_ID = f"{STACK_ID_PREFIX}-user-management-{AWS_REGION_SHORT}"
WEB_STACK_ID = f"{STACK_ID_PREFIX}-frontend-{AWS_REGION_SHORT}"
SBOM_GENERATOR_STACK_ID = f"{STACK_ID_PREFIX}-sbom-generator-{AWS_REGION_SHORT}"
PILOT_STACK_ID = f"{STACK_ID_PREFIX}-pilot-{AWS_REGION_SHORT}"

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
API_GW_URL_EXPORT_NAME = "apigwurl"
API_GW_ID_EXPORT_NAME = "apigwid"

# Cloudfront
CLOUDFRONT_DIST_ID = "CloudFrontDistribution"

# DevOps
DEVOPS_STACK_ID = f"{ENVIRONMENT}-harbor-devops-{AWS_REGION_SHORT}"
DEVOPS_OIDC_PROVIDER_ID = "DevopsOidcProvider"
DEVOPS_GITHUB_ROLE_ID = "DevopsGithubRole"
DEVOPS_GITHUB_ORG = "aquia-inc"
DEVOPS_GITHUB_REPO = "cyclonedx-python"

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

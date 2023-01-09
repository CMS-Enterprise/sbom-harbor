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


def environize(name: str, delimiter: str = "-", include_account=False) -> str:
    """environize returns a name that is unique per environment.
    Set include_account=True for resources that need to be globally unique such as S3 buckets."""

    environment_unique_name = f"{ENVIRONMENT}{delimiter}{name}"
    if include_account:
        environment_unique_name += f"{delimiter}{AWS_ACCOUNT_ID}"
    environment_unique_name += f"{delimiter}{AWS_REGION_SHORT}"
    return environment_unique_name


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
ENVIRONMENT = ENVIRONMENT.lower()

# CMS
CMS_PERMISSION_BOUNDARY_ARN = (
    f"arn:aws:iam::{AWS_ACCOUNT_ID}:policy/cms-cloud-admin/"
    f"ct-ado-poweruser-permissions-boundary-policy"
)
CMS_ROLE_PATH = "/delegatedadmin/developer/"

# Cognito
AUTHORIZATION_HEADER = "Authorization"
USER_POOL_ID = "CognitoUserPool"
USER_POOL_NAME = environize("HarborUsers")
USER_POOL_GROUP_ID = "AdminsUserPoolGroup"
USER_POOL_GROUP_NAME = environize("Admins")
USER_POOL_GROUP_DESCRIPTION = "Default group for authenticated administrator users"
USER_POOL_APP_CLIENT_ID = "UserPoolAppClient"
USER_POOL_APP_CLIENT_NAME = environize("Harbor")
USER_ROLE_ID = "CognitoUserRole"
USER_ROLE_NAME = environize("CognitoUser")

# Load Balancer
APP_LB_ID = "AppLoadBalancer"


# Lambdas
SBOM_INGRESS_LN = environize("SBOMIngress", delimiter="_")
CREATE_TOKEN_LN = environize("CreateToken", delimiter="_")
DELETE_TOKEN_LN = environize("DeleteToken", delimiter="_")
REGISTER_TEAM_LN = environize("RegisterTeam", delimiter="_")
UPDATE_TEAM_LN = environize("UpdateTeam", delimiter="_")
GET_TEAM_LN = environize("GetTeam", delimiter="_")
GET_TEAMS_FOR_ID_LN = environize("GetTeamsForId", delimiter="_")
USER_SEARCH_LN = environize("UserSearch", delimiter="_")
LOGIN_LN = environize("Login", delimiter="_")
SBOM_ENRICHMENT_LN = environize("SBOMEnrichmentIngress", delimiter="_")
DT_INTERFACE_LN = environize("DependencyTrackInterface", delimiter="_")
IC_INTERFACE_LN = environize("IonChannelInterface", delimiter="_")
DEFAULT_INTERFACE_LN = environize("DefaultEnrichmentInterface", delimiter="_")
ENRICHMENT_EGRESS_LN = environize("EnrichmentEgress", delimiter="_")
AUTHORIZER_LN = environize("JwtTokenAuthorizer", delimiter="_")
API_KEY_AUTHORIZER_LN = environize("APIKeyAuthorizer", delimiter="_")
SUMMARIZER_LN = environize("Summarizer", delimiter="_")
SBOM_GENERATOR_LN = environize("SBOMGenerator", delimiter="_")
PILOT_LN = environize("Pilot", delimiter="_")

# External bucket integration point
EXTERNAL_BUCKET_NAME = "dev-harbor-sbom-summary-share-557147098836-use1"

# Dependency Track
DT_API_INTEGRATION = "DT_API_INT"
DT_CONTAINER_ID = "dtContainer"
DT_DOCKER_ID = "dependencytrack/apiserver"
DT_FARGATE_SVC_NAME = environize("DTFargateService")
DT_INSTALL_LOC = "/apiserver"
DT_LB_ID = "DEPENDENCY-TRACK-LOAD-BALANCER"
DT_LB_LOGGING_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-LOGGING"
DT_LB_SG_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-SECURITY-GROUP"
DT_REST_API_GATEWAY = "DT_REST_API_GW"
DT_SBOM_QUEUE_NAME = environize("DT_SBOMQueue")
DT_TASK_DEF_ID = "DependencyTrackTaskDefinition"
DT_LOAD_BALANCER_ID = "DependencyTrackAlb"
DT_LOAD_BALANCER_NAME = environize("DependencyTrack")
DT_LOAD_BALANCER_LISTENER_ID = "DT-LB-LISTENER-ID"
DT_LOAD_BALANCER_TARGET_ID = "DT-LB-TARGET-ID"
EFS_MOUNT_ID = "dtApiStorage"
FARGATE_CLUSTER_ID = "HarborFargateCluster"
FARGATE_CLUSTER_NAME = environize("Harbor")
DT_ROOT_PWD = environize("DT_ROOT_PWD", delimiter="_")
DT_API_KEY = environize("DT_API_KEY", delimiter="_")
DT_API_BASE = environize("DT_API_BASE", delimiter="_")
DT_API_PORT = 8080

# S3 - SBOMs
S3_BUCKET_ID = "UploadsEnrichmentBucket"
S3_BUCKET_NAME = environize("harbor-sbom-uploads-enrichment", include_account=True)

# S3 - UI
S3_WS_BUCKET_ID = "WebAssetsBucket"
S3_WS_BUCKET_NAME = environize("harbor-web-assets", include_account=True)

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
VPC_NAME = environize("HarborNetwork")

# EC2
CIDR = "10.0.0.0/16"

# Subnets
PRIVATE = ec2.SubnetType.PRIVATE_WITH_EGRESS
PRIVATE_SUBNET_NAME = "Private"
PUBLIC = ec2.SubnetType.PUBLIC
PUBLIC_SUBNET_NAME = "Public"

# API Gateway
API_GW_URL_OUTPUT_ID = "apigwurl"
API_GW_URL_EXPORT_NAME = environize("apigwurl")
API_GW_ID_OUTPUT_ID = "apigwid"
API_GW_LOG_GROUP_NAME = environize("ApiGwAccessLogs")

# Cloudfront
CLOUDFRONT_DIST_ID = "CloudFrontDistribution"

# DevOps
DEVOPS_STACK_ID = environize("harbor-devops")
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

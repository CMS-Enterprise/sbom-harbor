""" Constants to be used throughout the system """

from os import getenv, path

from aws_cdk import Duration

regionCodes = {
    "us-east-1": "use1",
    "us-east-2": "use2",
    "us-west-1": "usw1",
    "us-west-2": "usw2",
}

# Environment vars
AWS_ACCOUNT_ID = getenv("CDK_DEFAULT_ACCOUNT")
AWS_REGION = getenv("CDK_DEFAULT_REGION") or getenv("AWS_REGION")
ENVIRONMENT = getenv("ENVIRONMENT") or "sandbox"
ENVIRONMENT = ENVIRONMENT.lower()
AWS_REGION_SHORT = regionCodes.get(AWS_REGION)


def environize(name: str, delimiter: str = "-", include_account=False) -> str:
    """environize returns a name that is unique per environment.
    Set include_account=True for resources that need to be globally unique such as S3 buckets."""

    environment_unique_name = f"{ENVIRONMENT}{delimiter}{name}"
    if include_account:
        environment_unique_name += f"{delimiter}{AWS_ACCOUNT_ID}"
    environment_unique_name += f"{delimiter}{AWS_REGION_SHORT}"
    return environment_unique_name


PYTHON_LOGGING_CONFIG = path.join(path.dirname(__file__), "python_logging.conf")

APP_PORT = 433

COGNITO_TEAM_DELIMITER = ","

# Enrichments
ENRICHMENT_ID = "ENRICHMENTID"
ENRICHMENT_ID_SQS_KEY = "enrichmentid"
FINDINGS_SQS_KEY = "findings_key"
SBOM_BUCKET_NAME_KEY = "sbom_bucket"
SBOM_S3_KEY = "sbom_s3_key"

# Dependency Track
DT_DEFAULT_ADMIN_PWD = "admin"
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

S3_META_TEAM_KEY = "x-amz-meta-sbom-api-team"
S3_META_PROJECT_KEY = "x-amz-meta-sbom-api-project"
S3_META_CODEBASE_KEY = "x-amz-meta-sbom-api-codebase"
S3_META_TIMESTAMP_KEY = "x-amz-meta-sbom-api-timestamp"

EMPTY_VALUE = "EMPTY"

APP_LOAD_BALANCER_ID = "AppLoadBalancer-ID"
APP_LOAD_BALANCER_LISTENER_ID = "AppLoadBalancer-Target-ID"
APP_LOAD_BALANCER_TARGET_ID = "AppLoadBalancer-Target-ID"

ALLOW_DT_PORT_SG = f"ALLOW_{DT_API_PORT}_SG"

USER_POOL_ID_KEY = "USER_POOL_NAME"
USER_POOL_CLIENT_ID_KEY = "USER_POOL_CLIENT_ID"

# DynamoDB
HARBOR_TEAMS_TABLE_NAME = environize("HarborTeams")
HARBOR_TEAMS_TABLE_ID = "TeamsDynamoDbTable"
HARBOR_TEAMS_TABLE_PARTITION_KEY = "TeamId"
HARBOR_TEAMS_TABLE_SORT_KEY = "EntityKey"

# Event Bus
EVENT_BUS_ID = "EnrichmentEventBus"
EVENT_BUS_NAME = environize("HarborEnrichments")
EVENT_BUS_SOURCE = "enrichment.lambda"
EVENT_BUS_DETAIL_TYPE = "SBOM.Event.Detail.Type"


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
STANDARD_LAMBDA_TIMEOUT = Duration.minutes(15)
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

PRIVATE_SUBNET_NAME = "Private"

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

# Ion Channel
IC_API_BASE = environize("IC_API_BASE", delimiter="_")
IC_API_KEY = environize("IC_API_KEY", delimiter="_")
IC_RULESET_TEAM_ID = environize("IC_RULESET_TEAM_ID", delimiter="_")

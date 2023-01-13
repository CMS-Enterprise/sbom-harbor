""" Constants to be used throughout the system"""
import os
from os import path

PYTHON_LOGGING_CONFIG = path.join(
    path.dirname(path.dirname(__file__)), "python_logging.conf"
)

ENVIRONMENT_KEY = "ENVIRONMENT"
AWS_REGION_KEY = "AWS_REGION_SHORT"
AWS_ACCOUNT_ID_KEY = "AWS_ACCOUNT_ID"

ENVIRONMENT = os.environ.get(ENVIRONMENT_KEY)
AWS_REGION_SHORT = os.environ.get(AWS_REGION_KEY)
AWS_ACCOUNT_ID = os.environ.get(AWS_ACCOUNT_ID_KEY)


def environize(name: str, delimiter: str = "-", include_account=False) -> str:
    """environize returns a name that is unique per environment.
    Set include_account=True for resources that need to be globally unique such as S3 buckets."""

    environment_unique_name = f"{ENVIRONMENT}{delimiter}{name}"
    if include_account:
        environment_unique_name += f"{delimiter}{AWS_ACCOUNT_ID}"
    environment_unique_name += f"{delimiter}{AWS_REGION_SHORT}"
    return environment_unique_name


DT_API_PORT = 8080
APP_PORT = 433

COGNITO_TEAM_DELIMITER = ","

EFS_MOUNT_ID = "dtApiStorage"
ENRICHMENT_ID = "ENRICHMENTID"
DT_TASK_DEF_ID = "dtTaskDefinition"
DT_DEFAULT_ADMIN_PWD = "admin"
ENRICHMENT_ID_SQS_KEY = "enrichmentid"
FINDINGS_SQS_KEY = "findings_key"
SBOM_BUCKET_NAME_KEY = "sbom_bucket"
SBOM_S3_KEY = "sbom_s3_key"
DT_API_KEY = environize("DT_API_KEY", include_account=True)
DT_API_BASE = environize("DT_API_BASE", include_account=True)
DT_ROOT_PWD = environize("DT_ROOT_PWD", include_account=True)

IC_API_BASE = environize("IC_API_BASE", include_account=True)
IC_API_KEY = environize("IC_API_KEY", include_account=True)
IC_RULESET_TEAM_ID = environize("IC_RULESET_TEAM_ID", include_account=True)

S3_META_TEAM_KEY = "x-amz-meta-sbom-api-team"
S3_META_PROJECT_KEY = "x-amz-meta-sbom-api-project"
S3_META_CODEBASE_KEY = "x-amz-meta-sbom-api-codebase"
S3_META_TIMESTAMP_KEY = "x-amz-meta-sbom-api-timestamp"

EMPTY_VALUE = "EMPTY"

APP_LOAD_BALANCER_ID = f"{ENVIRONMENT}-AppLoadBalancer-ID"
APP_LOAD_BALANCER_LISTENER_ID = f"{ENVIRONMENT}-AppLoadBalancer-Target-ID"
APP_LOAD_BALANCER_TARGET_ID = f"{ENVIRONMENT}-AppLoadBalancer-Target-ID"

DT_LB_ID = "DT-LB-ID"
DT_LOAD_BALANCER_LISTENER_ID = f"{ENVIRONMENT}-DT-LB-LISTENER-ID"
DT_LOAD_BALANCER_TARGET_ID = f"{ENVIRONMENT}-DT-LB-TARGET-ID"
DT_LOAD_BALANCER_NAME = f"{ENVIRONMENT}-DTLOADBALANCERNAME"
DT_LB_LOGGING_ID = f"{ENVIRONMENT}-DT_LB_LOGGING_ID"
DT_LB_SG_ID = f"{ENVIRONMENT}-DT_LB_SG_ID"
DT_CONTAINER_ID = f"{ENVIRONMENT}-dtContainer"
DT_DOCKER_ID = f"{ENVIRONMENT}-dependencytrack/apiserver"

DT_FARGATE_SVC_NAME = environize("DTFargateService", delimiter="_")

ALLOW_DT_PORT_SG = f"ALLOW_{DT_API_PORT}_SG"

USER_POOL_ID_KEY = "USER_POOL_NAME"
USER_POOL_CLIENT_ID_KEY = "USER_POOL_CLIENT_ID"

# DynamoDB
HARBOR_TEAMS_TABLE_NAME = f"{ENVIRONMENT}-HarborTeams"
HARBOR_TEAMS_TABLE_ID = "TeamsDynamoDbTable"
HARBOR_TEAMS_TABLE_PARTITION_KEY = "TeamId"
HARBOR_TEAMS_TABLE_SORT_KEY = "EntityKey"

# Event Bus
EVENT_BUS_ID = "EnrichmentEventBus"
EVENT_BUS_NAME = f"{ENVIRONMENT}-HarborEnrichments"
EVENT_BUS_SOURCE = "enrichment.lambda"
EVENT_BUS_DETAIL_TYPE = "SBOM.Event.Detail.Type"

FARGATE_CLUSTER_ID = environize("HarborFargateCluster", delimiter="_")
FARGATE_CLUSTER_NAME = environize("_HarborFargateClusterName_", delimiter="_")

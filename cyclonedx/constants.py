""" Constants to be used throughout the system"""
from os import path
from deploy.constants import DT_API_PORT, environize

PYTHON_LOGGING_CONFIG = path.join(
    path.dirname(path.dirname(__file__)), "python_logging.conf"
)

APP_PORT = 433

COGNITO_TEAM_DELIMITER = ","

ENRICHMENT_ID = "ENRICHMENTID"

DT_DEFAULT_ADMIN_PWD = "admin"
ENRICHMENT_ID_SQS_KEY = "enrichmentid"
FINDINGS_SQS_KEY = "findings_key"
SBOM_BUCKET_NAME_KEY = "sbom_bucket"
SBOM_S3_KEY = "sbom_s3_key"


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

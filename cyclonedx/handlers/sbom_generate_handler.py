"""
-> Module to house the SBOM Generator Handler
"""
from cyclonedx import harbor_logger
from cyclonedx.handlers.common import (
    QueryStringKeys,
    _extract_value_from_qs,
    harbor_response,
)

# config.fileConfig(PYTHON_LOGGING_CONFIG)
logger = harbor_logger.getChild(__name__)


def sbom_generate_handler(event: dict = None, context: dict = None) -> dict:
    """
    This is the Handler that validates an incoming SBOM generate request,
    and if valid, invokes the SBOM generate Lambda.
    """

    # Create a response object to add values to.
    response: dict = {}  # TODO

    try:
        team_id: str = _extract_value_from_qs(QueryStringKeys.TEAM_ID, event)
        project_id: str = _extract_value_from_qs(QueryStringKeys.PROJECT_ID, event)
        token: str = event["authorizationToken"]

        logger.info(team_id)
        logger.info(project_id)

        if token is None:
            return harbor_response(403, response)

    except KeyError as key_error:
        response["statusCode"] = 400
        response["body"] = str(key_error)

    return harbor_response(200, response)

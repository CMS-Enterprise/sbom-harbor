from botocore.exceptions import ClientError

from cyclonedx.constants import (
    TEAM_TOKEN_TABLE_NAME,
)
from cyclonedx.core_utils.handler_commons import dynamodb_resource
from cyclonedx.core_utils.cyclonedx_util import (
    __handle_delete_token_error,
    __token_response_obj,
)


def delete_token_handler(event: dict = None, context: dict = None):

    """ Handler for deleting a token belonging to a given team """

    # Grab the team and the token from the path parameters
    team_id = event["pathParameters"]["team"]
    token = event["pathParameters"]["token"]

    # Get our Team table from DynamoDB
    team_token_table = dynamodb_resource.Table(TEAM_TOKEN_TABLE_NAME)

    try:

        # Delete the token
        team_token_table.delete_item(
            Key={
                "TeamId": team_id,
                "token": token
            },
            ConditionExpression="attribute_exists(TeamId)",
        )

    except ClientError as e:
        return __handle_delete_token_error(
            token, team_id, e
        )

    return __token_response_obj(
        status_code=200,
        token=token,
    )

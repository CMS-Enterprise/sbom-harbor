"""
-> Module for Registering Teams
"""
import boto3
from jsonschema import validate
from jsonschema.exceptions import ValidationError

from cyclonedx.constants import (
    TEAM_MEMBER_TABLE_NAME,
    TEAM_TOKEN_TABLE_NAME,
    TEAM_TABLE_NAME,
)
from cyclonedx.handlers.common import team_schema
from cyclonedx.handlers.cyclonedx_util import (
    __create_team_response,
    __get_body_from_event,
)


def register_team_handler(event: dict = None, context: dict = None):

    """
    -> Register Team Handler
    """

    team_json: dict = __get_body_from_event(event)

    try:
        validate(instance=team_json, schema=team_schema)

        team_id = team_json["Id"]

        dynamodb_resource = boto3.resource("dynamodb")

        def update_table(
            key: str,
            update_team_json: dict,
            update_team_id: str,
            table: dynamodb_resource.Table,
        ):
            items = update_team_json[key]
            for item in items:
                item.update(
                    {
                        "TeamId": update_team_id,
                    }
                )
                table.put_item(Item=item)

        # Add the tokens to the token table if there are tokens.
        token_table = dynamodb_resource.Table(TEAM_TOKEN_TABLE_NAME)
        update_table("tokens", team_json, team_id, token_table)
        del team_json["tokens"]

        # Update the members
        member_table = dynamodb_resource.Table(TEAM_MEMBER_TABLE_NAME)
        update_table("members", team_json, team_id, member_table)
        del team_json["members"]

        team_table = dynamodb_resource.Table(TEAM_TABLE_NAME)
        team_table.put_item(Item=team_json)

        return __create_team_response(200, "Team Created")

    except ValidationError as err:
        return __create_team_response(500, f"Validation Error: {err}")

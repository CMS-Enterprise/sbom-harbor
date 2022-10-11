"""
-> Module for Updating Teams
"""
from json import dumps

import boto3
from botocore.exceptions import ClientError
from jsonschema import validate
from jsonschema.exceptions import ValidationError

from cyclonedx.constants import (
    TEAM_TABLE_NAME,
    TEAM_MEMBER_TABLE_NAME,
)
from cyclonedx.handlers.common import team_schema
from cyclonedx.handlers.cyclonedx_util import (
    __create_team_response,
    __get_body_from_event,
)


def update_team_handler(event: dict = None, context: dict = None):

    """
    -> Handler for Updating Teams
    """

    team_json: dict = __get_body_from_event(event)

    print(f"Incoming Team JSON: {team_json}")

    dynamodb_resource = boto3.resource("dynamodb")

    try:
        validate(instance=team_json, schema=team_schema)

        team_id = team_json["Id"]

        if "members" in team_json:
            incoming_members = team_json["members"]
            errors = replace_members(team_id, incoming_members)
            del team_json["members"]
            if len(errors) > 0:
                return __create_team_response(status_code=500, err=dumps(errors))

        team_table = dynamodb_resource.Table(TEAM_TABLE_NAME)
        team_table.delete_item(Key={"Id": team_id})

        team_table.put_item(Item=team_json)

        return __create_team_response(status_code=200, msg="Team Updated")

    except ValidationError as err:
        return __create_team_response(
            status_code=500,
            err=f"Validation Error: {err}",
        )


def replace_members(team_id: str, new_members: list):

    """
    -> Function for Replacing Members
    """

    dynamodb_resource = boto3.resource("dynamodb")

    team_table = dynamodb_resource.Table(TEAM_MEMBER_TABLE_NAME)
    team_query_rsp = team_table.query(
        Select="ALL_ATTRIBUTES",
        KeyConditionExpression="TeamId = :TeamId",
        ExpressionAttributeValues={
            ":TeamId": team_id,
        },
    )

    delete_requests = []
    for item in team_query_rsp["Items"]:
        delete_requests.append(
            {"DeleteRequest": {"Key": {"TeamId": team_id, "email": item["email"]}}}
        )

    put_requests = []
    for member in new_members:
        put_requests.append(
            {
                "PutRequest": {
                    "Item": {
                        "TeamId": team_id,
                        "isTeamLead": member["isTeamLead"],
                        "email": member["email"],
                    }
                },
            }
        )

    errors = []

    print(f"Delete Requests: {delete_requests}")
    if len(delete_requests) > 0:
        try:
            dynamodb_resource.batch_write_item(
                RequestItems={"SbomTeamMemberTable": delete_requests},
            )
        except ClientError as err:
            errors.append(err)

    print(f"Put Requests: {put_requests}")
    if len(put_requests) > 0:
        try:
            dynamodb_resource.batch_write_item(
                RequestItems={"SbomTeamMemberTable": put_requests},
            )
        except ClientError as err:
            errors.append(err)

    return errors if len(errors) < 1 else []

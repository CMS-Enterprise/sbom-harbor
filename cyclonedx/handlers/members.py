"""
-> This module contains the handlers for CRUDing Members
"""
from uuid import uuid4

from json import dumps

import boto3

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers.common import (
    _extract_id_from_path,
    _extract_team_id_from_qs,
    _get_method,
    _get_request_body_as_dict,
    _print_values,
    _should_process_children,
)
from cyclonedx.model.team import Team
from cyclonedx.model.member import Member


def members_handler(event: dict, context: dict) -> dict:

    """
    ->  "Members" Handler. Handles requests to the /members endpoint.
    """

    _print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    # Use MemberId Extract existing
    # member from DynamoDB with children
    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    # fmt: off
    # Declare a response dictionary
    response: dict = {
        member.entity_id: member.to_json()
        for member in team.members
    }
    # fmt: on

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(response),
    }


def _do_get(event: dict, db_client: HarborDBClient) -> dict:

    # Get the member id from the path
    member_id: str = _extract_id_from_path("member", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    member = db_client.get(
        model=Member(
            team_id=team_id,
            member_id=member_id,
        ),
        recurse=_should_process_children(event),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({member_id: member.to_json()}),
    }


def _do_post(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> Handler that creates a member, puts it in
    -> DynamoDB and returns it to the requester
    """

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    request_body: dict = _get_request_body_as_dict(event)

    # Generate a new member id
    member_id: str = str(uuid4())

    member: Member = db_client.create(
        model=Member(
            team_id=team_id,
            member_id=member_id,
            email=request_body[Member.Fields.EMAIL],
            is_team_lead=request_body[Member.Fields.IS_TEAM_LEAD],
        ),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({member_id: member.to_json()}),
    }


def _do_put(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> The behavior of this function is that the objects in the request_body
    -> will be updated.
    """

    # Get the team id from the query string
    team_id: str = _extract_team_id_from_qs(event)

    # Get the member id from the path
    member_id: str = _extract_id_from_path("member", event)

    # Extract the request body from the event
    member_dict: dict = _get_request_body_as_dict(event)

    # Use MemberId Extract existing member from DynamoDB with children
    member: Member = db_client.get(
        model=Member(
            team_id=team_id,
            member_id=member_id,
        ),
    )

    member_item: dict = member.get_item()
    original_email: str = member_item.get(Member.Fields.EMAIL)
    original_is_team_lead: bool = member_item.get(Member.Fields.IS_TEAM_LEAD)

    # replace only the data in the existing object with the
    # new data from the request body ignoring children
    # Update that object in DynamoDB
    member: Member = Member(
        team_id=team_id,
        member_id=member_id,
        email=member_dict.get(Member.Fields.EMAIL, original_email),
        is_team_lead=member_dict.get(Member.Fields.IS_TEAM_LEAD, original_is_team_lead),
    )

    member: Member = db_client.update(
        model=member,
        recurse=False,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({member_id: member.to_json()}),
    }


def _do_delete(event: dict, db_client: HarborDBClient) -> dict:

    # Get the member id from the path
    member_id: str = _extract_id_from_path("member", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    member: Member = db_client.get(
        model=Member(
            team_id=team_id,
            member_id=member_id,
        ),
    )

    db_client.delete(
        model=member,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({member_id: member.to_json()}),
    }


def member_handler(event: dict, context: dict) -> dict:

    """
    ->  "Member" Handler.  Handles requests to the /member endpoint.
    """

    # Print the incoming values, so we can see them in
    # CloudWatch if there is an issue.
    _print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the verb (method) of the request.  We will use it
    # to decide what type of operation we execute on the incoming data
    method: str = _get_method(event)

    try:
        result: dict = {}
        if method == "GET":
            result = _do_get(event, db_client)
        elif method == "POST":
            result = _do_post(event, db_client)
        elif method == "PUT":
            result = _do_put(event, db_client)
        elif method == "DELETE":
            result = _do_delete(event, db_client)
        return result
    except ValueError as ve:
        return {
            "statusCode": 400,
            "isBase64Encoded": False,
            "body": dumps({"error": str(ve)}),
        }
    except DatabaseError as de:
        return {
            "statusCode": 400,
            "isBase64Encoded": False,
            "body": dumps({"error": str(de)}),
        }

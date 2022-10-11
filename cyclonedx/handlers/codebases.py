"""
-> This module contains the handlers for CRUDing CodeBases
"""
import uuid

from json import dumps, loads

import boto3

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.handlers.common import (
    _extract_id_from_path,
    _extract_team_id_from_qs,
    _get_method,
    _print_values,
    _should_process_children,
)
from cyclonedx.model.team import Team
from cyclonedx.model.codebase import CodeBase


def codebases_handler(event: dict, context: dict) -> dict:

    """
    ->  "CodeBases" Handler. Handles requests to the /codebases endpoint.
    """

    _print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    # Use ProjectId Extract existing
    # project from DynamoDB with children
    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    # fmt: off
    # Declare a response dictionary
    response: dict = {
        codebase.entity_id: codebase.to_json()
        for codebase in team.codebases
    }
    # fmt: on

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(response),
    }


def _do_get(event: dict, db_client: HarborDBClient) -> dict:

    # Get the project id from the path
    codebase_id: str = _extract_id_from_path("codebase", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    codebase = db_client.get(
        model=CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
        ),
        recurse=_should_process_children(event),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({codebase_id: codebase.to_json()}),
    }


def _do_post(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> Handler that creates a codebase, puts it in
    -> DynamoDB and returns it to the requester
    """

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    request_body: dict = loads(event["body"])
    codebase_id: str = str(uuid.uuid4())

    codebase: CodeBase = db_client.create(
        model=CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
            name=request_body[CodeBase.Fields.NAME],
        ),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({codebase_id: codebase.to_json()}),
    }


def _do_put(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> The behavior of this function is that the objects in the request_body
    -> will be updated.
    """

    # Get the codebase id from the path
    codebase_id: str = _extract_id_from_path("codebase", event)

    # Get the ProjectId from the Path Parameter
    team_id: str = _extract_team_id_from_qs(event)

    # Use ProjectId Extract existing project from DynamoDB with children
    codebase: CodeBase = db_client.get(
        model=CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
        ),
    )

    codebase = db_client.update(
        model=codebase,
        recurse=False,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({codebase_id: codebase.to_json()}),
    }


def _do_delete(event: dict, db_client: HarborDBClient) -> dict:

    # Get the project id from the path
    codebase_id: str = _extract_id_from_path("codebase", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    codebase: CodeBase = db_client.get(
        model=CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
        ),
    )

    db_client.delete(
        model=codebase,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({codebase_id: codebase.to_json()}),
    }


def codebase_handler(event: dict, context: dict) -> dict:

    """
    ->  "Project" Handler.  Handles requests to the /project endpoint.
    """

    # Print the incoming values, so we can see them in
    # CloudWatch if there is an issue.
    _print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the verb (method) of the request.  We will use it
    # to decide what type of operation we execute on the incoming data
    method: str = _get_method(event)

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

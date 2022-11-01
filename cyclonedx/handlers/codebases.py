"""
-> This module contains the handlers for CRUDing CodeBases
"""

from json import loads

import boto3

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers.common import (
    _extract_id_from_path,
    _extract_project_id_from_qs,
    _extract_team_id_from_qs,
    _get_method,
    print_values,
    harbor_response,
    _should_process_children,
    update_codebase_data,
)
from cyclonedx.model import generate_model_id
from cyclonedx.model.team import Team
from cyclonedx.model.codebase import CodeBase


def codebases_handler(event: dict, context: dict) -> dict:

    """
    ->  "CodeBases" Handler. Handles requests to the /codebases endpoint.
    """

    print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    # Use CodeBaseId Extract existing
    # codebase from DynamoDB with children
    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    # fmt: off
    # Declare a response dictionary
    codebase_lists: list[list[CodeBase]] = [
        project.codebases
        for project in team.projects
    ]
    codebases: list[CodeBase] = [
        codebase
        for codebase_list in
            codebase_lists
        for codebase in
            codebase_list
    ]
    resp: dict = {
        codebase.entity_id: codebase.to_json()
        for codebase in codebases
    }
    # fmt: on

    return harbor_response(200, resp)


def _do_get(event: dict, db_client: HarborDBClient) -> dict:

    # Get the codebase id from the path
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

    return harbor_response(
        200,
        {
            codebase_id: codebase.to_json(),
        },
    )


def _do_post(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> Handler that creates a codebase, puts it in
    -> DynamoDB and returns it to the requester
    """

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)
    project_id: str = _extract_project_id_from_qs(event)

    request_body: dict = loads(event["body"])
    codebase_id: str = generate_model_id()

    codebase: CodeBase = db_client.create(
        model=CodeBase(
            team_id=team_id,
            project_id=project_id,
            codebase_id=codebase_id,
            name=request_body[CodeBase.Fields.NAME],
            language=request_body[CodeBase.Fields.LANGUAGE],
            build_tool=request_body[CodeBase.Fields.BUILD_TOOL],
        ),
    )

    return harbor_response(
        200,
        {
            codebase_id: codebase.to_json(),
        },
    )


def _do_put(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> The behavior of this function is that the objects in the request_body
    -> will be updated.
    """

    # Get the team id from the query string
    team_id: str = _extract_team_id_from_qs(event)

    # Get the team id from the query string
    project_id: str = _extract_project_id_from_qs(event)

    # Get the codebase id from the path
    codebase_id: str = _extract_id_from_path("codebase", event)

    # Extract the request body from the event
    request_body: dict = loads(event["body"])

    # Use CodeBaseId Extract existing codebase from DynamoDB with children
    codebase: CodeBase = db_client.get(
        model=CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
        ),
    )

    codebase: CodeBase = update_codebase_data(
        team_id=team_id,
        project_id=project_id,
        codebase_id=codebase_id,
        codebase_item=codebase.get_item(),
        codebase_dict=request_body,
    )

    codebase: CodeBase = db_client.update(
        model=codebase,
        recurse=False,
    )

    return harbor_response(
        200,
        {
            codebase_id: codebase.to_json(),
        },
    )


def _do_delete(event: dict, db_client: HarborDBClient) -> dict:

    # Get the codebase id from the path
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

    return harbor_response(
        200,
        {
            codebase_id: codebase.to_json(),
        },
    )


def codebase_handler(event: dict, context: dict) -> dict:

    """
    ->  "CodeBase" Handler.  Handles requests to the /codebase endpoint.
    """

    # Print the incoming values, so we can see them in
    # CloudWatch if there is an issue.
    print_values(event, context)

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
    except (ValueError, DatabaseError) as e:
        return harbor_response(400, {"error": str(e)})

"""
-> This module contains the handlers for CRUDing Projects
"""

from json import dumps

import boto3

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers.common import (
    QueryStringKeys,
    _extract_id_from_path,
    _extract_value_from_qs,
    _get_method,
    _get_request_body_as_dict,
    _should_process_children,
    _to_codebases,
    _update_codebases,
    harbor_response,
    print_values,
)
from cyclonedx.model import generate_model_id
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team


def projects_handler(event: dict, context: dict) -> dict:

    """
    ->  "Projects" Handler. Handles requests to the /projects endpoint.
    """

    print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the team id from the querystring
    team_id: str = _extract_value_from_qs(QueryStringKeys.TEAM_ID, event)

    # Use ProjectId Extract existing project from DynamoDB with children
    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    # Declare a response dictionary
    response: dict = {project.entity_id: project.to_json() for project in team.projects}

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(response),
    }


def _do_get(event: dict, db_client: HarborDBClient) -> dict:

    # Get the project id from the path
    project_id: str = _extract_id_from_path("project", event)

    # Get the team id from the querystring
    team_id: str = _extract_value_from_qs(QueryStringKeys.TEAM_ID, event)

    project = db_client.get(
        model=Project(team_id=team_id, project_id=project_id),
        recurse=_should_process_children(event),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({project_id: project.to_json()}),
    }


def _do_post(event: dict, db_client: HarborDBClient) -> dict:

    # Get the team id from the querystring
    team_id: str = _extract_value_from_qs(QueryStringKeys.TEAM_ID, event)

    request_body: dict = _get_request_body_as_dict(event)
    project_id: str = generate_model_id()

    project: Project = db_client.create(
        model=Project(
            team_id=team_id,
            project_id=project_id,
            name=request_body[Project.Fields.NAME],
            codebases=_to_codebases(
                team_id=team_id, project_id=project_id, request_body=request_body
            ),
        ),
        recurse=True,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({project_id: project.to_json()}),
    }


def _do_put(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> The behavior of this function is that the objets in the request_body
    -> will be updated.  If a new object (project or member) comes in the request,
    -> it will not be created.  If a child object noes not exist in the request_body
    -> and exists in the database, the object will not be deleted.  Objects can only
    -> be modified, never created or deleted.
    """

    # Get the project id from the path
    project_id: str = _extract_id_from_path("project", event)

    # Get the ProjectId from the Path Parameter
    team_id: str = _extract_value_from_qs(QueryStringKeys.TEAM_ID, event)

    # Use ProjectId Extract existing project from DynamoDB with children
    project: Project = db_client.get(
        model=Project(
            team_id=team_id,
            project_id=project_id,
        ),
        recurse=True,
    )

    # Extract the request body from the event
    request_body: dict = _get_request_body_as_dict(event)

    # Replace the name of the project if there is a 'name' key in the request body
    try:
        project.name = request_body[Project.Fields.NAME]
    except KeyError:
        ...

    project = _update_codebases(
        project=project,
        request_body=request_body,
    )

    project = db_client.update(
        model=project,
        recurse=False,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({project_id: project.to_json()}),
    }


def _do_delete(event: dict, db_client: HarborDBClient) -> dict:

    # Get the project id from the path
    project_id: str = _extract_id_from_path("project", event)

    # Get the team id from the querystring
    team_id: str = _extract_value_from_qs(QueryStringKeys.TEAM_ID, event)

    project: Project = db_client.get(
        model=Project(team_id=team_id, project_id=project_id),
        recurse=True,
    )

    db_client.delete(
        model=project,
        recurse=True,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({project_id: project.to_json()}),
    }


def project_handler(event: dict, context: dict) -> dict:

    """
    ->  "Project" Handler.  Handles requests to the /project endpoint.
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

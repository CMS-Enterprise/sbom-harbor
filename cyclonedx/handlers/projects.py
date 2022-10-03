"""
-> This module contains the handlers for CRUDing Projects
"""

import uuid

from json import dumps, loads

from cyclonedx.handlers.common import (
    _extract_id_from_path,
    _extract_team_id_from_qs,
    _get_method,
    _print_values,
    _should_process_children,
    _to_codebases,
    _update_codebases,
    db_client,
)
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team


def projects_handler(event: dict, context: dict) -> dict:

    """
    ->  "Projects" Handler. Handles requests to the /projects endpoint.
    """

    _print_values(event, context)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

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


def _do_get(event: dict) -> dict:

    # Get the project id from the path
    project_id: str = _extract_id_from_path("projects", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    project = db_client.get(
        model=Project(team_id=team_id, project_id=project_id),
        recurse=_should_process_children(event),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({project_id: project.to_json()}),
    }


def _do_post(event: dict) -> dict:

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    request_body: dict = loads(event["body"])
    project_id: str = str(uuid.uuid4())

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


def _do_put(event: dict) -> dict:

    """
    -> The behavior of this function is that the objets in the request_body
    -> will be updated.  If a new object (project or member) comes in the request,
    -> it will not be created.  If a child object noes not exist in the request_body
    -> and exists in the database, the object will not be deleted.  Objects can only
    -> be modified, never created or deleted.
    """

    # Get the project id from the path
    project_id: str = _extract_id_from_path("projects", event)

    # Get the ProjectId from the Path Parameter
    team_id: str = _extract_team_id_from_qs(event)

    # Use ProjectId Extract existing project from DynamoDB with children
    project: Project = db_client.get(
        model=Project(
            team_id=team_id,
            project_id=project_id,
        ),
        recurse=True,
    )

    # Extract the request body from the event
    request_body: dict = loads(event["body"])

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


def _do_delete(event: dict) -> dict:

    # Get the project id from the path
    project_id: str = _extract_id_from_path("projects", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

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
    _print_values(event, context)

    # Get the verb (method) of the request.  We will use it
    # to decide what type of operation we execute on the incoming data
    method: str = _get_method(event)

    result: dict = {}
    if method == "GET":
        result = _do_get(event)
    elif method == "POST":
        result = _do_post(event)
    elif method == "PUT":
        result = _do_put(event)
    elif method == "DELETE":
        result = _do_delete(event)

    return result

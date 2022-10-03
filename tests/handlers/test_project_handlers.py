import uuid
from json import dumps, loads

import pytest

from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers import (
    projects_handler,
    project_handler,
)
from cyclonedx.handlers.common import db_client
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.team import Team


def test_flow():

    team_id: str = str(uuid.uuid4())

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
        ),
    )

    # Create
    create_response: dict = create(team_id=team_id)
    response_dict: dict = loads(create_response['body'])

    project_id: str = list(response_dict.keys()).pop()
    project_dict: dict = response_dict[project_id]
    codebases_dict: dict = project_dict['codebases']
    codebase_ids: list = list(codebases_dict.keys())
    assert len(codebase_ids) == 2

    # Get Test 1
    get_response: dict = get(
        team_id=team_id,
        project_id=project_id,
    )
    response_dict = loads(get_response['body'])

    project_id: str = list(response_dict.keys()).pop()
    project_dict: dict = response_dict[project_id]
    codebases_dict: dict = project_dict['codebases']
    codebase_ids: list = list(codebases_dict.keys())
    assert len(codebase_ids) == 2

    # Get Test 2
    get_response: dict = get_all(team_id=team_id)
    response_dict = loads(get_response['body'])

    project_id: str = list(response_dict.keys()).pop()
    project_dict: dict = response_dict[project_id]
    codebases_dict: dict = project_dict['codebases']
    codebase_ids: list = list(codebases_dict.keys())
    assert len(codebase_ids) == 2

    # Update
    cb1_id: str = codebase_ids.pop()
    cb2_id: str = codebase_ids.pop()
    new_cb1_name: str = "New Codebase 1 name"
    new_cb2_name: str = "New Codebase 2 name"

    update(
        team_id=team_id,
        project_id=project_id,
        codebase1_id=cb1_id,
        new_cb_name1=new_cb1_name,
        codebase2_id=cb2_id,
        new_cb_name2=new_cb2_name,
    )

    test_cb: CodeBase = db_client.get(
        CodeBase(
            team_id=team_id,
            project_id=project_id,
            codebase_id=cb1_id,
        )
    )

    assert test_cb.name == new_cb1_name

    # Delete
    delete(
        team_id=team_id,
        project_id=project_id
    )

    # Get Test (Should return nothing)
    try:
        get_response: dict = get(
            team_id=team_id,
            project_id=project_id
        )
        print(get_response)
        pytest.fail()
    except DatabaseError:
        db_client.delete(Team(team_id=team_id))
        print("All clear.  Database is clean")

def create(team_id: str):

    event: dict = {
        "requestContext": {
            "http": {
                "method": "POST",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
        "body": dumps({
            "name": "Initial Project Name",
            "codebases": [
                {
                    "name": "Initial Frontend Name",
                    "language": "JAVASCRIPT",
                    "buildTool": "YARN",
                }, {
                    "name": "Initial Backend Name",
                    "language": "PYTHON",
                    "buildTool": "POETRY",
                }
            ]
        }),
    }

    return project_handler(event, {})


def get(team_id: str, project_id: str):

    event: dict = {
        "pathParameters": {
            "projects": project_id,
        },
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
    }

    return project_handler(event, {})


def get_all(team_id: str):

    event: dict = {
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
    }

    return projects_handler(event, {})

def update(
    team_id: str,
    project_id: str,
    codebase1_id: str,
    codebase2_id: str,
    new_cb_name1: str,
    new_cb_name2: str,
):

    event: dict = {
        "pathParameters": {
            "projects": project_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
        "body": dumps({
            "name": "Updated Project Name",
            "codebases": [
                {
                    "id": codebase1_id,
                    "name": new_cb_name1
                }, {
                    "id": codebase2_id,
                    "name": new_cb_name2
                }
            ]
        }),
    }

    return project_handler(event, {})

def delete(team_id: str, project_id: str):

    event: dict = {
        "pathParameters": {
            "projects": project_id,
        },
        "requestContext": {
            "http": {
                "method": "DELETE",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
    }

    return project_handler(event, {})

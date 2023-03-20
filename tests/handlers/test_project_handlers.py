"""
-> Test module for the project handlers
"""
import uuid
from json import dumps, loads

import boto3
from moto import mock_dynamodb

from cyclonedx.clients.db.dynamodb import HarborDBClient
from cyclonedx.handlers import project_handler, projects_handler
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.team import Team
from tests.conftest import create_mock_dynamodb_infra


@mock_dynamodb
def test_update():

    """
    -> Special test to ensure the data in the codebases is updating
    """

    db_client: HarborDBClient = HarborDBClient(
        dynamodb_resource=boto3.resource("dynamodb")
    )

    create_mock_dynamodb_infra(boto3.resource("dynamodb"))

    team_id: str = str(uuid.uuid4())

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
        ),
    )

    # Create
    create_response: dict = create(team_id=team_id, handler=project_handler)
    project_dict: dict = loads(create_response["body"])
    codebases_list: list = project_dict["codebases"]
    assert len(codebases_list) == 2

    project_id: str = project_dict["id"]

    # Get Test 1
    get_response: dict = get(
        team_id=team_id,
        project_id=project_id,
        handler=project_handler,
    )

    project_dict = loads(get_response["body"])
    codebases_list: dict = project_dict["codebases"]
    assert len(codebases_list) == 2

    # Get Test 2
    get_response: dict = get_all(
        team_id=team_id,
        handler=projects_handler,
    )
    response_list: list = loads(get_response["body"])

    project_dict: dict = response_list[0]
    codebases_list: list = project_dict["codebases"]
    assert len(codebases_list) == 2

    # Update
    cb1_id: str = codebases_list[0]["id"]
    cb2_id: str = codebases_list[1]["id"]
    new_cb1_name: str = "New Codebase 1 name"
    new_cb2_name: str = "New Codebase 2 name"

    update(
        team_id=team_id,
        project_id=project_id,
        codebase1_id=cb1_id,
        new_cb_name1=new_cb1_name,
        codebase2_id=cb2_id,
        new_cb_name2=new_cb2_name,
        handler=project_handler,
    )

    test_cb: CodeBase = db_client.get(
        CodeBase(
            team_id=team_id,
            project_id=project_id,
            codebase_id=cb1_id,
        )
    )

    assert test_cb.name == new_cb1_name


@mock_dynamodb
def test_flow():

    """
    -> Test the creation, updating and deletion of a project.
    """

    db_client: HarborDBClient = HarborDBClient(
        dynamodb_resource=boto3.resource("dynamodb")
    )

    create_mock_dynamodb_infra(boto3.resource("dynamodb"))

    team_id: str = str(uuid.uuid4())

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
        ),
    )

    # Create
    create_response: dict = create(team_id=team_id, handler=project_handler)
    project_dict: dict = loads(create_response["body"])
    codebases_list: list = project_dict["codebases"]
    assert len(codebases_list) == 2

    project_id: str = project_dict["id"]

    # Get Test 1
    get_response: dict = get(
        team_id=team_id,
        project_id=project_id,
        handler=project_handler,
    )

    project_dict = loads(get_response["body"])
    codebases_list: dict = project_dict["codebases"]
    assert len(codebases_list) == 2

    # Get Test 2
    get_response: dict = get_all(
        team_id=team_id,
        handler=projects_handler,
    )
    response_list: list = loads(get_response["body"])

    project_dict: dict = response_list[0]
    codebases_list: list = project_dict["codebases"]
    assert len(codebases_list) == 2

    # Update
    cb1_id: str = codebases_list[0]["id"]
    cb2_id: str = codebases_list[1]["id"]
    new_cb1_name: str = "New Codebase 1 name"
    new_cb2_name: str = "New Codebase 2 name"

    update(
        team_id=team_id,
        project_id=project_id,
        codebase1_id=cb1_id,
        new_cb_name1=new_cb1_name,
        codebase2_id=cb2_id,
        new_cb_name2=new_cb2_name,
        handler=project_handler,
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
        project_id=project_id,
        handler=project_handler,
    )

    # Get Test (Should return nothing)
    get_response: dict = get(
        team_id=team_id,
        project_id=project_id,
        handler=project_handler,
    )
    print(get_response)
    assert get_response["statusCode"] == 400
    db_client.delete(Team(team_id=team_id))


@mock_dynamodb
def test_no_team_id():

    """
    -> Attempt to create a member; Negative flow, no team id
    """

    for method in "GET", "PUT", "POST", "DELETE":
        event: dict = {
            "requestContext": {
                "http": {
                    "method": method,
                }
            },
            "queryStringParameters": {
                "children": True,
                "projectId": "TEST",
            },
            "body": dumps(
                {
                    "name": "TEST",
                    "language": "TEST",
                    "buildTool": "TEST",
                }
            ),
        }

        response: dict = project_handler(event, {})
        assert response["statusCode"] == 400


def create(team_id: str, handler):

    """
    -> Test Creating a Project
    """

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
        "body": dumps(
            {
                "name": "Initial Project Name",
                "codebases": [
                    {
                        "name": "Initial Frontend Name",
                        "language": "JAVASCRIPT",
                        "buildTool": "YARN",
                    },
                    {
                        "name": "Initial Backend Name",
                        "language": "PYTHON",
                        "buildTool": "POETRY",
                    },
                ],
            }
        ),
    }

    return handler(event, {})


def get(team_id: str, project_id: str, handler):

    """
    -> Test Getting a Project
    """

    event: dict = {
        "pathParameters": {
            "project": project_id,
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

    return handler(event, {})


def get_all(team_id: str, handler):

    """
    -> Test Getting all the Projects
    """

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

    return handler(event, {})


# pylint: disable=R0913
def update(
    team_id: str,
    project_id: str,
    codebase1_id: str,
    codebase2_id: str,
    new_cb_name1: str,
    new_cb_name2: str,
    handler,
):

    """
    -> Test Updating a Project
    """

    event: dict = {
        "pathParameters": {
            "project": project_id,
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
        "body": dumps(
            {
                "name": "Updated Project Name",
                "codebases": [
                    {"id": codebase1_id, "name": new_cb_name1},
                    {"id": codebase2_id, "name": new_cb_name2},
                ],
            }
        ),
    }

    return handler(event, {})


def delete(team_id: str, project_id: str, handler):

    """
    -> Test Deleting a Project
    """

    event: dict = {
        "pathParameters": {
            "project": project_id,
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

    return handler(event, {})
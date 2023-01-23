"""
-> Test for the codebase handlers
"""
import uuid
from json import dumps, loads
from typing import Callable

import boto3
from moto import mock_dynamodb

from cyclonedx.clients.db.dynamodb import HarborDBClient
from cyclonedx.handlers import codebase_handler, codebases_handler
from cyclonedx.model import HarborModel
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from tests.conftest import create_mock_dynamodb_infra


@mock_dynamodb
def test_create():

    """
    -> Test the creation, updating and deletion of a codebase.
    """

    dynamodb_resource = boto3.resource("dynamodb")

    db_client: HarborDBClient = HarborDBClient(dynamodb_resource=dynamodb_resource)

    create_mock_dynamodb_infra(dynamodb_resource)

    team_id: str = str(uuid.uuid4())
    project_id: str = str(uuid.uuid4())

    codebase_name: str = str(uuid.uuid4())
    language: str = "JAVA"
    build_tool: str = "MAVEN"
    clone_url: str = "https://github.com/cmsgov/ab2d-lambdas"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name="Test Project Name",
                ),
            ],
        ),
        recurse=True,
    )

    # Create
    create_response: dict = create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        clone_url=clone_url,
        handler=codebase_handler,
    )

    codebase_dict: dict = loads(create_response["body"])
    codebase_id: str = codebase_dict.get(CodeBase.Fields.ID)

    assert codebase_name == codebase_dict[CodeBase.Fields.NAME]
    assert codebase_dict[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict[CodeBase.Fields.BUILD_TOOL] == build_tool
    assert codebase_dict[CodeBase.Fields.CLONE_URL] == clone_url
    assert codebase_dict[HarborModel.Fields.ID] == codebase_id


@mock_dynamodb
def test_get():

    """
    -> Test creating and then getting a codebase
    """

    dynamodb_resource = boto3.resource("dynamodb")

    db_client: HarborDBClient = HarborDBClient(dynamodb_resource=dynamodb_resource)

    create_mock_dynamodb_infra(dynamodb_resource)

    team_id: str = str(uuid.uuid4())
    project_id: str = str(uuid.uuid4())

    codebase_name: str = str(uuid.uuid4())
    language: str = "JAVA"
    build_tool: str = "MAVEN"
    clone_url: str = "https://github.com/cmsgov/ab2d-lambdas"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name="Test Project Name",
                ),
            ],
        ),
        recurse=True,
    )

    create_response: dict = create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        clone_url=clone_url,
        handler=codebase_handler,
    )

    codebase_dict: dict = loads(create_response["body"])
    codebase_id: str = codebase_dict.get(CodeBase.Fields.ID)

    get_response: dict = get(
        team_id=team_id,
        codebase_id=codebase_id,
        handler=codebase_handler,
    )

    codebase_dict = loads(get_response["body"])

    assert codebase_name == codebase_dict[CodeBase.Fields.NAME]
    assert codebase_dict[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict[CodeBase.Fields.BUILD_TOOL] == build_tool
    assert codebase_dict[CodeBase.Fields.CLONE_URL] == clone_url
    assert codebase_dict[HarborModel.Fields.ID] == codebase_id


@mock_dynamodb
def test_get_all():

    """
    -> Test to get all codebases for a team
    """

    dynamodb_resource = boto3.resource("dynamodb")

    db_client: HarborDBClient = HarborDBClient(dynamodb_resource=dynamodb_resource)

    create_mock_dynamodb_infra(dynamodb_resource)

    team_id: str = str(uuid.uuid4())
    project_id: str = str(uuid.uuid4())

    codebase_name: str = str(uuid.uuid4())
    language: str = "JAVA"
    build_tool: str = "MAVEN"
    clone_url: str = "https://github.com/cmsgov/ab2d-lambdas"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name="Test Project Name",
                ),
            ],
        ),
        recurse=True,
    )

    create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        clone_url=clone_url,
        handler=codebase_handler,
    )

    create_response_1: dict = create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        clone_url=clone_url,
        handler=codebase_handler,
    )

    response_dict_1: dict = loads(create_response_1["body"])
    codebase_id_1: str = response_dict_1[CodeBase.Fields.ID]

    get_response: dict = get_all(
        team_id=team_id,
        handler=codebases_handler,
    )

    (codebase_dict_0, codebase_dict_1) = loads(get_response["body"])

    if codebase_dict_0[HarborModel.Fields.ID] == codebase_id_1:
        (codebase_dict_0, codebase_dict_1) = (codebase_dict_1, codebase_dict_0)

    assert codebase_dict_0[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict_0[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict_0[CodeBase.Fields.BUILD_TOOL] == build_tool
    assert codebase_dict_0[CodeBase.Fields.CLONE_URL] == clone_url

    assert codebase_dict_1[CodeBase.Fields.NAME] == codebase_name
    assert codebase_dict_1[CodeBase.Fields.LANGUAGE] == language
    assert codebase_dict_1[CodeBase.Fields.BUILD_TOOL] == build_tool
    assert codebase_dict_1[CodeBase.Fields.CLONE_URL] == clone_url


@mock_dynamodb
def test_update():

    """
    -> Special test to ensure the data in the codebases is updating
    """

    dynamodb_resource = boto3.resource("dynamodb")

    db_client: HarborDBClient = HarborDBClient(dynamodb_resource=dynamodb_resource)

    create_mock_dynamodb_infra(dynamodb_resource)

    team_id: str = str(uuid.uuid4())
    project_id: str = str(uuid.uuid4())

    codebase_name: str = str(uuid.uuid4())
    language: str = "JAVA"
    build_tool: str = "MAVEN"
    clone_url: str = "https://github.com/cmsgov/ab2d-lambdas"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name="Test Project Name",
                ),
            ],
        ),
        recurse=True,
    )

    # Create
    create_response: dict = create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        clone_url=clone_url,
        handler=codebase_handler,
    )

    response_dict: dict = loads(create_response["body"])
    codebase_id: str = response_dict[CodeBase.Fields.ID]

    # Update
    new_codebase_name: str = str(uuid.uuid4())
    new_language: str = "KOTLIN"
    new_build_tool: str = "GRADLE"
    new_clone_url: str = "https://github.com/cmsgov/xy32z-lambdas"

    update(
        team_id=team_id,
        project_id=project_id,
        codebase_id=codebase_id,
        new_name=new_codebase_name,
        new_language=new_language,
        new_build_tool=new_build_tool,
        new_clone_url=new_clone_url,
        handler=codebase_handler,
    )

    get_response: dict = get(
        team_id=team_id,
        codebase_id=codebase_id,
        handler=codebase_handler,
    )

    get_response_dict: dict = loads(get_response["body"])

    assert new_codebase_name == get_response_dict.get(CodeBase.Fields.NAME)
    assert new_language == get_response_dict.get(CodeBase.Fields.LANGUAGE)
    assert new_build_tool == get_response_dict.get(CodeBase.Fields.BUILD_TOOL)
    assert new_clone_url == get_response_dict.get(CodeBase.Fields.CLONE_URL)


@mock_dynamodb
def test_delete():

    """
    -> Test creating and then getting a codebase
    """

    dynamodb_resource = boto3.resource("dynamodb")

    db_client: HarborDBClient = HarborDBClient(dynamodb_resource=dynamodb_resource)

    create_mock_dynamodb_infra(dynamodb_resource)

    team_id: str = str(uuid.uuid4())
    project_id: str = str(uuid.uuid4())

    codebase_name: str = str(uuid.uuid4())
    language: str = "JAVA"
    build_tool: str = "MAVEN"
    clone_url: str = "https://github.com/cmsgov/ab2d-lambdas"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name="Test Project Name",
                ),
            ],
        ),
        recurse=True,
    )

    create_response: dict = create(
        team_id=team_id,
        project_id=project_id,
        name=codebase_name,
        language=language,
        build_tool=build_tool,
        clone_url=clone_url,
        handler=codebase_handler,
    )

    codebase_dict: dict = loads(create_response["body"])
    codebase_id: str = codebase_dict.get(CodeBase.Fields.ID)

    # Delete
    delete(
        team_id=team_id,
        codebase_id=codebase_id,
        handler=codebase_handler,
    )

    # Get Test, expect 400 because the codebase is not there.
    get_response: dict = get(
        team_id=team_id,
        codebase_id=codebase_id,
        handler=codebase_handler,
    )
    assert get_response["statusCode"] == 400
    db_client.delete(Team(team_id=team_id))


@mock_dynamodb
def test_no_team_id():

    """
    -> Attempt to create a codebase; Negative flow, no team id
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

        response: dict = codebase_handler(event, {})
        assert response["statusCode"] == 400


# pylint: disable=R0913
@mock_dynamodb
def test_no_project_id():

    """
    -> Create a codebase
    """

    for method in "GET", "PUT", "POST", "DELETE":
        event: dict = {
            "requestContext": {
                "http": {
                    "method": method,
                }
            },
            "queryStringParameters": {
                "teamId": "TEST",
                "children": True,
            },
            "body": dumps(
                {
                    "name": "TEST",
                    "language": "TEST",
                    "buildTool": "TEST",
                }
            ),
        }

    response: dict = codebase_handler(event, {})
    assert response["statusCode"] == 400


# pylint: disable=R0913
def create(
    team_id: str,
    project_id: str,
    name: str,
    language: str,
    build_tool: str,
    clone_url: str,
    handler,
):

    """
    -> Create a codebase
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
            "projectId": project_id,
        },
        "body": dumps(
            {
                "name": name,
                "language": language,
                "buildTool": build_tool,
                "cloneUrl": clone_url,
            }
        ),
    }

    return handler(event, {})


def get(team_id: str, codebase_id: str, handler):

    """
    -> Get a codebase
    """

    event: dict = {
        "pathParameters": {
            "codebase": codebase_id,
        },
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
    }

    return handler(event, {})


def get_all(team_id: str, handler):

    """
    -> Get all the codebases
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
    codebase_id: str,
    project_id: str,
    new_name: str,
    new_language: str,
    new_build_tool: str,
    new_clone_url: str,
    handler: Callable,
) -> dict:
    """
    -> Update a codebase's data
    """

    event: dict = {
        "pathParameters": {
            "codebase": codebase_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
            "projectId": project_id,
        },
        "body": dumps(
            {
                CodeBase.Fields.NAME: new_name,
                CodeBase.Fields.LANGUAGE: new_language,
                CodeBase.Fields.BUILD_TOOL: new_build_tool,
                CodeBase.Fields.CLONE_URL: new_clone_url,
            }
        ),
    }

    return handler(event, {})


def delete(team_id: str, codebase_id: str, handler):

    """
    -> Delete a codebase
    """

    event: dict = {
        "pathParameters": {
            "codebase": codebase_id,
        },
        "requestContext": {
            "http": {
                "method": "DELETE",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
    }

    return handler(event, {})

"""
-> Module to test the Teams handlers
"""
import os
import uuid
from json import dumps, loads
from typing import Callable

import boto3
from moto import mock_cognitoidp, mock_dynamodb

from cyclonedx.clients.ciam import CognitoUserData, HarborCognitoClient
from cyclonedx.constants import USER_POOL_ID_KEY
from cyclonedx.handlers import team_handler  # TODO Test teams_handler
from cyclonedx.handlers.common import ContextKeys
from cyclonedx.model import HarborModel
from cyclonedx.model.member import Member
from tests.conftest import create_mock_cognito_infra, create_mock_dynamodb_infra
from tests.handlers import EMAIL

project_id1 = str(uuid.uuid4())
project_id2 = str(uuid.uuid4())
member_id = str(uuid.uuid4())


@mock_dynamodb
@mock_cognitoidp
def test_create():

    """
    -> Create Team Test
    """

    cognito_idp = boto3.client("cognito-idp")
    email: str = "test@email.net"
    teams: str = "dawn-patrol,dusk-patrol"
    user_pool_id, username, _ = create_mock_cognito_infra(cognito_idp, teams, email)
    os.environ[USER_POOL_ID_KEY] = user_pool_id

    # Setup DynamoDB Mock
    create_mock_dynamodb_infra(boto3.resource("dynamodb"))

    # Create
    create_response: dict = create(team_handler, username=email)
    response_dict: dict = loads(create_response["body"])
    team_id: str = list(response_dict.keys()).pop()
    team_dict: dict = response_dict[team_id]
    members_dict: dict = team_dict["members"]
    member: dict = list(members_dict.values()).pop()
    assert member[Member.Fields.EMAIL] == EMAIL

    cognito_client: HarborCognitoClient = HarborCognitoClient()
    cognito_user_data: CognitoUserData = cognito_client.get_user_data(email)
    post_create_teams: set = set(f"{teams},{team_id}".split(","))
    actual_teams: set = set(cognito_user_data.teams.split(","))
    assert post_create_teams == actual_teams

    return team_id, team_dict, username


@mock_dynamodb
@mock_cognitoidp
def test_get():

    """
    -> Get Team Test
    """

    team_id, team_dict, username = test_create()

    projects_dict: dict = team_dict["projects"]
    projects_ids: list = list(projects_dict.keys())

    # Get Test
    get_response: dict = get(team_id, team_handler)
    response_dict = loads(get_response["body"])

    res_team_id: str = list(response_dict.keys()).pop()
    assert team_id == res_team_id

    team: dict = list(response_dict.values()).pop()
    assert team[HarborModel.Fields.ID] == team_id

    # Test to verify that a single token is also
    # created when the team is: ISPGCASP-864
    tokens: dict = team["tokens"]
    assert len(tokens.values()) == 1

    res_team_dict: dict = response_dict[res_team_id]
    res_projects_dict: dict = res_team_dict["projects"]
    res_projects_ids: list = list(res_projects_dict.keys())

    for pid in projects_ids:
        assert pid in res_projects_ids

    return team_id, res_projects_ids, username


@mock_dynamodb
@mock_cognitoidp
def test_update():

    """
    -> Update Team Test
    """

    team_id, res_projects_ids, username = test_get()

    # Update
    update_response: dict = update(
        team_id,
        res_projects_ids[0],
        res_projects_ids[1],
        team_handler,
    )
    print(dumps(loads(update_response["body"]), indent=2))

    return team_id, username


@mock_dynamodb
@mock_cognitoidp
def test_delete():

    """
    -> Delete Team Test
    """

    cognito_idp = boto3.client("cognito-idp")
    email: str = "test@email.net"
    teams: str = "dawn-patrol,dusk-patrol"
    user_pool_id, _, _ = create_mock_cognito_infra(cognito_idp, teams, email)
    os.environ[USER_POOL_ID_KEY] = user_pool_id

    # Setup DynamoDB Mock
    create_mock_dynamodb_infra(boto3.resource("dynamodb"))

    # Create
    create_response: dict = create(team_handler, username=email)
    response_dict: dict = loads(create_response["body"])
    team_id: str = list(response_dict.keys()).pop()

    # Delete
    delete(team_id, email, team_handler)

    # Get Test (Should return nothing)
    get_response: dict = get(team_id, team_handler)
    assert get_response["statusCode"] == 400


def create(handler, username: str):

    """
    -> Create a team
    """

    event: dict = {
        "requestContext": {
            "authorizer": {
                "lambda": {
                    ContextKeys.EMAIL: EMAIL,
                    ContextKeys.USERNAME: username,
                }
            },
            "http": {
                "method": "POST",
            },
        },
        "queryStringParameters": {"children": False},
        "body": dumps(
            {
                "name": "Initial Team Name",
                "projects": [
                    {
                        "name": "Initial Project Name 1",
                    },
                    {
                        "name": "Initial Project Name 2",
                    },
                ],
            }
        ),
    }

    return handler(event, {})


def get(team_id: str, handler):

    """
    -> Get a team
    """

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "children": True,
        },
    }

    return handler(event, {})


def update(team_id: str, project1_id: str, project2_id: str, handler):

    """
    -> Update a team
    """

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {"children": True},
        "body": dumps(
            {
                "name": "Updated Team Name",
                "projects": [
                    {"id": project1_id, "name": "Updated Project Name 1"},
                    {"id": project2_id, "name": "Updated Project Name 2"},
                ],
            }
        ),
    }

    return handler(event, {})


def delete(team_id: str, username: str, handler: Callable):

    """
    -> Delete a team
    """

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "requestContext": {
            "authorizer": {
                "lambda": {
                    ContextKeys.EMAIL: EMAIL,
                    ContextKeys.USERNAME: username,
                }
            },
            "http": {
                "method": "DELETE",
            },
        },
        "queryStringParameters": {
            "children": True,
        },
    }

    return handler(event, {})

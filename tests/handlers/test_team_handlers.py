import uuid
from json import dumps, loads

import boto3
import pytest

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers import team_handler

client: HarborDBClient = HarborDBClient(boto3.resource('dynamodb'))

project_id1 = str(uuid.uuid4())
project_id2 = str(uuid.uuid4())
member_id = str(uuid.uuid4())

def test_flow():

    # Create
    create_response: dict = create()
    response_dict: dict = loads(create_response['body'])
    team_id: str = list(response_dict.keys()).pop()
    team_dict: dict = response_dict[team_id]
    projects_dict: dict = team_dict['projects']
    projects_ids: list = list(projects_dict.keys())

    # Get Test
    get_response: dict = get(team_id)
    response_dict = loads(get_response['body'])

    res_team_id: str = list(response_dict.keys()).pop()
    assert team_id == res_team_id

    res_team_dict: dict = response_dict[res_team_id]
    res_projects_dict: dict = res_team_dict['projects']
    res_projects_ids: list = list(res_projects_dict.keys())

    for pid in projects_ids:
        assert pid in res_projects_ids

    # Update
    update_response: dict = update(team_id, res_projects_ids[0], res_projects_ids[1])
    print(dumps(loads(update_response['body']), indent=2))

    # Delete
    delete(team_id)

    # Get Test (Should return nothing)
    try:
        get_response: dict = get(team_id)
        print(get_response)
        pytest.fail()
    except DatabaseError:
        print("All clear.  Database is clean")

def create():

    event: dict = {
        "requestContext": {
            "http": {
                "method": "POST",
            }
        },
        "queryStringParameters": {
            "children": False
        },
        "body": dumps({
            "name": "Initial Team Name",
            "projects": [
                {
                    "name": "Initial Project Name 1"
                }, {
                    "name": "Initial Project Name 2"
                }
            ]
        }),
    }

    return team_handler(event, {})

def get(team_id: str):

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
            "children": True
        },
    }

    return team_handler(event, {})


def update(team_id: str, project1_id: str, project2_id: str):

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {
            "children": True
        },
        "body": dumps({
            "name": "Updated Team Name",
            "projects": [
                {
                    "id": project1_id,
                    "name": "Updated Project Name 1"
                }, {
                    "id": project2_id,
                    "name": "Updated Project Name 2"
                }
            ]
        }),
    }

    return team_handler(event, {})

def delete(team_id: str):

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "requestContext": {
            "http": {
                "method": "DELETE",
            }
        },
        "queryStringParameters": {
            "children": True
        },
    }

    return team_handler(event, {})

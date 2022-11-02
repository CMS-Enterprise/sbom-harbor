"""
-> Test for the token handlers
"""
import datetime
import uuid
from json import dumps, loads

import boto3
from moto import mock_dynamodb

from cyclonedx.clients.db.dynamodb import HarborDBClient

# TODO I'm testing moving this here to see
#  if the @mock_dynamodb annotation still works.
#  Pylint hates imports inside of functions, so
#  we should try leaving it here. However, if this
#  test fails, be highly suspicious of this
#  and move it back into the test function.
#  The Pylint error can be suppressed with:
#  # pylint: disable=C0415
#  over the imports inside the test function.
from cyclonedx.handlers import token_handler, tokens_handler
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token
from tests.conftest import create_mock_dynamodb_infra


@mock_dynamodb
def test_flow():

    """
    -> Test the creation, updating and deletion of a token.
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

    token_name: str = str(uuid.uuid4())

    # Create
    create_response: dict = create(
        team_id=team_id,
        name=token_name,
        handler=token_handler,
    )
    response_dict: dict = loads(create_response["body"])

    print(dumps(response_dict, indent=2))

    token_id: str = list(response_dict.keys()).pop()
    token_dict: dict = response_dict[token_id]
    assert token_name == token_dict[Token.Fields.NAME]
    assert token_dict[Token.Fields.ENABLED]
    assert token_dict[Token.Fields.CREATED]
    assert token_dict[Token.Fields.EXPIRES]
    assert token_dict[Token.Fields.TOKEN]

    # Get Test 1
    get_response: dict = get(
        team_id=team_id,
        token_id=token_id,
        handler=token_handler,
    )
    response_dict = loads(get_response["body"])
    token_dict: dict = response_dict[token_id]
    assert token_name == token_dict[Token.Fields.NAME]
    assert token_dict[Token.Fields.ENABLED]
    assert token_dict[Token.Fields.CREATED]
    assert token_dict[Token.Fields.EXPIRES]
    assert token_dict[Token.Fields.TOKEN]

    # Get Test 2
    get_response: dict = get_all(
        team_id=team_id,
        handler=tokens_handler,
    )
    response_dict = loads(get_response["body"])

    token_id: str = list(response_dict.keys()).pop()
    token_dict: dict = response_dict[token_id]
    assert token_name == token_dict[Token.Fields.NAME]
    assert token_dict[Token.Fields.ENABLED]
    assert token_dict[Token.Fields.CREATED]
    assert token_dict[Token.Fields.EXPIRES]
    assert token_dict[Token.Fields.TOKEN]

    # Update
    new_name: str = str(uuid.uuid4())
    new_expires: float = datetime.datetime.now().timestamp()

    update(
        team_id=team_id,
        token_id=token_id,
        new_name=new_name,
        expires=new_expires,
        enabled=False,
        handler=token_handler,
    )

    test_token: Token = db_client.get(
        Token(
            team_id=team_id,
            token_id=token_id,
        )
    )

    assert new_name == test_token.name
    assert not test_token.enabled

    # Delete
    delete(
        team_id=team_id,
        token_id=token_id,
        handler=token_handler,
    )

    # Get Test (Should return nothing)
    get_response: dict = get(
        team_id=team_id,
        token_id=token_id,
        handler=token_handler,
    )
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

        response: dict = token_handler(event, {})
        assert response["statusCode"] == 400


def create(team_id: str, name: str, handler):

    """
    -> Create a token
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
                "name": name,
            }
        ),
    }

    return handler(event, {})


def get(team_id: str, token_id: str, handler):

    """
    -> Get a token
    """

    event: dict = {
        "pathParameters": {
            "token": token_id,
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
    -> Get all the tokens
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
    token_id: str,
    new_name: str,
    expires: float,
    enabled: bool,
    handler,
):
    """
    -> Update a token's data
    """

    event: dict = {
        "pathParameters": {
            "token": token_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
        "body": dumps(
            {
                "name": new_name,
                "expires": expires,
                "enabled": enabled,
            }
        ),
    }

    return handler(event, {})


def delete(team_id: str, token_id: str, handler):
    """
    -> Delete a token
    """

    event: dict = {
        "pathParameters": {
            "token": token_id,
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

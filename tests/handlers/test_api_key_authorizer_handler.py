"""
-> Module for the code that tests the api_key_authorizer_handler
"""
import datetime
from uuid import uuid4

import boto3
from dateutil.relativedelta import relativedelta
from moto import mock_dynamodb

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.handlers.api_key_authorizer import api_key_authorizer_handler
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token
from tests.conftest import create_harbor_table


@mock_dynamodb
def test_good_token():

    """
    -> Test a good token that will work
    """

    team_id: str = "team-id"
    token_id: str = "token-id"
    token_name: str = "token-name"
    created = datetime.datetime.now()
    expires = created + relativedelta(weeks=1)
    token: str = str(uuid4())

    # Create Resource. It will be a mock because of the
    # annotation over the class
    resource = boto3.resource("dynamodb")

    create_harbor_table(resource)
    HarborDBClient(resource).create(
        Team(
            team_id=team_id,
            name="Test Team :-)",
            tokens=[
                Token(
                    team_id=team_id,
                    token_id=token_id,
                    name=token_name,
                    created=created.isoformat(),
                    expires=expires.isoformat(),
                    enabled=True,
                    token=token,
                )
            ],
        ),
        recurse=True,
    )

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "methodArn": "wtf-wfc",
        "authorizationToken": token,
    }

    policy: dict = api_key_authorizer_handler(event)
    policy_document: dict = policy["policyDocument"]
    statement_0: dict = policy_document["Statement"][0]
    statement_1: dict = policy_document["Statement"][1]

    assert statement_0["Effect"] == "Allow"
    assert statement_1["Effect"] == "Allow"


@mock_dynamodb
def test_expired_token():

    """
    -> Test an expired token that will fail
    """

    team_id: str = "team-id"
    token_id: str = "token-id"
    token_name: str = "token-name"
    created = datetime.datetime.now()
    expires = created - relativedelta(weeks=1)
    token: str = str(uuid4())

    # Create Resource. It will be a mock because of the
    # annotation over the class
    resource = boto3.resource("dynamodb")

    create_harbor_table(resource)
    HarborDBClient(resource).create(
        Team(
            team_id=team_id,
            name="Test Team :-)",
            tokens=[
                Token(
                    team_id=team_id,
                    token_id=token_id,
                    name=token_name,
                    created=created.isoformat(),
                    expires=expires.isoformat(),
                    enabled=True,
                    token=token,
                )
            ],
        ),
        recurse=True,
    )

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "methodArn": "wtf-wfc",
        "authorizationToken": token,
    }

    policy: dict = api_key_authorizer_handler(event)
    policy_document: dict = policy["policyDocument"]
    statement_0: dict = policy_document["Statement"][0]

    assert statement_0["Effect"] == "Deny"


@mock_dynamodb
def test_disabled_token():

    """
    -> Test a disabled token that will fail
    """

    team_id: str = "team-id"
    token_id: str = "token-id"
    token_name: str = "token-name"
    created = datetime.datetime.now()
    expires = created + relativedelta(weeks=1)
    token: str = str(uuid4())

    # Create Resource. It will be a mock because of the
    # annotation over the class
    resource = boto3.resource("dynamodb")

    create_harbor_table(resource)
    HarborDBClient(resource).create(
        Team(
            team_id=team_id,
            name="Test Team :-)",
            tokens=[
                Token(
                    team_id=team_id,
                    token_id=token_id,
                    name=token_name,
                    created=created.isoformat(),
                    expires=expires.isoformat(),
                    enabled=False,
                    token=token,
                )
            ],
        ),
        recurse=True,
    )

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "methodArn": "wtf-wfc",
        "authorizationToken": token,
    }

    policy: dict = api_key_authorizer_handler(event)
    policy_document: dict = policy["policyDocument"]
    statement_0: dict = policy_document["Statement"][0]

    assert statement_0["Effect"] == "Deny"


@mock_dynamodb
def test_missing_token():

    """
    -> Test an missing token that will fail
    """

    team_id: str = "team-id"
    token: str = str(uuid4())

    # Create Resource. It will be a mock because of the
    # annotation over the class
    resource = boto3.resource("dynamodb")

    create_harbor_table(resource)
    HarborDBClient(resource).create(
        Team(
            team_id=team_id,
            name="Test Team :-)",
        ),
    )

    event: dict = {
        "pathParameters": {
            "team": team_id,
        },
        "methodArn": "wtf-wfc",
        "authorizationToken": token,
    }

    policy: dict = api_key_authorizer_handler(event)
    policy_document: dict = policy["policyDocument"]
    statement_0: dict = policy_document["Statement"][0]

    assert statement_0["Effect"] == "Deny"

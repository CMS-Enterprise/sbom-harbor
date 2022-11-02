"""
-> Module to house the tests for the User Search Handler
"""
from json import loads
from os import environ

import boto3
from moto import mock_cognitoidp

from cyclonedx.constants import USER_POOL_ID_KEY
from cyclonedx.handlers import user_search_handler
from cyclonedx.handlers.common import QueryStringKeys
from tests.conftest import create_mock_cognito_infra, create_users_in_cognito


@mock_cognitoidp
def test_user_search():

    """
    -> Tests searching Cognito for matching email addresses
    """

    team_id: str = "test-team-id"
    username_email: str = "user@name.net"

    cognito_client = boto3.client("cognito-idp")

    user_pool_id, _, _ = create_mock_cognito_infra(
        teams=team_id,
        email_username=username_email,
        cognito_client=cognito_client,
    )

    emails: list[str] = [
        "bill@red.net",
        "abener@red.net",
        "chil@red.net",
        "abe@red.net",
        "phil@red.net",
    ]

    create_users_in_cognito(
        cognito_client,
        user_pool_id,
        emails,
        team_id,
    )

    environ[USER_POOL_ID_KEY] = user_pool_id

    mock_event: dict = {
        "queryStringParameters": {
            QueryStringKeys.FILTER: "abe",
        }
    }

    response: dict = user_search_handler(mock_event)
    matching_users: list[str] = loads(response["body"])

    assert len(matching_users) == 2
    assert "abener@red.net" in matching_users
    assert "abe@red.net" in matching_users

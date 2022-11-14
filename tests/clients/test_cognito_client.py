"""
-> Module to house the tests for HarborCognitoClient
"""
from os import environ

import boto3
from moto import mock_cognitoidp

from cyclonedx.clients.ciam import CognitoUserData, HarborCognitoClient, JwtData
from cyclonedx.constants import USER_POOL_CLIENT_ID_KEY, USER_POOL_ID_KEY
from tests.conftest import (
    FINAL_TEST_PASSWORD,
    create_mock_cognito_infra,
    create_users_in_cognito,
)


@mock_cognitoidp
def test_get_jwt_data():

    """
    -> Test to make sure we can extract attributes from a JWT
    """

    cognito_client = boto3.client("cognito-idp")
    username: str = "user@name.net"
    user_pool_id, _, _ = create_mock_cognito_infra(
        teams="",
        email_username=username,
        cognito_client=cognito_client,
    )
    environ[USER_POOL_ID_KEY] = user_pool_id

    # pylint: disable=C0301
    jwt_p1: str = "eyJraWQiOiJmYUdKYWR4NDB1UmxLMExyd2ZXQklObjhuaTRWdGZTYzc0ODJ2MmpRQWJFPSIsImFsZyI6IlJTMjU2In0"
    jwt_p2: str = "eyJzdWIiOiI2YzNiMjc3MC0yZjc1LTQ4MTMtYTk2OC00MmY1MzcyMzJlNzMiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV9Lb1BsVzZ5Y2oiLCJjbGllbnRfaWQiOiIzcnVlNDZmamZ1ZmU4OTdoZG05ODNwM3FzNiIsIm9yaWdpbl9qdGkiOiI1OGJiNzQ5Yy0wY2MzLTQ5YjgtOGIwMS1iMzE5NGE1Y2U0ZDUiLCJldmVudF9pZCI6ImUwMmVkNmIzLWVkY2EtNGMzZi05Y2M4LTI2ZjI3OWYxN2VjYiIsInRva2VuX3VzZSI6ImFjY2VzcyIsInNjb3BlIjoiYXdzLmNvZ25pdG8uc2lnbmluLnVzZXIuYWRtaW4iLCJhdXRoX3RpbWUiOjE2NjY4ODY3MjYsImV4cCI6MTY2Njg5MDMyNiwiaWF0IjoxNjY2ODg2NzI2LCJqdGkiOiI4NGI2MDBjNi1lMjUxLTRjOTItYTE5ZS1lN2ZjNDdkZjhjYjEiLCJ1c2VybmFtZSI6IjZjM2IyNzcwLTJmNzUtNDgxMy1hOTY4LTQyZjUzNzIzMmU3MyJ9"
    jwt_p3: str = "dKTbAyMVr8Fxk2nSHFd3qQ_-r3OJKl4R01ofgd1IcVllyIPJQVnxdkMgQxULFOuePg56eoradI_azFM6XPwOPrDGT2qh__zKzOPv1PImX9DW3NYkE5IK9-oK0GG3-rlnkp79CYlNb8qVlS3OAtEf_KeVgDCkVHzHQqr4uS_5tj3KkA_mABjbzGjCWAl0EKJrNsFdDmaCV04M2n7-ivlCPrNooO0TLHqWi9BYv9fyOUS-rzqekuvIQi5F7ckGaYiMu2DK-uVU_Tg-DThCP3NmgYLYLHaKBZGkdtXzFyKv1nckgHEGCMK1YGgue9g99A4AMp7edReneY0vM58b2iuu-A"
    test_token: str = f"{jwt_p1}.{jwt_p2}.{jwt_p3}"
    username: str = "6c3b2770-2f75-4813-a968-42f537232e73"

    token_data: JwtData = HarborCognitoClient().get_jwt_data(test_token)
    assert token_data.token == test_token
    assert token_data.username == username


@mock_cognitoidp
def test_get_jwt():

    """
    -> Test to make sure we can get a JWT from Cognito
    """

    team_id: str = "test-team-id"
    username_email: str = "user@name.net"

    cognito_client = boto3.client("cognito-idp")

    user_pool_id, user_pool_client_id, _ = create_mock_cognito_infra(
        teams=team_id,
        email_username=username_email,
        cognito_client=cognito_client,
    )
    environ[USER_POOL_ID_KEY] = user_pool_id
    environ[USER_POOL_CLIENT_ID_KEY] = user_pool_client_id

    hcc: HarborCognitoClient = HarborCognitoClient()

    jwt: str = hcc.get_jwt(
        username=username_email,
        password=FINAL_TEST_PASSWORD,
    )

    assert jwt


@mock_cognitoidp
def test_add_team_to_member():

    """
    -> Test to add a team to a member
    """

    team_id: str = "test-team-id"
    username: str = "user@name.net"

    cognito_client = boto3.client("cognito-idp")

    user_pool_id, _, _ = create_mock_cognito_infra(
        teams="",
        email_username=username,
        cognito_client=cognito_client,
    )
    environ[USER_POOL_ID_KEY] = user_pool_id

    hcc: HarborCognitoClient = HarborCognitoClient()

    hcc.add_team_to_member(
        team_id=team_id,
        cognito_username=username,
    )

    cud: CognitoUserData = hcc.get_user_data(username)
    assert team_id == cud.teams


@mock_cognitoidp
def test_remove_team_from_member():

    """
    -> Test to make sure we can remove a team from a member
    """

    team_id: str = "test-team-id"
    username: str = "user@name.net"

    cognito_client = boto3.client("cognito-idp")

    user_pool_id, _, _ = create_mock_cognito_infra(
        teams=team_id,
        email_username=username,
        cognito_client=cognito_client,
    )
    environ[USER_POOL_ID_KEY] = user_pool_id

    hcc: HarborCognitoClient = HarborCognitoClient()

    hcc.remove_team_from_member(
        team_id=team_id,
        cognito_username=username,
    )

    user_data: CognitoUserData = hcc.get_user_data(username)
    assert user_data.teams == ""


@mock_cognitoidp
def test_get_user_data():

    """
    -> Test to verify we can get data from a user in Cognito
    """

    team_id: str = "test-team-id"
    username_email: str = "user@name.net"

    cognito_client = boto3.client("cognito-idp")

    user_pool_id, _, _ = create_mock_cognito_infra(
        teams=team_id,
        email_username=username_email,
        cognito_client=cognito_client,
    )
    environ[USER_POOL_ID_KEY] = user_pool_id
    hcc: HarborCognitoClient = HarborCognitoClient()

    user_data: CognitoUserData = hcc.get_user_data(username_email)
    assert user_data.teams == team_id
    assert user_data.email == username_email


@mock_cognitoidp
def test_get_matching_users():

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
    hcc: HarborCognitoClient = HarborCognitoClient()

    matching_users: list[str] = hcc.get_matching_users("abe")

    assert len(matching_users) == 2
    assert "abener@red.net" in matching_users
    assert "abe@red.net" in matching_users

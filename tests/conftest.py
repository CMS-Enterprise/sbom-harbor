"""
-> The Conftest module is the location we put our Pytest Fixtures
-> and any other test related startup code.
"""

import os

import boto3
import pytest
from moto import mock_dynamodb, mock_s3

from cyclonedx.clients.ciam import CognitoUserData
from cyclonedx.clients.db.dynamodb import HarborDBClient
from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_NAME,
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)
from cyclonedx.handlers.ingress import sbom_ingress_handler

# This file is where all pytest fixtures should be
# placed to make sure they are available for all tests to use
# If this file becomes too large we may want to
# instead break it up into smaller files

test_bucket_name = "sbom.bucket.test"

FINAL_TEST_PASSWORD = "L0g1nTe5tP@55!"


@pytest.fixture(autouse=True)
def test_data_path():
    """use this fixture to properly load test files"""
    return os.path.dirname(os.path.abspath(__file__))


@pytest.fixture(autouse=True)
def aws_credentials():
    """mocked AWS Credentials for moto. This is set to auto use so all other fixtures can use it"""
    os.environ["AWS_ACCESS_KEY_ID"] = "testing"
    os.environ["AWS_SECRET_ACCESS_KEY"] = "testing"
    os.environ["AWS_SECURITY_TOKEN"] = "testing"
    os.environ["AWS_SESSION_TOKEN"] = "testing"
    os.environ["sbom_bucket"] = test_bucket_name


@pytest.fixture(name="s3_test_obj")
def fixture_s3_test_obj():
    """creates and makes available a test bucket"""
    with mock_s3():
        s3 = boto3.resource("s3")
        s3.create_bucket(Bucket=test_bucket_name)
        yield s3


@pytest.fixture(name="s3_test_bucket")
def fixture_s3_test_bucket(s3_test_obj):
    """handy fixture for just using the bucket"""
    return s3_test_obj.Bucket(test_bucket_name)


@pytest.fixture
def upload_to_test_bucket(s3_test_bucket):
    """upload a file to the test bucket with a file path and the key to save the file as"""

    def file_and_key(file, key):
        return s3_test_bucket.upload_file(
            Filename=file,
            Key=key,
        )

    return file_and_key


@pytest.fixture
def upload_to_ingress():
    """uploads a file to test bucket by going through the ingress handler"""

    def file_to_upload(file):
        wrapped_sbom = {
            "pathParameters": {
                "team": "testTeam",
                "project": "TestProject",
                "codebase": "TestCodebase",
            },
            "body": file,
        }

        return sbom_ingress_handler(wrapped_sbom)

    return file_to_upload


@pytest.fixture(name="test_dynamo_db_resource")
def fixture_test_dynamo_db_resource():
    """returns a dynamodb resource object"""
    with mock_dynamodb():
        dynamodb_resource = boto3.resource("dynamodb")
        yield dynamodb_resource


@pytest.fixture
def test_harbor_db_client(test_dynamo_db_resource):
    """creates and returns a stubbed HarborDBClient"""
    return HarborDBClient(test_dynamo_db_resource)


@pytest.fixture
def test_harbor_teams_table(test_dynamo_db_resource):

    """
    -> creates a teams table insided the mocked
    -> dynamodb and returns a pointer to the table object
    """

    yield create_mock_dynamodb_infra(test_dynamo_db_resource)


def create_mock_dynamodb_infra(dynamodb_resource):

    """
    -> A function to create the harbor table
    """

    return dynamodb_resource.create_table(
        TableName=HARBOR_TEAMS_TABLE_NAME,
        AttributeDefinitions=[
            {
                "AttributeName": HARBOR_TEAMS_TABLE_PARTITION_KEY,
                "AttributeType": "S",
            },
            {
                "AttributeName": HARBOR_TEAMS_TABLE_SORT_KEY,
                "AttributeType": "S",
            },
        ],
        KeySchema=[
            {
                "AttributeName": HARBOR_TEAMS_TABLE_PARTITION_KEY,
                "KeyType": "HASH",
            },
            {
                "AttributeName": HARBOR_TEAMS_TABLE_SORT_KEY,
                "KeyType": "RANGE",
            },
        ],
        ProvisionedThroughput={
            "ReadCapacityUnits": 1,
            "WriteCapacityUnits": 1,
        },
    )


def create_mock_cognito_infra(
    cognito_client,
    teams: str = "dawn-patrol,dusk-patrol",
    email_username: str = "test@email.net",
) -> [str, str]:

    """
    -> Using Moto, we set up the mock Cognito infrastructure
    """

    create_user_pool_result = cognito_client.create_user_pool(
        PoolName="Test Cognito Pool",
        Schema=[
            {
                "Name": "email",
                "AttributeDataType": "String",
                "Required": True,
                "Mutable": True,
            }
        ],
    )
    user_pool_id: str = create_user_pool_result["UserPool"]["Id"]

    user_pool_client_name: str = "test_user_pool_client"
    create_user_pool_result = cognito_client.create_user_pool_client(
        UserPoolId=user_pool_id,
        ClientName=user_pool_client_name,
    )
    user_pool_client_data: dict = create_user_pool_result["UserPoolClient"]
    user_pool_client_id: str = user_pool_client_data["ClientId"]

    cognito_client.add_custom_attributes(
        UserPoolId=user_pool_id,
        CustomAttributes=[
            {
                "Name": "teams",
                "AttributeDataType": "String",
                "DeveloperOnlyAttribute": False,
                "Mutable": True,
                "Required": False,
            },
        ],
    )

    create_users_in_cognito(
        cognito_client,
        user_pool_id,
        [email_username],
        teams,
    )

    return user_pool_id, user_pool_client_id, email_username


def create_users_in_cognito(
    cognito_client,
    user_pool_id: str,
    email_usernames: list[str],
    teams: str,
):

    """
    -> Create mock users in Cognito
    """

    def do_create(email_username: str) -> None:

        """
        -> Inner function to keep the code out of the list comprehension
        """

        cognito_client.admin_create_user(
            UserPoolId=user_pool_id,
            Username=email_username,
            UserAttributes=[
                {
                    "Name": CognitoUserData.Attrib.EMAIL,
                    "Value": email_username,
                },
                {
                    "Name": CognitoUserData.Attrib.TEAMS,
                    "Value": teams,
                },
            ],
            TemporaryPassword="AbC123P@55!",
            ForceAliasCreation=True,
            MessageAction="SUPPRESS",
            DesiredDeliveryMediums=["EMAIL"],
        )

        cognito_client.admin_set_user_password(
            UserPoolId=user_pool_id,
            Username=email_username,
            Password=FINAL_TEST_PASSWORD,
            Permanent=True,
        )

    # pylint: disable=W0106
    [do_create(email_username) for email_username in email_usernames]

import os

import boto3
import pytest
from moto import (
    mock_s3,
    mock_dynamodb
)

from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_NAME,
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY
)

from cyclonedx.handlers.ingress import sbom_ingress_handler
from cyclonedx.db.harbor_db_client import HarborDBClient

# This file is where all pytest fixtures should be placed to make sure they are available for all tests to use
# If this file becomes too large we may want to instead break it up into smaller files

test_bucket_name = "sbom.bucket.test"


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


@pytest.fixture
def s3_test_obj():
    """creates and makes available a test bucket"""
    with mock_s3():
        s3 = boto3.resource('s3')
        s3.create_bucket(Bucket=test_bucket_name)
        yield s3


@pytest.fixture
def s3_test_bucket(s3_test_obj):
    """handy fixture for just using the bucket"""
    return s3_test_obj.Bucket(test_bucket_name)


@pytest.fixture
def upload_to_test_bucket(s3_test_bucket):
    """upload a file to the test bucket with a file path and the key to save the file as"""
    def file_and_key(file, key):
        return s3_test_bucket.upload_file(Filename=file, Key=key)
    return file_and_key


@pytest.fixture
def upload_to_ingress():
    """uploads a file to test bucket by going through the ingress handler"""
    def file_to_upload(file):
        wrapped_sbom = {
            "pathParameters": {
                "team": "testTeam",
                "project": "TestProject",
                "codebase": "TestCodebase"
            },
            "body": file.read()
        }

        return sbom_ingress_handler(wrapped_sbom)
    return file_to_upload


@pytest.fixture
def test_dynamo_db_resource():
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
    """creates a teams table insided the mocked dynamodb and returns a pointer to the table ojbect"""
    teams_table = test_dynamo_db_resource.create_table(
        TableName=HARBOR_TEAMS_TABLE_NAME,
        AttributeDefinitions=[
            {
                "AttributeName": HARBOR_TEAMS_TABLE_PARTITION_KEY,
                "AttributeType": "S"
            },
            {
                "AttributeName": HARBOR_TEAMS_TABLE_SORT_KEY,
                "AttributeType": "S"
            }
        ],
        KeySchema=[
            {
                "AttributeName": HARBOR_TEAMS_TABLE_PARTITION_KEY,
                "KeyType": "HASH"
            },
            {
                "AttributeName": HARBOR_TEAMS_TABLE_SORT_KEY,
                "KeyType": "RANGE"
            }
        ],
        ProvisionedThroughput={
            "ReadCapacityUnits": 1,
            "WriteCapacityUnits": 1
        }
    )
    yield teams_table

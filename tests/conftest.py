import os
import pytest
from boto3 import client
from moto import mock_s3

# This file is where all pytest fixtures should be placed to make sure they are available for all tests to use
# If this file becomes too large we may want to instead break it up into smaller files


@pytest.fixture(autouse=True)
def aws_credentials():
    """Mocked AWS Credentials for moto. This is set to auto use so all other fixtures can use it"""
    os.environ["AWS_ACCESS_KEY_ID"] = "testing"
    os.environ["AWS_SECRET_ACCESS_KEY"] = "testing"
    os.environ["AWS_SECURITY_TOKEN"] = "testing"
    os.environ["AWS_SESSION_TOKEN"] = "testing"


@pytest.fixture
def test_s3_bucket(test_bucket_name):
    """creates and makes available a test bucket"""
    with mock_s3():
        conn = client("s3", region_name="us-east-1")
        conn.create_bucket(Bucket=test_bucket_name)
        yield conn


@pytest.fixture
def test_bucket_name():
    """convenience method to get the test buckets name"""
    return "sbom.bucket.test"

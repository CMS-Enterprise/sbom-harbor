"""
-> Module to house the common test properties
"""
import json
from uuid import uuid4
from cyclonedx.handlers.common import (
    _wildcardize,
    allow_policy,
    deny_policy,
    harbor_response,
)
from tests.handlers import (
    EMAIL,
    METHOD_ARN,
    TEAMS,
)


def test_wildcardize():

    """
    -> Tests _wildcardize()
    """

    resultant_arn: str = "arn:aws:execute-api:us-east-1:531175407938:hvi0slqerb/*/*"

    keep: str = "arn:aws:execute-api:us-east-1:531175407938:hvi0slqerb"
    toss: str = "/$default/GET/api/v1/project/b3a96b74-4125-40a3-9074-0d50bd1d3ed5"
    example_arn: str = f"{keep}{toss}"
    wildcardized_arn: str = _wildcardize(example_arn)
    assert resultant_arn == wildcardized_arn


def test_allow_policy():

    """
    -> Tests allow_policy()
    """

    policy: dict = allow_policy(METHOD_ARN, EMAIL, TEAMS)

    assert policy["context"]["teams"] == TEAMS
    assert policy["context"]["user_email"] == EMAIL
    assert policy["policyDocument"]["Statement"][0]["Resource"] == _wildcardize(
        METHOD_ARN
    )
    assert policy["policyDocument"]["Statement"][1]["Resource"] == _wildcardize(
        METHOD_ARN
    )


def test_deny_policy():

    """
    -> Tests deny_policy()
    """

    policy: dict = deny_policy()
    assert policy["policyDocument"]["Statement"][0]["Effect"] == "Deny"


def test_harbor_response():

    """
    -> Tests harbor_response() to validate the shape of the output object
    """

    # expected response params
    status_code = 200
    body: dict = {"id": f"{uuid4()}"}
    headers: dict = {"Content-Type": "application/json"}

    # generate a response object
    response = harbor_response(
        status_code,
        body,
    )

    # ensure all required params are included
    assert response == {
        "statusCode": status_code,
        "headers": headers,
        "isBase64Encoded": False,
        "body": json.dumps(body),
    }

"""
-> Module to house the common test properties
"""
from cyclonedx.handlers.common import (
    _wildcardize,
    allow_policy,
    deny_policy,
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

"""
-> Module to test the JWT Authorizer
"""
import copy
import pytest
from moto import mock_cognitoidp

from cyclonedx.handlers.common import (
    _wildcardize,
    allow_policy,
    deny_policy,
)
from cyclonedx.handlers.jwt_authorizer_handler import (
    _get_arn_token_username,
    _get_cognito_user_pool_id,
    _get_teams,
    _get_user,
    jwt_authorizer_handler,
)

# pylint: disable=C0301
token_part_1: str = "eyJraWQiOiJmYUdKYWR4NDB1UmxLMExyd2ZXQklObjhuaTRWdGZTYzc0ODJ2MmpRQWJFPSIsImFsZyI6IlJTMjU2In0"
token_part_2: str = "eyJzdWIiOiJkYTdlYjRjZS0xYTM5LTRiNWEtOTU3Mi0zNmJkMzI5YzRjODgiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV9Lb1BsVzZ5Y2oiLCJjbGllbnRfaWQiOiIzcnVlNDZmamZ1ZmU4OTdoZG05ODNwM3FzNiIsIm9yaWdpbl9qdGkiOiJiM2Y2NDA4Yy04MTA5LTQyMzktYmQwZi1jMTVlZTVlY2IxOGQiLCJldmVudF9pZCI6ImYyMWM4NDMyLTQ1MTAtNDA4MS1iNmU1LWNiMmYzNWYxNDljMCIsInRva2VuX3VzZSI6ImFjY2VzcyIsInNjb3BlIjoiYXdzLmNvZ25pdG8uc2lnbmluLnVzZXIuYWRtaW4iLCJhdXRoX3RpbWUiOjE2NjYwOTQzMzIsImV4cCI6MTY2NjA5NzkzMiwiaWF0IjoxNjY2MDk0MzMyLCJqdGkiOiI2MTQ5OWJiNi1mM2FmLTRlNDEtYWFiYy1kNjc1OGU4N2M0YzYiLCJ1c2VybmFtZSI6ImRhN2ViNGNlLTFhMzktNGI1YS05NTcyLTM2YmQzMjljNGM4OCJ9"
token_part_3: str = "fFertjqRXGrD8tzcfpHDSd1oMJbzLfN0193Q7DAnsJ27EfOqUWQmmUh7Op-vwhvvjRybDEjmCZUIMA2TJQ88FYL8d2ju9FjNk-COoqd070uPCWDBY4vA6qcHo7f6WaW1Xh4A7HQLhKHrp4RitvbBEHhhmzdK7yJlaoJlvs5EjqQnB1laibaBbHWacCO_4WF08Lzh7_DdC-dvPy_IeE-3xbzm30lpxHtX5d3JEGMjXmAvJmmUyf0BDMh0WTOww-ZRkGcpituMZY2Hl-EGUIEF2vdJvM1kJcsEaKtryofqoVe4IT9V2vYY4WNVfQ-_nP8ALr5sLwxlgSlpoUZT52ye4g"
TOKEN: str = f"{token_part_1}.{token_part_2}.{token_part_3}"
USERNAME: str = "da7eb4ce-1a39-4b5a-9572-36bd329c4c88"
METHOD_ARN: str = "arn:aws:execute-api:us-east-1:531175407938:hvi0slqerb/$default/GET/api/v1/project/0dba7774-58e0-4d4e-ac5a-1f2b71b22bc5"
TEAMS: str = "dawn-patrol,dusk-patrol"
USER_POOL_ID: str = "us-east-1_KoPlW6ycj"
aws_lambda_test_event: dict = {
    "version": "1.0",
    "type": "REQUEST",
    "methodArn": METHOD_ARN,
    "identitySource": TOKEN,
    "authorizationToken": TOKEN,
    "resource": METHOD_ARN,
    "path": "/api/v1/project/0dba7774-58e0-4d4e-ac5a-1f2b71b22bc5",
    "httpMethod": "GET",
    "headers": {
        "Content-Length": "0",
        "Host": "hvi0slqerb.execute-api.us-east-1.amazonaws.com",
        "User-Agent": "Amazon CloudFront",
        "X-Amz-Cf-Id": "t86h4FiSbK1_OwNNBDNb5YG1b2GY_IF84RHxMPQA0LY5h-w51VUtpA==",
        "X-Amzn-Trace-Id": "Root=1-634e94fc-358c663b7dbfa6bb281902eb",
        "X-Forwarded-For": "108.3.156.227, 64.252.69.199",
        "X-Forwarded-Port": "443",
        "X-Forwarded-Proto": "https",
        "accept-encoding": "gzip",
        "authorization": TOKEN,
        "via": "1.1 4e2a7874b5959279490dd3b94b18a312.cloudfront.net (CloudFront)",
    },
    "queryStringParameters": {
        "teamId": "f96ef074-e8ac-4e80-bcb1-54937bc50e16",
    },
    "requestContext": {
        "accountId": "531175407938",
        "apiId": "hvi0slqerb",
        "domainName": "hvi0slqerb.execute-api.us-east-1.amazonaws.com",
        "domainPrefix": "hvi0slqerb",
        "extendedRequestId": "aMw3bjM0IAMEP2Q=",
        "httpMethod": "GET",
        "identity": {
            "accessKey": None,
            "accountId": None,
            "caller": None,
            "cognitoAmr": None,
            "cognitoAuthenticationProvider": None,
            "cognitoAuthenticationType": None,
            "cognitoIdentityId": None,
            "cognitoIdentityPoolId": None,
            "principalOrgId": None,
            "sourceIp": "108.3.156.227",
            "user": None,
            "userAgent": "Amazon CloudFront",
            "userArn": None,
        },
        "path": "/api/v1/project/0dba7774-58e0-4d4e-ac5a-1f2b71b22bc5",
        "protocol": "HTTP/1.1",
        "requestId": "aMw3bjM0IAMEP2Q=",
        "requestTime": "18/Oct/2022:11:58:52 +0000",
        "requestTimeEpoch": 1666094332145,
        "resourceId": "GET /api/v1/project/{project}",
        "resourcePath": "/api/v1/project/{project}",
        "stage": "$default",
    },
    "pathParameters": {"project": "0dba7774-58e0-4d4e-ac5a-1f2b71b22bc5"},
    "stageVariables": {},
}

test_cognito_response: dict = {
    "Username": "da7eb4ce-1a39-4b5a-9572-36bd329c4c88",
    "UserAttributes": [
        {
            "Name": "custom:teams",
            "Value": TEAMS,
        },
        {"Name": "sub", "Value": "da7eb4ce-1a39-4b5a-9572-36bd329c4c88"},
        {"Name": "email", "Value": "sbomadmin@aquia.io"},
    ],
    "UserCreateDate": "DATE",
    "UserLastModifiedDate": "DATE",
    "Enabled": True,
    "UserStatus": "CONFIRMED",
    "ResponseMetadata": {
        "RequestId": "5b483d38-5feb-490e-9f9d-7b4804366ff4",
        "HTTPStatusCode": 200,
        "HTTPHeaders": {
            "date": "Wed, 19 Oct 2022 07:23:27 GMT",
            "content-type": "application/x-amz-json-1.1",
            "content-length": "349",
            "connection": "keep-alive",
            "x-amzn-requestid": "5b483d38-5feb-490e-9f9d-7b4804366ff4",
        },
        "RetryAttempts": 0,
    },
}


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


def test_get_policy():

    """
    -> _get_policy() isn't testable until it is implemented.
    """

    ...


def test_get_teams():

    """
    -> Tests _get_teams()
    """

    assert _get_teams(test_cognito_response) == TEAMS


def test_get_teams_no_teams_attrib():

    """
    -> Tests _get_teams()
    """

    # Create a new response so we don't mess with the
    # one defined for all the tests
    new_response: dict = copy.deepcopy(test_cognito_response)

    # Get rid of the UserAttributes attribute
    del new_response["UserAttributes"]

    assert _get_teams(new_response) == ""


def test_get_teams_no_teams_in_attrib():

    """
    -> Tests _get_teams()
    """

    # Create a new response so we don't mess with the
    # one defined for all the tests
    new_response: dict = copy.deepcopy(test_cognito_response)

    # Get rid of the teams in the new dictionary
    new_response["UserAttributes"][0]["Value"] = ""

    assert _get_teams(new_response) == ""


def test_get_cognito_user_pool_id():

    """
    -> Tests _get_cognito_user_pool_id()
    """

    cupid: str = _get_cognito_user_pool_id(aws_lambda_test_event)
    assert USER_POOL_ID == cupid


def test_get_arn_token_username():

    """
    -> Tests _get_arn_token_username()
    """

    (arn, token, username) = _get_arn_token_username(aws_lambda_test_event)

    assert METHOD_ARN == arn
    assert TOKEN == token
    assert USERNAME == username


def test_get_user():

    """
    -> Tests _get_user()
    """

    class MockCognitoClient:

        """
        -> Using a tiny mock class rather than Moto
        -> because no setup is necessary
        """

        # pylint: disable=R0201
        def admin_get_user(self, UserPoolId, Username):

            """
            -> Mock Cognito IDP admin_get_user() method
            """

            assert USER_POOL_ID == UserPoolId
            assert USERNAME == Username
            return test_cognito_response

    response: dict = _get_user(USERNAME, aws_lambda_test_event, MockCognitoClient())
    assert test_cognito_response == response


def test_verify_token():

    """
    -> _verify_token() isn't testable until it is implemented.
    """

    ...


def test_allow_policy():

    """
    -> Tests allow_policy()
    """

    policy: dict = allow_policy(METHOD_ARN, TEAMS)

    assert policy["context"]["teams"] == TEAMS
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


@mock_cognitoidp
def test_jwt_authorizer_handler():

    """
    -> Tests the jwt_authorizer_handler
    -> Currently, we can only determine that it is indeed a policy document
    -> We need to improve these tests once we have _verify_token() implemented
    """

    policy: dict = jwt_authorizer_handler(aws_lambda_test_event, {})

    try:
        policy["policyDocument"]
    except KeyError:
        pytest.fail()

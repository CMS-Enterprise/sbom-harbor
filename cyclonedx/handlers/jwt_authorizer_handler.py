"""
-> Module to house the JWT Custom Authorizer
"""
from botocore.exceptions import ClientError
from jose import jwt

from cyclonedx.clients.ciam import CognitoUserData, HarborCognitoClient
from cyclonedx.clients.ciam.jwt_data import JwtData
from cyclonedx.handlers.common import allow_policy, deny_policy, print_values


def _get_policy(method_arn: str, email: str, teams: str, username: str, token: str):

    """
    -> Get the policy that we must return for access or denial
    """

    ap: dict = allow_policy(method_arn, email, teams, username)
    dp: dict = deny_policy()
    token_verified: bool = _verify_token(token)
    return ap if token_verified else dp


def _get_cognito_user_pool_id(event: dict):

    """
    -> Extracts the Cognito Pool ID from the JWT
    """

    token: str = event["authorizationToken"]
    claims = jwt.get_unverified_claims(token)
    iss: str = claims["iss"]
    return iss.rsplit("/", 1)[-1]


def _verify_token(token: str):

    """
    -> TODO Implement this function: https://jiraent.cms.gov/browse/ISPGCASP-393
    -> In the future, this function will determine if a given user
    -> has the permissions to access the specified resource
    """

    return True


def jwt_authorizer_handler(event, context):

    """
    -> JWT Custom Authorizer Handler: Uses a JWT to determine
    -> if a given user has assess to a specific resource
    """

    print_values(event, context)

    cognito_client: HarborCognitoClient = HarborCognitoClient()

    try:

        # Get the JWT token from the event
        token: str = event["authorizationToken"]
        method_arn: str = event["methodArn"]

        jwt_data: JwtData = HarborCognitoClient.get_jwt_data(token)
        token: str = jwt_data.token
        username = jwt_data.username

        user_data: CognitoUserData = cognito_client.get_user_data(username)

        return _get_policy(
            method_arn=method_arn,
            email=user_data.email,
            teams=user_data.teams,
            token=token,
            username=username,
        )
    except ClientError:
        return deny_policy()

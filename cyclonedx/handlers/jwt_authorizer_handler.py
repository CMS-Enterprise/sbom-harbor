"""
-> Module to house the JWT Custom Authorizer
"""
from typing import Callable

import boto3
from botocore.exceptions import ClientError
from jose import jwt

from cyclonedx.handlers.common import (
    allow_policy,
    deny_policy,
    print_values,
)


def _get_policy(method_arn: str, teams: str, token: str):

    """
    -> Get the policy that we must return for access or denial
    """

    ap: dict = allow_policy(method_arn, teams)
    dp: dict = deny_policy()
    token_verified: bool = _verify_token(token)
    return ap if token_verified else dp


def _get_teams(response: dict) -> str:

    """
    -> Extracts the teams from the Cognito User Query Response
    """

    try:
        user_attrib = response["UserAttributes"]

        filter_lambda: Callable = lambda o: o["Name"] == "custom:teams"
        teams_filter: filter = filter(filter_lambda, user_attrib)
        teams_attr: list = list(teams_filter)

        teams: str = ""
        if len(teams_attr) > 0:
            teams_attr_value: dict = teams_attr[0]
            teams: str = teams_attr_value["Value"]

        return teams
    except KeyError as ke:
        print(f"KeyError while getting teams: {ke}")
        return ""


def _get_cognito_user_pool_id(event: dict):

    """
    -> Extracts the Cognito Pool ID from the JWT
    """

    token: str = event["authorizationToken"]
    claims = jwt.get_unverified_claims(token)
    iss: str = claims["iss"]
    return iss.rsplit("/", 1)[-1]


def _get_arn_token_username(event: dict):

    """
    -> Gets the Function ARN, the token and the username
    -> from the event
    """

    method_arn: str = event["methodArn"]
    token: str = event["authorizationToken"]
    claims = jwt.get_unverified_claims(token)
    username = claims["username"]
    return method_arn, token, username


def _get_user(username: str, event: dict, client):

    """
    -> Gets the Cognito user based on their username
    """

    cognito_user_pool_id = _get_cognito_user_pool_id(event)
    return client.admin_get_user(
        UserPoolId=cognito_user_pool_id,
        Username=username,
    )


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

    (method_arn, token, username) = _get_arn_token_username(event)

    try:

        cognito_user = _get_user(
            username,
            event,
            boto3.client("cognito-idp"),
        )
        teams: str = _get_teams(cognito_user)

        return _get_policy(
            method_arn=method_arn,
            teams=teams,
            token=token,
        )
    except ClientError:
        return deny_policy()

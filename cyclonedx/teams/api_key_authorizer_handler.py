"""
-> Handler and associated policies are required for
-> authorization when uploading and SBOM.
"""

import datetime

import boto3

from cyclonedx.constants import (
    TEAM_TOKEN_TABLE_NAME,
)


def allow_policy(method_arn: str, teams: str):

    """
    :param method_arn: is the special amazon identifier for the method
    :param teams: are the teams that the user belongs to.
    :return: a policy that allows access to the resources requested.
    """

    return {
        "principalId": "apigateway.amazonaws.com",
        "context": {
            "teams": teams,
        },
        "policyDocument": {
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Action": "execute-api:Invoke",
                    "Effect": "Allow",
                    "Resource": method_arn,
                },
                {
                    "Action": "cognito-idp:ListUsers",
                    "Effect": "Allow",
                    "Resource": method_arn,
                },
            ],
        },
    }


def deny_policy():

    """
    :return: A policy that denies access to the resource.
    """

    return {
        "principalId": "*",
        "policyDocument": {
            "Version": "2012-10-17",
            "Statement": [{"Action": "*", "Effect": "Deny", "Resource": "*"}],
        },
    }


def api_key_authorizer_handler(event: dict, context: dict):

    """
    -> This is the handler used when uploading an SBOM.
    :param event: is the AWS event that fired the Lambda
    :param context: is the context...?  IDK what this is useful for.
    :return: a policy that either allows or denies access to the requested resources
    """

    # Extract the Method ARN and the token from the event
    method_arn = event["methodArn"]
    token = event["authorizationToken"]

    # Extract the path parameters and get the team
    path_params = event["pathParameters"]
    team_id = path_params["team"]

    # Get our Team table from DynamoDB
    team_token_table = boto3.resource("dynamodb").Table(TEAM_TOKEN_TABLE_NAME)

    # Get the team from the table
    get_team_tokens_rsp = team_token_table.query(
        Select="ALL_ATTRIBUTES",
        KeyConditionExpression="TeamId = :TeamId",
        ExpressionAttributeValues={
            ":TeamId": team_id,
        },
    )

    try:
        tokens = get_team_tokens_rsp["Items"]
    except KeyError as err:
        print(f"Key error: {err}")
        print(f"Query Response(Team): {get_team_tokens_rsp}")

    # Set the policy to default Deny
    policy = deny_policy()

    # Go through the tokens the team has
    for team_token in tokens:

        # Make sure the token is enabled
        if team_token["token"] == token and team_token["enabled"]:
            now = datetime.datetime.now().timestamp()
            expires = team_token["expires"]

            # Make sure the token is not expired
            if now < float(expires):
                policy = allow_policy(method_arn, "")

    # If the token exists, is enabled and not expired, then allow
    return policy

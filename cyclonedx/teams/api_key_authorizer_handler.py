import datetime

from cyclonedx.constants import (
    TEAM_TOKEN_TABLE_NAME,
)
from cyclonedx.core_utils.handler_commons import (
    dynamodb_resource,
    deny_policy,
    allow_policy
)


def api_key_authorizer_handler(event, context):

    # Extract the Method ARN and the token from the event
    method_arn = event["methodArn"]
    token = event["authorizationToken"]

    # Extract the path parameters and get the team
    path_params = event["pathParameters"]
    team_id = path_params["team"]

    # Get our Team table from DynamoDB
    team_token_table = dynamodb_resource.Table(TEAM_TOKEN_TABLE_NAME)

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
                policy = allow_policy(method_arn)

    # If the token exists, is enabled and not expired, then allow
    return policy

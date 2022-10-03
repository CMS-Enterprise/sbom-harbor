
import boto3
from jose import jwt


def allow_policy(method_arn: str, teams: str):
    return {
        "principalId": "apigateway.amazonaws.com",
        "context": {
            "teams": teams,
        },
        "policyDocument": {
            "Version": "2012-10-17",
            "Statement": [{
                "Action": "execute-api:Invoke",
                "Effect": "Allow",
                "Resource": method_arn
            }, {
                "Action": "cognito-idp:ListUsers",
                "Effect": "Allow",
                "Resource": method_arn
            }]
        }
    }


def deny_policy():
    return {
        "principalId": "*",
        "policyDocument": {
            "Version": "2012-10-17",
            "Statement": [{
                "Action": "*",
                "Effect": "Deny",
                "Resource": "*"
            }]
        }
    }


def jwt_authorizer_handler(event, context):

    print("<EVENT>")
    print(event)
    print("</EVENT>")

    print("<CONTEXT>")
    print(context)
    print("</CONTEXT>")

    method_arn = event["methodArn"]
    token = event["authorizationToken"]

    claims = jwt.get_unverified_claims(token)

    print(f"token_decoded: {claims}")

    iss: str = claims["iss"]
    cognito_user_pool_id = iss.rsplit('/', 1)[-1]

    username = claims["username"]
    client = boto3.client('cognito-idp')
    response = client.admin_get_user(
        UserPoolId=cognito_user_pool_id,
        Username=username
    )

    teams_attr = list(
        filter(
            lambda o: o["Name"] == "custom:teams",
            response['UserAttributes']
        )
    )[0]

    teams: str = teams_attr['Value']

    return allow_policy(method_arn, teams) if verify_token(token) else deny_policy()


def verify_token(token: str):
    return True

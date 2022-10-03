from os import environ
from json import dumps

from cyclonedx.constants import (
    USER_POOL_CLIENT_ID_KEY,
    USER_POOL_NAME_KEY,
)
from cyclonedx.core_utils.handler_commons import cognito_client
from cyclonedx.handlers.cyclonedx_util import (
    __get_body_from_first_record,
)


def login_handler(event, context):
    body = __get_body_from_first_record(event)

    username = body["username"]
    password = body["password"]

    try:
        resp = cognito_client.admin_initiate_auth(
            UserPoolId=environ.get(USER_POOL_NAME_KEY),
            ClientId=environ.get(USER_POOL_CLIENT_ID_KEY),
            AuthFlow='ADMIN_NO_SRP_AUTH',
            AuthParameters={
                "USERNAME": username,
                "PASSWORD": password
            }
        )
    except Exception as err:
        return __get_login_failed_response(401, err)

    jwt = resp['AuthenticationResult']['AccessToken']

    print("Log in success")
    print(f"Access token: {jwt}", )
    print(f"ID token: {resp['AuthenticationResult']['IdToken']}")

    return __get_login_success_response(jwt)


def __get_login_failed_response(status_code: int, err: Exception):
    return {
        "statusCode": status_code,
        "isBase64Encoded": False,
        "body": dumps(
            {
                "error": str(err),
            }
        ),
    }


def __get_login_success_response(jwt: str):
    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(
            {
                "token": jwt,
            }
        ),
    }

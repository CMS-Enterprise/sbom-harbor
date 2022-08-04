from cyclonedx.core_utils.handler_commons import (
    allow_policy,
    deny_policy
)


def jwt_authorizer_handler(event, context):

    print("<EVENT>")
    print(event)
    print("</EVENT>")

    print("<CONTEXT>")
    print(context)
    print("</CONTEXT>")

    method_arn = event["methodArn"]
    token = event["authorizationToken"]

    return allow_policy(method_arn) if verify_token(token) else deny_policy()


def verify_token(token: str):
    return True

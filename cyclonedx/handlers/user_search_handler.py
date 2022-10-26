"""
-> Module for User Search
"""
from os import environ

from cyclonedx.constants import (
    USER_POOL_NAME_KEY,
)
from cyclonedx.handlers.common import cognito_client, harbor_response
from cyclonedx.handlers.dependency_track import (
    __get_query_string_params_from_event,
)


def user_search_handler(event: dict = None, context: dict = None):

    """
    -> Handler for User Search
    """

    query_params = __get_query_string_params_from_event(event)

    filter_str = query_params["filter"]
    user_filter = f'email ^= "{filter_str}"'

    response = cognito_client.list_users(
        UserPoolId=environ.get(USER_POOL_NAME_KEY),
        AttributesToGet=[
            "email",
        ],
        Limit=60,  # Max is 60
        Filter=user_filter,
    )

    users = response["Users"]
    emails = []
    for user in users:
        attr = user["Attributes"]
        emails.append(attr[0]["Value"])

    return harbor_response(200, emails)

"""
-> Module for User Search
"""
from cyclonedx.ciam import HarborCognitoClient
from cyclonedx.handlers.common import harbor_response, print_values
from cyclonedx.handlers.dependency_track import (
    __get_query_string_params_from_event,
)


def user_search_handler(event: dict = None, context: dict = None):

    """
    -> Handler for User Search
    """

    print_values(event, context)
    query_params = __get_query_string_params_from_event(event)
    filter_str = query_params["filter"]
    cognito_client: HarborCognitoClient = HarborCognitoClient()
    emails: list[str] = cognito_client.get_matching_users(filter_str)

    return harbor_response(200, emails)

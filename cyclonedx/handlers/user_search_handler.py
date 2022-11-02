"""
-> Module for User Search
"""
from cyclonedx.ciam import HarborCognitoClient
from cyclonedx.handlers.common import (
    QueryStringKeys,
    _extract_value_from_qs,
    harbor_response,
    print_values,
)


def user_search_handler(event: dict = None, context: dict = None) -> dict:

    """
    -> Handler for User Search
    """

    print_values(event, context)
    filter_str = _extract_value_from_qs(QueryStringKeys.FILTER, event)
    cognito_client: HarborCognitoClient = HarborCognitoClient()
    emails: list[str] = cognito_client.get_matching_users(filter_str)

    return harbor_response(200, emails)

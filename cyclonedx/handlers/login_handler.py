"""
-> Module for the Login Handler
"""

from cyclonedx.clients.ciam import HarborCognitoClient
from cyclonedx.clients.dependency_track.dependency_track import (
    __get_body_from_first_record,
)
from cyclonedx.exceptions.ciam_exception import HarborCiamError
from cyclonedx.handlers.common import harbor_response, print_values


def login_handler(event: dict, context: dict):

    """
    -> Login Handler
    """

    print_values(event, context)

    body: dict = __get_body_from_first_record(event)
    username = body["username"]
    password = body["password"]

    cognito_client: HarborCognitoClient = HarborCognitoClient()

    try:
        jwt: str = cognito_client.get_jwt(username, password)
        return harbor_response(200, {"token": jwt})
    except HarborCiamError as ciam_err:
        return harbor_response(401, {"error": str(ciam_err)})

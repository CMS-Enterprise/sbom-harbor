import datetime
from decimal import Decimal
from uuid import uuid4

import boto3
from dateutil.relativedelta import relativedelta

from cyclonedx.constants import (
    TEAM_TOKEN_TABLE_NAME,
)
from cyclonedx.core_utils.cyclonedx_util import (
    __get_body_from_event,
    __token_response_obj,
)


def create_token_handler(event: dict = None, context: dict = None):

    """ Handler that creates a token, puts it in
    DynamoDB and returns it to the requester """

    # Get the team from the path parameters
    # and extract the body from the event
    team_id = event["pathParameters"]["team"]
    body = __get_body_from_event(event)

    # Create a new token starting with "sbom-api",
    # create a creation and expiration time
    token = f"sbom-api-{uuid4()}"
    now = datetime.datetime.now()
    later = now + relativedelta(years=1)

    # Get the timestamps to put in the database
    created = now.timestamp()
    expires = later.timestamp()

    # If a token name is given, set that as the name
    # otherwise put in a default
    name = body["name"] if body["name"] else "NoName"

    # Create a data structure representing the token
    # and it's metadata
    token_item = {
        "TeamId": team_id,
        "name": name,
        "created": Decimal(created),
        "expires": Decimal(expires),
        "enabled": True,
        "token": token,
    }

    # Get the dynamodb resource and add the token
    # to the existing team
    dynamodb = boto3.resource('dynamodb')
    table = dynamodb.Table(TEAM_TOKEN_TABLE_NAME)

    try:
        table.put_item(
            Item=token_item
        )
    except Exception as err:

        # If something happened in AWS that made it where the
        # call could not be completed, send an internal service error.
        return __token_response_obj(
            500, token, f"Request Error from boto3: {err}"
        )

    return __token_response_obj(200, token)

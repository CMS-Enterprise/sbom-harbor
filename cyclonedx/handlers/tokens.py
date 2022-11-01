"""
-> This module contains the handlers for CRUDing Tokens
"""
from datetime import datetime

from json import dumps, loads

import boto3
from dateutil.relativedelta import relativedelta

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers.common import (
    _extract_id_from_path,
    _extract_team_id_from_qs,
    _get_method,
    harbor_response,
    print_values,
    _should_process_children,
)
from cyclonedx.model import generate_model_id
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token, generate_token


def tokens_handler(event: dict, context: dict) -> dict:

    """
    ->  "Tokens" Handler. Handles requests to the /tokens endpoint.
    """

    print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    # Use ProjectId Extract existing
    # project from DynamoDB with children
    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    # fmt: off
    # Declare a response dictionary
    response: dict = {
        token.entity_id: token.to_json()
        for token in team.tokens
    }
    # fmt: on

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(response),
    }


def _do_get(event: dict, db_client: HarborDBClient) -> dict:

    # Get the project id from the path
    token_id: str = _extract_id_from_path("token", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    token = db_client.get(
        model=Token(
            team_id=team_id,
            token_id=token_id,
        ),
        recurse=_should_process_children(event),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({token_id: token.to_json()}),
    }


def _do_post(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> Handler that creates a token, puts it in
    -> DynamoDB and returns it to the requester
    """

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)
    token_id: str = generate_model_id()

    request_body: dict = loads(event["body"])

    # Create a creation and expiration time
    now = datetime.now()

    try:
        expires: str = request_body[Token.Fields.EXPIRES]
        expires_dt: datetime = datetime.fromisoformat(expires)
        if expires_dt <= now:
            raise ValueError("Specified expiration date is before now.")
    except KeyError:
        later = now + relativedelta(years=1)
        expires = later.isoformat()
    except ValueError as ve:
        raise ValueError("Unable to parse expiration date.") from ve

    # Get the timestamps to put in the database
    created = now.isoformat()

    token: Token = db_client.create(
        model=Token(
            team_id=team_id,
            token_id=token_id,
            name=request_body[Token.Fields.NAME],
            created=created,
            expires=expires,
            enabled=True,
            token=generate_token(),
        ),
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({token_id: token.to_json()}),
    }


def _do_put(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> The behavior of this function is that the objects in the request_body
    -> will be updated.
    """

    # Get the token id from the path
    token_id: str = _extract_id_from_path("token", event)

    # Get the ProjectId from the Path Parameter
    team_id: str = _extract_team_id_from_qs(event)

    # Use ProjectId Extract existing project from DynamoDB with children
    token: Token = db_client.get(
        model=Token(
            team_id=team_id,
            token_id=token_id,
        ),
    )

    # Extract the request body from the event
    request_body: dict = loads(event["body"])

    # Replace the name of the project if there is a 'name' key in the request body
    try:
        token.name = request_body.get(Token.Fields.NAME)
        token.enabled = request_body.get(Token.Fields.ENABLED)
    except KeyError:
        ...

    token = db_client.update(
        model=token,
        recurse=False,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({token_id: token.to_json()}),
    }


def _do_delete(event: dict, db_client: HarborDBClient) -> dict:

    # Get the project id from the path
    token_id: str = _extract_id_from_path("token", event)

    # Get the team id from the querystring
    team_id: str = _extract_team_id_from_qs(event)

    token: Token = db_client.get(
        model=Token(
            team_id=team_id,
            token_id=token_id,
        ),
    )

    db_client.delete(
        model=token,
    )

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps({token_id: token.to_json()}),
    }


def token_handler(event: dict, context: dict) -> dict:

    """
    ->  "Project" Handler.  Handles requests to the /project endpoint.
    """

    # Print the incoming values, so we can see them in
    # CloudWatch if there is an issue.
    print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    # Get the verb (method) of the request.  We will use it
    # to decide what type of operation we execute on the incoming data
    method: str = _get_method(event)

    try:
        result: dict = {}
        if method == "GET":
            result = _do_get(event, db_client)
        elif method == "POST":
            result = _do_post(event, db_client)
        elif method == "PUT":
            result = _do_put(event, db_client)
        elif method == "DELETE":
            result = _do_delete(event, db_client)
        return result
    except (ValueError, DatabaseError) as e:
        return harbor_response(400, {"error": str(e)})

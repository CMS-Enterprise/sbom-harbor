"""
-> This module contains the handlers for CRUDing Teams
"""
import datetime
from datetime import timedelta
from json import loads

import boto3
from cyclonedx.constants import COGNITO_TEAM_DELIMITER
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.handlers.common import (
    _extract_id_from_path,
    _get_method,
    print_values,
    harbor_response,
    extract_attrib_from_event,
    _should_process_children,
    _to_members,
    _to_projects,
    _update_members,
    _update_projects,
)
from cyclonedx.model import generate_model_id
from cyclonedx.model.team import Team
from cyclonedx.model.member import Member
from cyclonedx.model.token import Token, generate_token


def teams_handler(event: dict, context: dict) -> dict:

    """
    ->  "Teams" Handler. Handles requests to the /teams endpoint.
    """

    print_values(event, context)

    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    try:

        # Dig the teams ids out of the response we put into the policy
        # that dictates if the user can even access the resource.
        team_ids: str = extract_attrib_from_event("teams", event)

        # Split the string up if the delimiter exists.  Each string token
        # is treated like a separate team id.
        if COGNITO_TEAM_DELIMITER in team_ids:
            team_ids_lst = team_ids.split(COGNITO_TEAM_DELIMITER)
        else:
            team_ids_lst = [team_ids]

        # Get the children if there are any
        get_children: bool = _should_process_children(event)

        # Declare a response dictionary
        response_dict: dict = {}

        # Iterate over the list of ids and get the teams.
        for team_id in team_ids_lst:
            team: Team = Team(team_id=team_id)
            team = db_client.get(team, recurse=get_children)
            response_dict[team.team_id] = team.to_json()

    except DatabaseError as de:
        return harbor_response(400, {"error": str(de)})
    except KeyError as ke:
        return harbor_response(400, {"error": str(ke)})

    return harbor_response(200, response_dict)


def _do_get(event: dict, db_client: HarborDBClient) -> dict:

    team_id: str = _extract_id_from_path("team", event)
    team = db_client.get(
        model=Team(team_id=team_id),
        recurse=_should_process_children(event),
    )

    return harbor_response(
        200,
        {
            team_id: team.to_json(),
        },
    )


def _do_post(event: dict, db_client: HarborDBClient) -> dict:

    request_body: dict = loads(event["body"])
    team_id: str = generate_model_id()
    user_email: str = extract_attrib_from_event("user_email", event)

    created: datetime = datetime.datetime.now()
    expires: datetime = created + timedelta(weeks=1)

    creating_member: Member = Member(
        team_id=team_id,
        member_id=generate_model_id(),
        email=user_email,
        is_team_lead=True,
    )

    members: list[Member] = _to_members(team_id, request_body)

    if creating_member not in members:
        members.append(creating_member)

    team: Team = db_client.create(
        model=Team(
            team_id=team_id,
            name=request_body[Team.Fields.NAME],
            members=members,
            projects=_to_projects(team_id, request_body),
            tokens=[
                Token(
                    team_id=team_id,
                    token_id=generate_model_id(),
                    name="Initial Token",
                    created=created.isoformat(),
                    expires=expires.isoformat(),
                    enabled=True,
                    token=generate_token(),
                )
            ],
        ),
        recurse=True,
    )

    return harbor_response(
        200,
        {
            team_id: team.to_json(),
        },
    )


def _do_put(event: dict, db_client: HarborDBClient) -> dict:

    """
    -> The behavior of this function is that the objects in the request_body
    -> will be updated.  If a new object (project or member) comes in the request,
    -> it will not be created.  If a child object noes not exist in the request_body
    -> and exists in the database, the object will not be deleted.  Objects can only
    -> be modified, never created or deleted.
    """

    # Get the TeamId from the Path Parameter
    team_id: str = _extract_id_from_path("team", event)

    # Use TeamId Extract existing team from DynamoDB with children
    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    # Extract the request body from the event
    request_body: dict = loads(event["body"])

    # Replace the name of the team if there is a 'name' key in the request body
    try:
        team.name = request_body[Team.Fields.NAME]
    except KeyError:
        ...

    team = _update_projects(
        team=team,
        request_body=request_body,
    )

    team = _update_members(
        team=team,
        request_body=request_body,
    )

    team = db_client.update(
        model=team,
        recurse=False,
    )

    return harbor_response(
        200,
        {
            team_id: team.to_json(),
        },
    )


def _do_delete(event: dict, db_client: HarborDBClient) -> dict:

    team_id: str = _extract_id_from_path("team", event)

    team: Team = db_client.get(
        model=Team(team_id=team_id),
        recurse=True,
    )

    db_client.delete(
        model=team,
        recurse=True,
    )

    return harbor_response(
        200,
        {
            team_id: {},
        },
    )


def team_handler(event: dict, context: dict) -> dict:

    """
    ->  "Team" Handler.  Handles requests to the /team endpoint.
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
    except ValueError as ve:
        return harbor_response(400, {"error": str(ve)})
    except DatabaseError as de:
        return harbor_response(400, {"error": str(de)})

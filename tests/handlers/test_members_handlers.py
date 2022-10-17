"""
-> Test for the member handlers
"""
import uuid
from json import (
    dumps,
    loads,
)

import boto3
from moto import mock_dynamodb

from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.model.team import Team
from cyclonedx.model.member import Member

# TODO I'm testing moving this here to see
#  if the @mock_dynamodb annotation still works.
#  Pylint hates imports inside of functions, so
#  we should try leaving it here. However, if this
#  test fails, be highly suspicious of this
#  and move it back into the test function.
#  The Pylint error can be suppressed with:
#  # pylint: disable=C0415
#  over the imports inside the test function.
from cyclonedx.handlers import (
    members_handler,
    member_handler,
)

from tests.conftest import create_harbor_table


@mock_dynamodb
def test_flow():

    """
    -> Test the creation, updating and deletion of a member.
    """

    db_client: HarborDBClient = HarborDBClient(
        dynamodb_resource=boto3.resource("dynamodb")
    )

    create_harbor_table(boto3.resource("dynamodb"))

    team_id: str = str(uuid.uuid4())
    email: str = "test@email.net"

    db_client.create(
        Team(
            team_id=team_id,
            name="Test Team Name",
        ),
    )

    # Create
    create_response: dict = create(
        team_id=team_id,
        email=email,
        is_team_lead=True,
        handler=member_handler,
    )
    response_dict: dict = loads(create_response["body"])

    print(dumps(response_dict, indent=2))

    member_id: str = list(response_dict.keys()).pop()
    member_dict: dict = response_dict[member_id]
    assert email == member_dict[Member.Fields.EMAIL]
    assert member_dict[Member.Fields.IS_TEAM_LEAD]

    # Get Test 1
    get_response: dict = get(
        team_id=team_id,
        member_id=member_id,
        handler=member_handler,
    )
    response_dict = loads(get_response["body"])
    member_dict: dict = response_dict[member_id]
    assert email == member_dict[Member.Fields.EMAIL]
    assert member_dict[Member.Fields.IS_TEAM_LEAD]

    # Get Test 2
    get_response: dict = get_all(
        team_id=team_id,
        handler=members_handler,
    )
    response_dict = loads(get_response["body"])

    member_id: str = list(response_dict.keys()).pop()
    member_dict: dict = response_dict[member_id]
    assert email == member_dict[Member.Fields.EMAIL]
    assert member_dict[Member.Fields.IS_TEAM_LEAD]

    # Update
    new_email: str = "new@email.org"

    update(
        team_id=team_id,
        member_id=member_id,
        new_email=new_email,
        new_is_team_lead=False,
        handler=member_handler,
    )

    test_member: Member = db_client.get(
        Member(
            team_id=team_id,
            member_id=member_id,
        )
    )

    assert new_email == test_member.email
    assert not test_member.is_team_lead

    # Delete
    delete(
        team_id=team_id,
        member_id=member_id,
        handler=member_handler,
    )

    # Get Test (Should return nothing)
    get_response: dict = get(
        team_id=team_id,
        member_id=member_id,
        handler=member_handler,
    )
    assert get_response["statusCode"] == 400
    db_client.delete(Team(team_id=team_id))


@mock_dynamodb
def test_no_team_id():

    """
    -> Attempt to create a member; Negative flow, no team id
    """

    for method in "GET", "PUT", "POST", "DELETE":
        event: dict = {
            "requestContext": {
                "http": {
                    "method": method,
                }
            },
            "queryStringParameters": {
                "children": True,
                "projectId": "TEST",
            },
            "body": dumps(
                {
                    "name": "TEST",
                    "language": "TEST",
                    "buildTool": "TEST",
                }
            ),
        }

        response: dict = member_handler(event, {})
        assert response["statusCode"] == 400


# pylint: disable=R0913
def create(
    team_id: str,
    email: str,
    is_team_lead: bool,
    handler,
):

    """
    -> Create a member
    """

    event: dict = {
        "requestContext": {
            "http": {
                "method": "POST",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
        "body": dumps(
            {
                Member.Fields.EMAIL: email,
                Member.Fields.IS_TEAM_LEAD: is_team_lead,
            }
        ),
    }

    return handler(event, {})


def get(team_id: str, member_id: str, handler):

    """
    -> Get a member
    """

    event: dict = {
        "pathParameters": {
            "member": member_id,
        },
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
    }

    return handler(event, {})


def get_all(team_id: str, handler):

    """
    -> Get all the members
    """

    event: dict = {
        "requestContext": {
            "http": {
                "method": "GET",
            }
        },
        "queryStringParameters": {
            "children": True,
            "teamId": team_id,
        },
    }

    return handler(event, {})


# pylint: disable=R0913
def update(
    team_id: str,
    member_id: str,
    new_email: str,
    new_is_team_lead: bool,
    handler,
):
    """
    -> Update a member's data
    """

    event: dict = {
        "pathParameters": {
            "member": member_id,
        },
        "requestContext": {
            "http": {
                "method": "PUT",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
        "body": dumps(
            {
                Member.Fields.EMAIL: new_email,
                Member.Fields.IS_TEAM_LEAD: new_is_team_lead,
            }
        ),
    }

    return handler(event, {})


def delete(team_id: str, member_id: str, handler):

    """
    -> Delete a member
    """

    event: dict = {
        "pathParameters": {
            "member": member_id,
        },
        "requestContext": {
            "http": {
                "method": "DELETE",
            }
        },
        "queryStringParameters": {
            "teamId": team_id,
        },
    }

    return handler(event, {})

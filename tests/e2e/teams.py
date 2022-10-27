"""
-> End-to-End Test for the teams
"""

from uuid import uuid4

import boto3
from _pytest.outcomes import fail

import requests
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.model.project import Project
from cyclonedx.model.member import Member
from cyclonedx.model.team import Team
from tests.data.add_test_team_data_to_dynamodb import test_add_test_team
from tests.data.create_cognito_users import test_create_cognito_users
from tests.e2e import (
    login,
    get_cloudfront_url,
    print_response,
)

cf_url: str = get_cloudfront_url()


def test_user_creating_team_is_member():

    """
    -> Function to test that if a user creates a team,
    -> they are a member of that team.
    """

    test_create_cognito_users()

    _, team_id = test_create_team()

    new_team: Team = HarborDBClient(boto3.resource("dynamodb")).get(
        Team(team_id=team_id),
        recurse=True,
    )

    member: Member = new_team.members.pop()
    assert member.email == "sbomadmin@aquia.io"


def test_get_two_separate_endpoints():

    """
    -> In this test, we get a single JWT from the login endpoint.
    -> We then get a team and then a project using the same JWT.
    -> If successful, then we know we can hit two different endpoints
    -> with the same JWT and not be Forbidden(401).
    """

    resource = boto3.resource("dynamodb")

    team_id: str = str(uuid4())
    project_id: str = str(uuid4())

    HarborDBClient(resource).create(
        Team(
            team_id=team_id,
            name="Test Team",
            projects=[
                Project(
                    name="Test Project",
                    team_id=team_id,
                    project_id=project_id,
                )
            ],
        ),
        recurse=True,
    )

    # Getting only one JWT
    jwt: str = login(cf_url)

    # Get the Project
    url: str = f"{cf_url}/api/v1/project/{project_id}?teamId={team_id}"
    print(f"Sending To: GET:{url}")
    project_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )
    print_response(project_rsp)

    # Get the team
    url: str = f"{cf_url}/api/v1/team/{team_id}"
    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )
    print_response(team_rsp)

    HarborDBClient(resource).delete(
        Team(
            team_id=team_id,
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                )
            ],
        ),
        recurse=True,
    )


def test_get_teams():

    """
    -> Get the teams
    """

    test_add_test_team()
    test_create_cognito_users()

    jwt = login(cf_url)

    url = f"{cf_url}/api/v1/teams"

    print(f"Sending To: GET:{url}")
    teams_rsp = requests.get(url, headers={"Authorization": jwt})

    try:

        teams = teams_rsp.json()
        print_response(teams_rsp)

        dp1 = teams["dawn-patrol"]
        assert not dp1["members"]
        assert not dp1["tokens"]
        assert not dp1["projects"]

        dp2 = teams["dusk-patrol"]
        assert not dp2["members"]
        assert not dp2["tokens"]
        assert not dp2["projects"]

    except KeyError:
        fail()


def test_get_teams_with_children():

    """
    -> Get Teams With Children
    """

    test_add_test_team()
    test_create_cognito_users()

    jwt = login(cf_url)

    url = f"{cf_url}/api/v1/teams?children=true"

    print(f"Sending To: GET:{url}")
    teams_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )

    try:

        teams = teams_rsp.json()
        print_response(teams_rsp)

        dp1 = teams["dawn-patrol"]
        assert dp1["members"]
        assert dp1["tokens"]
        assert dp1["projects"]

        dp2 = teams["dusk-patrol"]
        assert dp2["members"]
        assert dp2["tokens"]
        assert dp2["projects"]

    except KeyError:
        fail()


def test_get_team(team_id: str = None):

    """
    -> Get Team. The results of this query will not have children
    """

    if not team_id:
        jwt, team_id = test_create_team()
    else:
        jwt = login(cf_url)

    url = f"{cf_url}/api/v1/team/{team_id}"

    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()

    assert list(team_dict.keys()) == [team_id]


def test_get_team_with_children(team_id: str = None):

    """
    -> Get team with children
    """

    if not team_id:
        jwt, team_id = test_create_team()
    else:
        jwt = login(cf_url)

    url = f"{cf_url}/api/v1/team/{team_id}?children=true"

    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()

    assert list(team_dict.keys()) == [team_id]

    return team_dict


def test_create_team():

    """
    -> Create Team
    """

    jwt = login(cf_url)

    name: str = "Test Team Name"

    url = f"{cf_url}/api/v1/team"

    print(f"Sending To: POST:{url}")
    team_rsp = requests.post(
        url,
        headers={
            "Authorization": jwt,
        },
        json={
            "name": name,
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()
    new_team_id: str = list(team_dict.keys())[0]

    assert new_team_id != ""

    return jwt, new_team_id


def test_create_team_with_children():

    """
    -> Create Team with Children
    """

    jwt = login(cf_url)

    name: str = "TestTeamName"
    proj1_name: str = "TestProjectName1"
    proj2_name: str = "TestProjectName2"

    url = f"{cf_url}/api/v1/team"

    print(f"Sending To: POST:{url}")
    team_rsp = requests.post(
        url,
        headers={"Authorization": jwt},
        json={
            "name": name,
            "projects": [
                {
                    "name": proj1_name,
                },
                {
                    "name": proj2_name,
                },
            ],
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()
    new_team_id: str = list(team_dict.keys())[0]
    project_data: dict = team_dict[new_team_id]["projects"]
    project_ids: list[str] = list(project_data.keys())

    assert "" is not new_team_id

    return jwt, {
        "team_id": new_team_id,
        "project_ids": project_ids,
    }


def test_update_team():

    """
    -> Update Team
    """

    jwt = login(cf_url)

    new_name: str = "test_name_update"

    url = f"{cf_url}/api/v1/team/dawn-patrol"

    print(f"Sending To: PUT:{url}")
    team_rsp = requests.put(
        url,
        headers={
            "Authorization": jwt,
        },
        json={
            "name": new_name,
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()
    team_data: dict = team_dict["dawn-patrol"]
    new_name_from_service: str = team_data["name"]

    assert new_name == new_name_from_service


def test_update_team_with_children():

    # TODO Complete

    """
    -> This test should update the team name and the names of both projects.
    -> We know the projects exist because the test_create_team_with_children test
    -> creates them.
    """

    jwt, ids = test_create_team_with_children()

    team_id: str = ids["team_id"]
    project_ids: list[str] = ids["project_ids"]

    name: str = "TestTeamName"
    proj1_name: str = "1oCHANGEDProjectNameo1"
    proj2_name: str = "2oCHANGEDProjectNameo2"

    url = f"{cf_url}/api/v1/team/{team_id}?children=true"

    print(f"Sending To: PUT:{url}")
    team_rsp = requests.put(
        url,
        headers={
            "Authorization": jwt,
        },
        json={
            "name": name,
            "projects": [
                {
                    "id": project_ids[0],
                    "name": proj1_name,
                },
                {
                    "id": project_ids[1],
                    "name": proj2_name,
                },
            ],
        },
    )

    print_response(team_rsp)


def test_delete_team():

    """
    -> Delete Team
    """

    jwt, team_id = test_create_team()

    url = f"{cf_url}/api/v1/team/{team_id}"

    print(f"Sending To: DELETE:{url}")
    team_rsp = requests.delete(
        url,
        headers={"Authorization": jwt},
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()
    team_id_after_delete: str = list(team_dict.keys())[0]

    assert team_id == team_id_after_delete


def test_initial_token_created_when_team_created():

    """
    -> Test that an initial token is created when a team is created
    """

    _, team_id = test_create_team()
    rsp: dict = test_get_team_with_children(team_id)
    team: dict = list(rsp.values()).pop()
    tokens: dict = team["tokens"]

    assert len(tokens.values()) == 1

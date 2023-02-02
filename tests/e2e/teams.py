"""
-> End-to-End Test for the teams
"""

from uuid import uuid4

import boto3
import requests
from _pytest.outcomes import fail
from requests import Response, get

from cyclonedx.clients.db.dynamodb import HarborDBClient
from cyclonedx.model.member import Member
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from tests.data.add_test_team_data_to_dynamodb import test_add_test_team
from tests.data.create_cognito_users import test_create_cognito_users
from tests.e2e import (
    create_team_with_projects,
    get_cloudfront_url,
    get_team_url,
    login,
    print_response,
)


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

    member: Member = new_team.members[0]
    assert member.email == "sbomadmin@aquia.io"


def test_get_two_separate_endpoints():

    """
    -> In this test, we get a single JWT from the login endpoint.
    -> We then get a team and then a project using the same JWT.
    -> If successful, then we know we can hit two different endpoints
    -> with the same JWT and not be Forbidden(401).
    """

    resource = boto3.resource("dynamodb")
    cf_url: str = get_cloudfront_url()
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

    cf_url: str = get_cloudfront_url()
    test_add_test_team()
    test_create_cognito_users()

    jwt = login(cf_url)

    url = f"{cf_url}/api/v1/teams"

    print(f"Sending To: GET:{url}")
    teams_rsp = requests.get(url, headers={"Authorization": jwt})

    try:

        teams = teams_rsp.json()
        print_response(teams_rsp)

        dp1 = [team for team in teams if team["name"] == "dawn-patrol"][0]
        assert not dp1["members"]
        assert not dp1["tokens"]
        assert not dp1["projects"]

        dp2 = [team for team in teams if team["name"] == "dusk-patrol"][0]
        assert not dp2["members"]
        assert not dp2["tokens"]
        assert not dp2["projects"]

    except KeyError:
        fail()


def test_get_teams_with_children():

    """
    -> Get Teams With Children
    """

    cf_url: str = get_cloudfront_url()
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

        dp1 = [team for team in teams if team["name"] == "dawn-patrol"][0]
        assert dp1["members"]
        assert dp1["tokens"]
        assert dp1["projects"]

        dp2 = [team for team in teams if team["name"] == "dusk-patrol"][0]
        assert dp2["members"]
        assert dp2["tokens"]
        assert dp2["projects"]

    except KeyError:
        fail()


def test_get_team(team_id: str = None):

    """
    -> Get Team. The results of this query will not have children
    """

    cf_url: str = get_cloudfront_url()

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

    assert team_dict["id"] == team_id


def test_get_team_with_children(team_id: str = None):

    """
    -> Get team with children
    """

    cf_url: str = get_cloudfront_url()

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

    assert team_dict["id"] == team_id

    return team_dict


def test_create_team():

    """
    -> Create Team
    """

    cf_url: str = get_cloudfront_url()

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
    new_team_id: str = team_dict["id"]

    assert new_team_id != ""

    return jwt, new_team_id


def test_create_team_with_children():

    """
    -> Create Team with Children
    """

    cf_url: str = get_cloudfront_url()

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
            "members": [
                {"email": "sbomadmin@aquia.io", "isTeamLead": "true"},
            ],
            "projects": [
                {
                    "name": proj1_name,
                    "codebases": [
                        {
                            "name": proj1_name,
                        }
                    ],
                },
                {
                    "name": proj2_name,
                    "codebases": [
                        {
                            "name": proj2_name,
                        }
                    ],
                },
            ],
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()
    new_team_id: str = team_dict["id"]
    project_list: dict = team_dict["projects"]
    project_ids: list[str] = [project["id"] for project in project_list]

    assert "" is not new_team_id

    return jwt, {
        "team_id": new_team_id,
        "project_ids": project_ids,
    }


def test_update_team():

    """
    -> Update Team
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    # Create a team with 2 projects
    team_name: str = "1Team1"
    proj1_name: str = "1Project1"
    proj2_name: str = "2Project2"

    team_url: str = get_team_url(cf_url)
    create_json: dict = create_team_with_projects(
        team_name=team_name,
        project_names=[
            proj1_name,
            proj2_name,
        ],
        team_url=team_url,
        jwt=jwt,
    )

    team_id: str = create_json.get("id")
    print(f"Sending To: GET:{team_url}")
    get_rsp: Response = get(
        f"{team_url}/{team_id}?children=true",
        headers={
            "Authorization": jwt,
        },
    )
    print_response(get_rsp)

    new_name: str = "NEWTESTTEAMNAME"

    print(f"Sending To: PUT:{team_url}/{team_id}?children=true")
    team_rsp = requests.put(
        f"{team_url}/{team_id}?children=true",
        headers={
            "Authorization": jwt,
        },
        json={
            "name": new_name,
        },
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()
    new_name_from_service: str = team_dict["name"]

    assert new_name == new_name_from_service


def test_update_team_with_children():

    # TODO Complete

    """
    -> This test should update the team name and the names of both projects.
    -> We know the projects exist because the test_create_team_with_children test
    -> creates them.
    """

    cf_url: str = get_cloudfront_url()

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

    cf_url: str = get_cloudfront_url()

    url = f"{cf_url}/api/v1/team/{team_id}"

    print(f"Sending To: DELETE:{url}")
    team_rsp = requests.delete(
        url,
        headers={"Authorization": jwt},
    )

    print_response(team_rsp)
    team_dict: dict = team_rsp.json()

    assert len(team_dict) == 0


def test_initial_token_created_when_team_created():

    """
    -> Test that an initial token is created when a team is created
    """

    _, team_id = test_create_team()
    team: dict = test_get_team_with_children(team_id)
    tokens: dict = team["tokens"]

    assert len(tokens) == 1

def test_check_duplicate_team_name():

    """
    -> Test that team creation fails if the team name submitted already exists in dynamoDB
    """
    # Create team 1
    cf_url: str = get_cloudfront_url()

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

    # Create team 2 with duplicate name

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

    assert team_rsp.status_code == 400

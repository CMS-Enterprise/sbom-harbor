""" End-to-End Test for the system """

from json import dumps
from uuid import uuid4

import boto3
from _pytest.outcomes import fail

import requests
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team

client = boto3.client("cloudfront")
distributions = client.list_distributions()
distribution_list = distributions["DistributionList"]

try:
    sbom_api_distribution = distribution_list["Items"][0]
    cf_domain_name = sbom_api_distribution["DomainName"]
    origins = sbom_api_distribution["Origins"]["Items"]

    apigw_domain_name = ""
    for origin in origins:
        domain_name: str = origin["DomainName"]
        if "execute-api" in domain_name:
            apigw_domain_name = domain_name

    CF_URL = f"https://{cf_domain_name}"

except KeyError:
    ...


def _login():

    login_url = f"https://{cf_domain_name}/api/v1/login"
    user = "sbomadmin@aquia.io"
    password = "L0g1nTe5tP@55!"

    print(f"Sending To: POST:{login_url}, With: {user}, {password}")
    login_rsp = requests.post(login_url, json={"username": user, "password": password})

    login_rsp_json = login_rsp.json()
    print(f"Login Response: {dumps(login_rsp_json, indent=2)}")
    return login_rsp_json["token"]


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
    jwt: str = _login()

    # Get the Project
    url: str = f"https://{cf_domain_name}/api/v1/project/{project_id}?teamId={team_id}"
    print(f"Sending To: GET:{url}")
    project_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )
    print(
        f"Response: ({project_rsp.status_code}) {dumps(project_rsp.json(), indent=2)}"
    )

    # Get the team
    url: str = f"https://{cf_domain_name}/api/v1/team/{team_id}"
    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )
    print(f"Response: ({team_rsp.status_code}) {dumps(team_rsp.json(), indent=2)}")

    # jwt: str = _login()

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

    jwt = _login()

    url = f"https://{cf_domain_name}/api/v1/teams"

    print(f"Sending To: GET:{url}")
    teams_rsp = requests.get(url, headers={"Authorization": jwt})

    try:

        teams = teams_rsp.json()
        print(teams)

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

    jwt = _login()

    url = f"https://{cf_domain_name}/api/v1/teams?children=true"

    print(f"Sending To: GET:{url}")
    teams_rsp = requests.get(
        url,
        headers={
            "Authorization": jwt,
        },
    )

    try:

        teams = teams_rsp.json()
        print(teams)

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


def test_get_team():

    """
    -> Get Team
    """

    jwt = _login()

    team_id: str = "18f863b5-0d3d-43cf-87e3-33a6a7d5842d"

    url = f"https://{cf_domain_name}/api/v1/team/{team_id}"

    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(url, headers={"Authorization": jwt})

    team_dict: dict = team_rsp.json()

    print(f"Team Endpoint Response: {dumps(team_dict, indent=2)}")
    assert list(team_dict.keys()) == [team_id]


def test_get_team_with_children():

    """
    -> Get team with children
    """

    jwt = _login()

    team_id: str = "18f863b5-0d3d-43cf-87e3-33a6a7d5842d"

    url = f"https://{cf_domain_name}/api/v1/team/{team_id}?children=true"

    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(url, headers={"Authorization": jwt})

    team_dict: dict = team_rsp.json()

    print(dumps(team_dict, indent=2))

    assert list(team_dict.keys()) == [team_id]

    try:

        dp1 = team_dict[team_id]
        assert dp1["projects"]

    except KeyError:
        fail()


def test_create_team():

    """
    -> Create Team
    """

    jwt = _login()

    name: str = "TeamName"

    url = "https://b5jpfzyp5l.execute-api.us-east-1.amazonaws.com/api/v1/team"

    print(f"Sending To: POST:{url}")
    team_rsp = requests.post(
        url,
        headers={"Authorization": jwt},
        json={
            "name": name,
        },
    )

    team_dict: dict = team_rsp.json()
    print(f"Create, team dict: {team_dict}")
    new_team_id: str = list(team_dict.keys())[0]

    assert "" is not new_team_id

    return jwt, new_team_id


def test_create_team_with_children():

    """
    -> Create Team with Children
    """

    jwt = _login()

    name: str = "TestTeamName"
    proj1_name: str = "TestProjectName1"
    proj2_name: str = "TestProjectName2"

    url = "https://b5jpfzyp5l.execute-api.us-east-1.amazonaws.com/api/v1/team"

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

    team_dict: dict = team_rsp.json()
    print(f"Create, team dict: {dumps(team_dict, indent=2)}")
    new_team_id: str = list(team_dict.keys())[0]
    project_data: dict = team_dict[new_team_id]["projects"]
    project_ids: list[str] = list(project_data.keys())

    assert "" is not new_team_id

    return jwt, {"team_id": new_team_id, "project_ids": project_ids}


def test_update_team():

    """
    -> Update Team
    """

    jwt = _login()

    new_name: str = "test_name_update"

    url = (
        "https://b5jpfzyp5l.execute-api.us-east-1.amazonaws.com/api/v1/team/dawn-patrol"
    )

    print(f"Sending To: PUT:{url}")
    team_rsp = requests.put(
        url,
        headers={"Authorization": jwt},
        json={
            "name": new_name,
        },
    )

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

    (jwt, ids) = test_create_team_with_children()

    team_id: str = ids["team_id"]
    project_ids: list[str] = ids["project_ids"]

    name: str = "TestTeamName"
    proj1_name: str = "1oCHANGEDProjectNameo1"
    proj2_name: str = "2oCHANGEDProjectNameo2"
    apigw_url: str = "https://dvu7djeqv2.execute-api.us-east-1.amazonaws.com"
    url = f"{apigw_url}/api/v1/team/{team_id}?children=true"

    print(f"Sending To: PUT:{url}")
    team_rsp = requests.put(
        url,
        headers={"Authorization": jwt},
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

    team_dict: dict = team_rsp.json()
    print(f"Create, team dict: {dumps(team_dict, indent=2)}")


def test_delete_team():

    """
    -> Delete Team
    """

    jwt, team_id = test_create_team()

    url = (
        f"https://b5jpfzyp5l.execute-api.us-east-1.amazonaws.com/api/v1/team/{team_id}"
    )

    print(f"Sending To: DELETE:{url}")
    team_rsp = requests.delete(
        url,
        headers={"Authorization": jwt},
    )

    team_dict: dict = team_rsp.json()

    print(f"Delete, team dict: {team_dict}")
    team_id_after_delete: str = list(team_dict.keys())[0]

    assert team_id == team_id_after_delete

""" End-to-End Test for the system """

import importlib.resources as pr
from json import loads, dumps
from optparse import OptionParser
import boto3

from _pytest.outcomes import fail

import requests
import tests.sboms as sboms


client = boto3.client("cloudfront")
distributions = client.list_distributions()
distribution_list = distributions["DistributionList"]

# only one right now

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
    APIGW_URL = f"https://{apigw_domain_name}"

    URL = CF_URL

    REGION = "us-east-1"
    STAGE = "prod"

    USER = "sbomadmin@aquia.io"
    PASS = "L0g1nTe5tP@55!"

    LOGIN_URL = f"{URL}/api/login"
    TEAM_URL = f"{URL}/api/team"

    team = "abc123"
    project = "AwesomeProj"
    codebase = "Website"

    SBOM_UPLOAD_URL = f"{URL}/api/{team}/{project}/{codebase}/sbom"
    USER_SEARCH_URL = f"{URL}/api/user/search"

    SBOM = loads(pr.read_text(sboms, "keycloak.json"))
    parser = OptionParser("usage: %prog [options]")
    parser.add_option("--fail", dest="fail", help="fail flag", action="store")
except KeyError:
    ...


def __get_token_url(team_name: str, token=None):
    url = f"{URL}/api/{team_name}/token"

    if token:
        url = f"{url}/{token}"

    return url


def __login():

    login_url = "https://dvu7djeqv2.execute-api.us-east-1.amazonaws.com/api/login"
    user = "sbomadmin@aquia.io"
    password = "L0g1nTe5tP@55!"

    print(f"Sending To: POST:{login_url}, With: {user}, {password}")
    login_rsp = requests.post(login_url, json={"username": user, "password": password})

    print(f"Login Rsp: {login_rsp.text}")

    login_rsp_json = login_rsp.json()
    print(f"Login Response: {dumps(login_rsp_json, indent=2)}")
    return login_rsp_json["token"]


def test_get_teams():

    """
    -> Get the teams
    """

    jwt = __login()

    url = "https://b5jpfzyp5l.execute-api.us-east-1.amazonaws.com/api/v1/teams"

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

    jwt = __login()

    url = "https://b5jpfzyp5l.execute-api.us-east-1.amazonaws.com/api/v1/teams?children=true"

    print(f"Sending To: GET:{url}")
    teams_rsp = requests.get(url, headers={"Authorization": jwt})

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

    jwt = __login()

    team_id: str = "18f863b5-0d3d-43cf-87e3-33a6a7d5842d"

    url = (
        f"https://dvu7djeqv2.execute-api.us-east-1.amazonaws.com/api/v1/team/{team_id}"
    )

    print(f"Sending To: GET:{url}")
    team_rsp = requests.get(url, headers={"Authorization": jwt})

    team_dict: dict = team_rsp.json()

    print(f"Team Endpoint Response: {dumps(team_dict, indent=2)}")
    assert list(team_dict.keys()) == [team_id]


def test_get_team_with_children():

    """
    -> Get team with children
    """

    jwt = __login()

    team_id: str = "18f863b5-0d3d-43cf-87e3-33a6a7d5842d"

    apigw_url: str = "https://dvu7djeqv2.execute-api.us-east-1.amazonaws.com"
    url = f"{apigw_url}/api/v1/team/{team_id}?children=true"

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

    jwt = __login()

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

    jwt = __login()

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

    jwt = __login()

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


def test_token():

    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    (options, _) = parser.parse_args()

    login_fail = False
    create_fail = False
    delete_fail = False
    if options.fail:
        if options.fail == "login":
            login_fail = True
        elif options.fail == "create":
            create_fail = True
        elif options.fail == "delete":
            delete_fail = True
        else:
            print(f"{options.fail} is not a failure option")

    print(f"Sending To: POST:{LOGIN_URL}")
    login_rsp = requests.post(
        LOGIN_URL,
        json={"username": USER, "password": "wrong_password" if login_fail else PASS},
    )

    login_rsp_json = login_rsp.json()
    print(f"Response: {login_rsp_json}")

    if not login_fail:

        jwt = login_rsp_json["token"]

        create_token_url = __get_token_url("Team_DNE" if create_fail else team)
        print(f"Sending To: POST:{create_token_url}")
        create_token_rsp = requests.post(
            create_token_url,
            headers={"Authorization": jwt},
            json={"name": "Test Token from e2e"},
        )

        token_json = create_token_rsp.json()
        print(token_json)

        if not create_fail:

            token = token_json["token"]
            delete_url = __get_token_url(
                team, "not_real_token" if delete_fail else token
            )
            print(f"Sending To: DELETE:{delete_url}")
            delete_token_rsp = requests.delete(
                delete_url,
                headers={"Authorization": jwt},
            )

            print(delete_token_rsp.text)


def test_sbom_upload():
    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    working_token = "8d191d16-467e-4150-8416-f51fc7ca1b93"
    made_up_token = "8d191d16-467e-4150-8416-f51fc7ca1b69"
    disabled_token = "8d191d16-467e-4150-8416-f51fc7ca1b94"
    expired_token = "8d191d16-467e-4150-8416-f51fc7ca1b95"

    print("Sending To: %s" % SBOM_UPLOAD_URL)

    good_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={"Authorization": working_token},
    )

    if good_token_rsp.status_code == 200:
        print("Correct token test passed")
    else:
        print(f"Correct token test failed, received: {good_token_rsp.status_code}")
        print(good_token_rsp.text)

    made_up_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={"Authorization": made_up_token},
    )

    if made_up_token_rsp.status_code == 403:
        print("Bad Token test passed")
    else:
        print(f"Bad Token test failed, received: {made_up_token_rsp.status_code}")
        print(made_up_token_rsp.text)

    disabled_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={"Authorization": disabled_token},
    )

    if disabled_token_rsp.status_code == 403:
        print("Disabled Token test passed")
    else:
        print(f"Disabled Token test failed, received: {disabled_token_rsp.status_code}")
        print(disabled_token_rsp.text)

    expired_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={"Authorization": expired_token},
    )

    if expired_token_rsp.status_code == 403:
        print("Expired token test passed")
    else:
        print(f"Expired Token test failed, received: {expired_token_rsp.status_code}")
        print(expired_token_rsp.text)


def test_user_search():

    """
    -> User Search
    """

    jwt = __login()

    user_mar = "mar"
    url = f"{USER_SEARCH_URL}?filter={user_mar}"
    print(f"Sending To: GET:{url}")
    user_search_rsp = requests.get(
        url,
        headers={"Authorization": jwt},
    )

    mar_result = user_search_rsp.json()
    if "maria@aquia.io" in mar_result and "martha@aquia.io" in mar_result:
        print("Passed using 'mar' filter")
    else:
        print("Failed using 'mar' filter")

    user_qui = "qui"
    url = f"{USER_SEARCH_URL}?filter={user_qui}"
    print(f"Sending To: GET:{url}")
    user_search_rsp = requests.get(
        url,
        headers={"Authorization": jwt},
    )

    qui_result = user_search_rsp.json()
    if (
        "quinn@aquia.io" in qui_result
        and "quinton@aquia.io" in qui_result
        and "quison@aquia.io" in qui_result
    ):
        print("Passed using 'qui' filter")
    else:
        print("Failed using 'qui' filter")

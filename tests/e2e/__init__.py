from json import dumps

import boto3
import requests
from requests import Response, delete, get, post

from cyclonedx.clients import HarborDBClient
from cyclonedx.constants import AWS_REGION_SHORT
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team


class TestCodebaseValues:

    """
    -> Class to keep values to use for codebase mocking
    """

    NAME = "Keycloak"
    LANGUAGE = "JAVA"
    BUILD_TOOL = "MAVEN"
    CLONE_URL = "https://github.com/cmsgov/ab2d-lambdas"


def cleanup(team_id: str, team_url: str, jwt: str):

    """
    -> Allows us to clean up after ourselves
    """

    print(f"Sending To: DELETE:{team_url}")
    delete_rsp: Response = delete(
        f"{team_url}/{team_id}?children=true",
        headers={
            "Authorization": jwt,
        },
    )
    print_response(delete_rsp)


def print_response(response: Response):

    """
    -> Uniform response so we know what to look for
    """

    status_code: int = response.status_code
    dump: str = dumps(response.json(), indent=2)
    print(f"Response: ({status_code}) {dump}")


def create_codebase(team_id: str, project_id: str, cf_url: str, jwt: str):

    """
    -> Create a codebase using the HTTP API
    """

    codebase_url: str = f"{cf_url}/api/v1/codebase"
    codebase_url = f"{codebase_url}?teamId={team_id}"
    codebase_url = f"{codebase_url}&projectId={project_id}"
    print(f"Sending To: POST:{codebase_url}")
    create_codebase_rsp: Response = post(
        codebase_url,
        headers={
            "Authorization": jwt,
        },
        json={
            CodeBase.Fields.NAME: TestCodebaseValues.NAME,
            CodeBase.Fields.LANGUAGE: TestCodebaseValues.LANGUAGE,
            CodeBase.Fields.BUILD_TOOL: TestCodebaseValues.BUILD_TOOL,
            CodeBase.Fields.CLONE_URL: TestCodebaseValues.CLONE_URL,
        },
    )
    print_response(create_codebase_rsp)
    codebase_json: dict = create_codebase_rsp.json()
    return codebase_json.get(CodeBase.Fields.ID)


def get_upload_url(cf_url: str, team_id: str, project_id: str, codebase_id: str):

    """
    -> Get the "Upload" Url
    """

    return f"{cf_url}/api/v1/{team_id}/{project_id}/{codebase_id}/sbom"


def get_entity_by_id(
    team_id: str,
    entity_key: str,
    entity_id: str,
    cf_url: str,
    jwt: str,
) -> dict:

    """
    -> Get an entity by the id
    """

    url: str = f"{cf_url}/api/v1/{entity_key}/{entity_id}"
    url = f"{url}?teamId={team_id}"
    get_rsp: Response = get(
        url,
        headers={
            "Authorization": jwt,
        },
    )

    return get_rsp.json()


def get_upload_token(
    cf_url: str,
    jwt: str,
    team_id: str,
):

    """
    -> Get the first upload token in a team
    """

    url: str = f"{cf_url}/api/v1/tokens"
    url = f"{url}?teamId={team_id}"
    get_rsp: Response = get(
        url,
        headers={
            "Authorization": jwt,
        },
    )

    token_json: dict = get_rsp.json()

    return token_json[0]["token"]


def create_team_with_projects(
    team_name: str,
    project_names: list[str],
    team_url: str,
    jwt: str,
) -> dict:

    """
    -> Create a test team with a project for each project name
    """

    # fmt: off
    projects: list[dict] = [
        { Project.Fields.NAME: project_name }
        for project_name in project_names
    ]
    # fmt: on

    request_body: dict = {
        Team.Fields.NAME: team_name,
        "projects": projects,
    }

    team_url_plus_children: str = f"{team_url}?children=true"

    print(f"Sending To: POST:{team_url_plus_children}")
    print(f"Request Body: {request_body}")
    create_rsp: Response = post(
        team_url_plus_children,
        headers={
            "Authorization": jwt,
        },
        json=request_body,
    )
    print_response(create_rsp)

    return create_rsp.json()


def get_team_url(cf_url: str):

    """
    -> Get the "Team" Url
    """

    return f"{cf_url}/api/v1/team"


def login(cf_url: str) -> str:

    """
    -> Gets a JWT so we can make requests
    """

    login_url = f"{cf_url}/api/v1/login"
    user = "sbomadmin@aquia.io"
    password = "L0g1nTe5tP@55!"

    print(f"Sending To: POST:{login_url}, With: {user}, {password}")
    login_rsp = requests.post(
        login_url,
        json={
            "username": user,
            "password": password,
        },
    )

    login_rsp_json = login_rsp.json()
    print(f"Login Response: {dumps(login_rsp_json, indent=2)}")
    return login_rsp_json["token"]


def get_harbor_table_name(environment: str):
    return f"{environment}-HarborTeams-{AWS_REGION_SHORT}"


def get_harbor_client(session: boto3.Session, environment: str):
    resource = session.resource("dynamodb")
    htn: str = get_harbor_table_name(environment)

    print(f"Working with Harbor Table: (> {htn} <)")

    return HarborDBClient(resource, htn)


def get_cloudfront_url(session: boto3.Session, environment: str):

    """
    -> Extracts the CloudFront url using boto3
    """

    client = session.client("cloudfront")
    distributions = client.list_distributions()
    distribution_list = distributions["DistributionList"]

    try:

        sbom_api_distribution = list(
            filter(
                lambda d: "Comment" in d and d["Comment"] == environment,
                distribution_list["Items"],
            ),
        )[0]

        cf_domain_name = sbom_api_distribution["DomainName"]
        cf_url = f"https://{cf_domain_name}"
        print(f"CloudFront url is: {cf_url}")
        return cf_url
    except KeyError:
        ...

"""
-> init file for e2e tests module
"""
from json import dumps
from os import environ

import boto3
import requests
from requests import Response, delete, post

from cyclonedx.model.project import Project
from cyclonedx.model.team import Team


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


def get_cloudfront_url():

    """
    -> Extracts the CloudFront url using boto3
    """

    session = boto3.Session(
        profile_name="sandbox",
    )
    client = session.client("cloudfront")
    distributions = client.list_distributions()
    distribution_list = distributions["DistributionList"]

    try:
        sbom_api_distribution = distribution_list["Items"][0]
        cf_domain_name = sbom_api_distribution["DomainName"]
        cf_url = f"https://{cf_domain_name}"
        print(f"CloudFront url is: {cf_url}")
        return cf_url
    except KeyError:
        ...


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

    print(f"Sending To: POST:{team_url}")
    create_rsp: Response = post(
        team_url,
        headers={
            "Authorization": jwt,
        },
        json={
            Team.Fields.NAME: team_name,
            "projects": projects,
        },
    )
    print_response(create_rsp)

    return create_rsp.json()


def get_team_url(cf_url: str):

    """
    -> Get the "Team" Url
    """

    return f"{cf_url}/api/v1/team"


def get_upload_url(cf_url: str, team_id: str, project_id: str, codebase_id: str):

    """
    -> Get the "Upload" Url
    """

    return f"{cf_url}/api/v1/{team_id}/{project_id}/{codebase_id}/sbom"

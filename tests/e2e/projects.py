"""
-> Module to defile e2e tests that focus on projects
-> These tests will clean up after themselves in DynamoDB
"""
from uuid import uuid4

from requests import Response, get, put

from cyclonedx.model.project import Project
from tests.e2e import (
    cleanup,
    create_team_with_projects,
    get_cloudfront_url,
    login,
    print_response,
)


def test_no_duplicate_projects_on_create():

    """
    -> Verify that creating a team with 2 projects does not duplicate the projects
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    # Create a team with 2 projects
    team_name: str = "1Team1"
    proj1_name: str = "1Project1"
    proj2_name: str = "2Project2"

    team_url: str = f"{cf_url}/api/v1/team"

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

    get_json: dict = get_rsp.json()
    projects: list[dict] = get_json.get("projects")

    # fmt: off
    project_names: list[str] = [
        project.get(Project.Fields.NAME)
        for project in projects
    ]
    # fmt: on

    assert len(project_names) == 2
    assert proj1_name in project_names
    assert proj2_name in project_names

    cleanup(
        team_id=team_id,
        team_url=team_url,
        jwt=jwt,
    )


def test_no_duplicate_projects_on_update():

    """
    -> Verify that creating a team with 2 projects does not duplicate the projects
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    # Create a team with 2 projects
    team_name: str = "1Team1"
    proj1_name: str = "1Project1"
    proj2_name: str = "2Project2"

    team_url: str = f"{cf_url}/api/v1/team"
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

    get_json: dict = get_rsp.json()
    projects: list[dict] = get_json.get("projects")

    new_project_names: list[str] = []
    new_fisma_ids: list[str] = []
    for project in projects:
        npn: str = str(uuid4())
        nfi: str = str(uuid4())
        new_project_names.append(npn)
        new_fisma_ids.append(nfi)
        project[Project.Fields.NAME] = npn
        project[Project.Fields.FISMA] = nfi

    print(f"Sending To: PUT:{team_url}")
    put_rsp: Response = put(
        f"{team_url}/{team_id}?children=true",
        headers={
            "Authorization": jwt,
        },
        json=get_json,
    )
    print_response(put_rsp)

    # fmt: off
    project_names: list[str] = [
        project.get(Project.Fields.NAME)
        for project in put_rsp.json().get("projects")
    ]

    fisma_ids: list[str] = [
        project.get(Project.Fields.FISMA)
        for project in projects
    ]
    # fmt: on

    assert len(project_names) == 2
    assert len(fisma_ids) == 2

    for npn in new_project_names:
        assert npn in project_names

    for nfi in new_fisma_ids:
        assert nfi in fisma_ids

    cleanup(
        team_id=team_id,
        team_url=team_url,
        jwt=jwt,
    )

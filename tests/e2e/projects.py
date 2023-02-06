"""
-> Module to defile e2e tests that focus on projects
-> These tests will clean up after themselves in DynamoDB
"""
from json import dumps
from time import sleep
from uuid import uuid4

from requests import Response, get, put

from cyclonedx.model.project import Project
from tests.e2e import (
    cleanup,
    create_team_with_projects,
    get_cloudfront_url,
    get_entity_by_id,
    get_team_url,
    login,
    print_response,
)


def test_no_duplicate_projects_on_create(session, environment):

    """
    -> Verify that creating a team with 2 projects does not duplicate the projects
    """

    cf_url: str = get_cloudfront_url(session, environment)
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


def test_no_duplicate_projects_on_update(session, environment):

    """
    -> Verify that creating a team with 2 projects does not duplicate the projects
    """

    cf_url: str = get_cloudfront_url(session, environment)
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


def test_update_project(session, environment):

    """
    -> Test updating a project
    """

    cf_url: str = get_cloudfront_url(session, environment)
    jwt: str = login(cf_url)

    # Create a team with 2 projects
    team_name: str = "1Team1"
    proj1_name: str = "1Project1"
    proj2_name: str = "2Project2"

    team_url: str = get_team_url(cf_url)
    create_rsp: dict = create_team_with_projects(
        team_name=team_name,
        project_names=[proj1_name, proj2_name],
        team_url=team_url,
        jwt=jwt,
    )

    team_id: str = create_rsp.get("id")
    projects: list[dict] = create_rsp.get("projects")
    project1, _ = projects

    proj1_id: str = project1.get("id")
    new_proj1_name: str = "NEW PROJECT 1 NAME"
    project1[Project.Fields.NAME] = new_proj1_name

    project_url: str = f"{cf_url}/api/v1/project/{proj1_id}?teamId={team_id}"
    print(f"Sending To: PUT:{project_url}")
    put_rsp: Response = put(
        project_url,
        headers={
            "Authorization": jwt,
        },
        json=project1,
    )
    print_response(put_rsp)

    # There needs to be a sleep here because DynamoDB does not
    # update fast enough to get the new data if it's not.
    sleep(10)

    get_project_rsp: dict = get_entity_by_id(
        team_id=team_id,
        entity_key="project",
        entity_id=proj1_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    assert get_project_rsp.get(Project.Fields.NAME) == new_proj1_name

    print(dumps(create_rsp, indent=2))
    cleanup(
        team_id=team_id,
        team_url=team_url,
        jwt=jwt,
    )

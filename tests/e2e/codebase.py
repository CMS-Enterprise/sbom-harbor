"""
-> Module to house tests for Code Bases
"""
from time import sleep

from requests import put

from cyclonedx.model import HarborModel
from cyclonedx.model.codebase import CodeBase
from tests.e2e import (
    TestCodebaseValues,
    cleanup,
    create_codebase,
    create_team_with_projects,
    get_cloudfront_url,
    get_entity_by_id,
    get_team_url,
    login,
)


def test_codebase_update():

    """
    -> Test updating a codebase
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    create_rsp: dict = create_team_with_projects(
        team_name="test_sbom_ingress Team",
        project_names=["test_get_codebase_response Project"],
        team_url=get_team_url(cf_url),
        jwt=jwt,
    )

    team_id: str = create_rsp.get(HarborModel.Fields.ID)

    projects: list[dict] = create_rsp.get("projects")
    project: dict = projects[0]
    project_id: str = project.get(HarborModel.Fields.ID)

    codebase_id: str = create_codebase(
        team_id=team_id,
        project_id=project_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    codebase_json: dict = get_entity_by_id(
        team_id=team_id,
        entity_key="codebase",
        entity_id=codebase_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    assert codebase_json.get(CodeBase.Fields.NAME) == TestCodebaseValues.NAME
    assert codebase_json.get(CodeBase.Fields.LANGUAGE) == TestCodebaseValues.LANGUAGE
    assert (
        codebase_json.get(CodeBase.Fields.BUILD_TOOL) == TestCodebaseValues.BUILD_TOOL
    )

    new_test_name: str = "UPDATE TEST NAME"
    new_test_language: str = "UPDATE TEST LANGUAGE"
    new_test_build_tool: str = "UPDATE TEST BUILD TOOL"

    codebase_url: str = f"{cf_url}/api/v1/codebase/{codebase_id}"
    codebase_url = f"{codebase_url}?projectId={project_id}"
    codebase_url = f"{codebase_url}&teamId={team_id}"
    put(
        codebase_url,
        headers={
            "Authorization": jwt,
        },
        json={
            CodeBase.Fields.NAME: new_test_name,
            CodeBase.Fields.LANGUAGE: new_test_language,
            CodeBase.Fields.BUILD_TOOL: new_test_build_tool,
        },
    )

    # There needs to be a sleep here because DynamoDB does not
    # update fast enough to get the new data if it's not.
    sleep(10)

    print(f"Running get_codebase_by_id: team_id={team_id}, codebase_id={codebase_id}")
    codebase_json: dict = get_entity_by_id(
        team_id=team_id,
        entity_key="codebase",
        entity_id=codebase_id,
        cf_url=cf_url,
        jwt=jwt,
    )
    print(codebase_json)

    assert codebase_json.get(CodeBase.Fields.NAME) == new_test_name
    assert codebase_json.get(CodeBase.Fields.LANGUAGE) == new_test_language
    assert codebase_json.get(CodeBase.Fields.BUILD_TOOL) == new_test_build_tool

    cleanup(
        team_id=team_id,
        team_url=get_team_url(cf_url),
        jwt=jwt,
    )


def test_get_codebase_response():

    """
    -> Test getting a codebase
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    create_rsp: dict = create_team_with_projects(
        team_name="test_sbom_ingress Team",
        project_names=["test_get_codebase_response Project"],
        team_url=get_team_url(cf_url),
        jwt=jwt,
    )

    team_id: str = create_rsp.get(HarborModel.Fields.ID)

    projects: list[dict] = create_rsp.get("projects")
    project: dict = projects[0]
    project_id: str = project.get(HarborModel.Fields.ID)

    codebase_id: str = create_codebase(
        team_id=team_id,
        project_id=project_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    codebase_get_rsp_json: dict = get_entity_by_id(
        team_id=team_id,
        entity_key="codebase",
        entity_id=codebase_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    assert codebase_get_rsp_json[HarborModel.Fields.ID] == codebase_id

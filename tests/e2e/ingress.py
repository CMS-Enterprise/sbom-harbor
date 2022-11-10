"""
-> Module to test the SBOM Upload
"""
from importlib.resources import files
from json import dumps, loads

from importlib_resources.abc import Traversable
from requests import Response, post

from cyclonedx.model import HarborModel
from cyclonedx.model.codebase import CodeBase
from tests import sboms
from tests.e2e import (
    cleanup,
    create_team_with_projects,
    get_cloudfront_url,
    get_team_url,
    get_upload_url,
    login,
    print_response,
)


def test_sbom_ingress():

    """
    -> Main Test
    """

    sbom_folder: Traversable = files(sboms)
    keycloak_sbom_obj: Traversable = sbom_folder.joinpath("keycloak.json")
    keycloak_sbom: dict = loads(keycloak_sbom_obj.read_text())

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    create_rsp: dict = create_team_with_projects(
        team_name="test_sbom_ingress Team",
        project_names=["test_sbom_ingress Project"],
        team_url=get_team_url(cf_url),
        jwt=jwt,
    )

    team_id: str = create_rsp.get(HarborModel.Fields.ID)

    projects: list[dict] = create_rsp.get("projects")
    project: dict = projects[0]
    project_id: str = project.get(HarborModel.Fields.ID)

    tokens: list[dict] = create_rsp.get("tokens")
    token: dict = tokens[0]
    upload_token: str = token["token"]

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
            CodeBase.Fields.NAME: "Keycloak",
            CodeBase.Fields.LANGUAGE: "JAVA",
            CodeBase.Fields.BUILD_TOOL: "MAVEN",
        },
    )
    print_response(create_codebase_rsp)
    codebase_json: dict = create_codebase_rsp.json()
    codebase_id: str = codebase_json.get(CodeBase.Fields.ID)

    sbom_upload_url: str = get_upload_url(
        cf_url=cf_url,
        team_id=team_id,
        project_id=project_id,
        codebase_id=codebase_id,
    )

    sbom_upload_rsp: Response = post(
        sbom_upload_url,
        headers={
            "Authorization": upload_token,
        },
        json=keycloak_sbom,
    )
    print_response(sbom_upload_rsp)

    cleanup(
        team_id=team_id,
        team_url=get_team_url(cf_url),
        jwt=jwt,
    )

    print("<CreateResponse>")
    print(dumps(create_rsp, indent=2))
    print("</CreateResponse>")

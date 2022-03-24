""" Unit tests """

import os

from botocore.client import BaseClient
from pytest_mock import mocker
from boto3 import client

import cyclonedx.api as api
import tests.sboms as sboms
import importlib.resources as pr
from time import sleep
from json import loads
from requests import Response, get, put
from cyclonedx import core
from cyclonedx.dtendpoints import DTEndpoints


def test_get_schemas() -> None:

    """
    Get Schema Test
    """

    cdx_core = core.CycloneDxCore()
    schema = cdx_core.get_schema("1.2")
    assert schema is not None


def test_store_handler() -> None:

    """
    Store Handler test
    """

    # pr.read_text(sboms, "bom-1.2.schema.json")
    # mock_bom = dumps({"bomFormat": "CycloneDX", "specVersion": "1.4"})
    # mock_event = {"requestContext": "TestContext", "body": mock_bom}
    #
    # cyclonedx.api.store_handler(mock_event, {})


def __upload_bom(bom):

    """
    Testing uploading a bom into DT
    """

    response = api.dt_ingress_handler(bom)
    print(response.text)

    return response.json()


def dt_team():

    """
    Easy DT API test functions to see if it's up
    """

    key = os.getenv("DT_API_KEY")
    headers = {"X-Api-Key": key, "Accept": "application/json"}

    response = get("http://localhost:8081/api/v1/team", headers=headers)
    print(response.text)


def get_findings():

    """
    Gets findings and shows them to you
    """

    uuid = "acd68120-3fec-457d-baaa-a456a39984de"

    end_point = f"http://localhost:8081/api/v1/finding/project/{uuid}"

    key = os.getenv("DT_API_KEY")
    headers = {"X-Api-Key": key, "Accept": "application/json"}
    response = get(end_point, headers=headers)

    print(f"Hitting endpoint: {end_point}")
    print(response.text)


def test_bom_upload_state():

    """
    Uploads an SBOM
    """

    key: str = os.getenv("DT_API_KEY")
    bom: dict = loads(pr.read_text(sboms, "keycloak.json"))
    token_container: dict = __upload_bom(bom)

    # pylint: disable=W0212
    while not api.__findings_ready(key, token_container["token"]):
        sleep(0.5)
        print("Not ready...")

    print("Results are in!")

    end_point = DTEndpoints.get_findings()
    print(f"Hitting endpoint: {end_point}")

    findings = get(end_point)

    print("<findings>")
    print(findings)
    print("</findings>")


def test_create_project():
    create_project_headers: dict = {
        "Accept": "application/json",
        "Content-Type": "application/json",
    }

    create_proj_body = {
        "author": "EnrichmentLambda",
        "version": "1.0.0",
        "classifier": "APPLICATION",
        "description": "auto generated project",
    }

    create_proj_rsp: Response = put(
        DTEndpoints.create_project(),
        headers=create_project_headers,
        data=create_proj_body,
    )

    print(f"Sending request to endpoint: {DTEndpoints.create_project()}")
    print(create_proj_rsp)


def test_extract_api_key():

    # mocker.patch("botocore.client.BaseClient.get_parameter")
    juice_sbom = pr.read_text(sboms, "juice.json")
    rsp = api.dt_ingress_handler(juice_sbom)
    print(rsp)

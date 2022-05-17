""" Unit tests """

import os

import requests
from json import loads, dumps
from boto3.dynamodb.types import TypeDeserializer, TypeSerializer

import cyclonedx.api as api
import cyclonedx.util as util

import tests.sboms as sboms
import importlib.resources as pr
from time import sleep
from json import dumps, loads
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

    response = get(DTEndpoints.get_teams_data(), headers=headers)
    print(response.text)


def get_findings():

    """
    Gets findings and shows them to you
    """

    uuid = "acd68120-3fec-457d-baaa-a456a39984de"

    key = os.getenv("DT_API_KEY")
    headers = {"X-Api-Key": key, "Accept": "application/json"}
    response = get(DTEndpoints.get_findings(uuid), headers=headers)

    print(response.text)


def test_bom_upload_state():

    """
    Uploads an SBOM
    """

    key: str = os.getenv("DT_API_KEY")
    bom: dict = loads(pr.read_text(sboms, "keycloak.json"))
    token_container: dict = __upload_bom(bom)

    # pylint: disable=W0212
    while not util.__findings_ready(key, token_container["token"]):
        sleep(0.5)
        print("Not ready...")

    print("Results are in!")

    end_point = DTEndpoints.get_findings(key)
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


def test_dt_ingress_handler():

    juice_sbom = pr.read_text(sboms, "juice.json")
    juice_sbom = dumps(loads(juice_sbom))
    rsp = api.dt_interface_handler(juice_sbom)
    print(rsp)


def test_upload_bom():
    juice_sbom = pr.read_text(sboms, "juice.json")
    juice_sbom = dumps(loads(juice_sbom))
    token = util.__upload_sbom("2f357abe-954d-4680-b978-60b597a4cd47", juice_sbom)
    print(f"Token: {token}")


def cpe_test():

    """API Explained here: https://nvd.nist.gov/developers/products"""

    cpe_ep = "https://services.nvd.nist.gov/rest/json/cpes/1.0/"

    rsp = requests.get(
        cpe_ep,
        params={
            "apiKey": os.getenv("NVD_API_KEY"),
            "includeDeprecated": False,
            "resultsPerPage": 5,
            "keyword": "adobe",
            # "addOns": "cves",
        },
    )

    print(f"Calling to: {cpe_ep},  Response: {rsp.text}")


def cve_test():

    # Adobe Illustrator versions 25.4.3 (and earlier) and 26.0.2
    # (and earlier) are affected by an out-of-bounds read vulnerability
    # that could lead to disclosure of sensitive memory. An attacker
    # could leverage this vulnerability to bypass mitigations such as ASLR.
    # Exploitation of this issue requires user interaction in that a victim
    # must open a malicious file.
    cve_id = "CVE-2022-23196"

    single_cve_ep = "https://services.nvd.nist.gov/rest/json/cve/1.0/"
    url = f"{single_cve_ep}/{cve_id}"

    rsp = requests.get(url, params={"apiKey": os.getenv("NVD_API_KEY")})

    print(f"Calling to: {url},  Response: {rsp.text}")


def dynamodb_parser_test():
    from os import path
    __cwd = path.dirname(__file__)
    team_fh = open(f"{__cwd}/scripts/team.json", "r")
    json = team_fh.read()

    print("<JSON>")
    print(json)
    print("</JSON>")

    serializer = TypeSerializer()

    ser_json = serializer.serialize(loads(json))

    print("<SerJSON>")
    print(dumps(ser_json))
    print("</SerJSON>")





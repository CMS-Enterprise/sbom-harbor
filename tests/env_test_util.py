"""
-> This module is used to store functions we have written
-> that help test a working AWS environment.
"""

import os
from json import dumps

import boto3
import requests
from requests import Response, get, put

from cyclonedx.constants import (
    S3_META_TEAM_KEY,
    S3_META_PROJECT_KEY,
    S3_META_CODEBASE_KEY,
)
from cyclonedx.dtendpoints import DTEndpoints
from cyclonedx.handlers.cyclonedx_util import ICClient
from cyclonedx.enrichment import des_interface_handler


def test_add_team_to_teams_custom_attribute():

    """Test to add a team id to a custom attribute"""

    client = boto3.client("cognito-idp")
    response = client.admin_get_user(
        UserPoolId="us-east-1_kkIk0EQIa",
        Username="8848f29e-8cb5-4857-a1b4-daaa90566858",
    )

    print(response)

    teams = list(
        filter(lambda o: o["Name"] == "custom:teams", response["UserAttributes"])
    )[0]

    print(teams)

    current_teams = teams["Value"]

    new_teams = f"{current_teams},my-new-team"

    client.admin_update_user_attributes(
        UserPoolId="us-east-1_kkIk0EQIa",
        Username="8848f29e-8cb5-4857-a1b4-daaa90566858",
        UserAttributes=[
            {"Name": "custom:teams", "Value": new_teams},
        ],
    )

    response = client.admin_get_user(
        UserPoolId="us-east-1_kkIk0EQIa",
        Username="8848f29e-8cb5-4857-a1b4-daaa90566858",
    )

    print(response)


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
        timeout=20,
    )

    print(f"Calling to: {cpe_ep},  Response: {rsp.text}")


def cve_test():

    """Test that gets data from NVD"""

    # Adobe Illustrator versions 25.4.3 (and earlier) and 26.0.2
    # (and earlier) are affected by an out-of-bounds read vulnerability
    # that could lead to disclosure of sensitive memory. An attacker
    # could leverage this vulnerability to bypass mitigations such as ASLR.
    # Exploitation of this issue requires user interaction in that a victim
    # must open a malicious file.
    cve_id = "CVE-2022-23196"

    single_cve_ep = "https://services.nvd.nist.gov/rest/json/cve/1.0/"
    url = f"{single_cve_ep}/{cve_id}"

    rsp = requests.get(url, params={"apiKey": os.getenv("NVD_API_KEY")}, timeout=20)

    print(f"Calling to: {url},  Response: {rsp.text}")


def test_put_sbom_in_s3():

    """Puts an SBOM in S3 with the correct metadata fro testing"""

    sbom = "sboms/keycloak.json"
    bucket_name = f"sbom.bucket.{os.environ.get('AWS_ACCOUNT_NUM')}"
    s3_obj_name = "sbom-keycloak.json"

    print(f"Putting: {s3_obj_name} in {bucket_name}")

    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, sbom)

    with open(filename, "rb") as fh:
        s3 = boto3.resource("s3")
        s3.Object(bucket_name, s3_obj_name).put(
            Body=fh,
            Metadata={
                S3_META_TEAM_KEY: "EVAL TestTeam",
                S3_META_PROJECT_KEY: "TestProject",
                S3_META_CODEBASE_KEY: "TestCodebase",
            },
        )


def test_get_analysis_id():

    """A test to get the analysis ID from Ion Channel"""

    try:
        name: str = "TEST STeam-SProject-SCodebase"
        ic_client = ICClient(name, True)
        response = ic_client.get_analysis_id("6c4ce5c9-fcf4-43af-9c8e-f42f06267cd4")

        print(dumps(response, indent=2))

    except ValueError as ex:
        print(f"Problem starting ICClient: {ex}")


def test_nvd_lookup():

    """Tests the Default ES to see if it can get results from NVD"""

    # cpe:2.3: part : vendor : product : version : update : edition :
    #    language : sw_edition : target_sw : target_hw : other
    # https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir7695.pdf

    des_interface_handler(
        {
            "version": "0",
            "id": "53fa0c0f-dbc7-e64b-a003-a5b7e8cdb600",
            "detail-type": "test_detail_type_string",
            "source": "enrichment.lambda_objects",
            "account": "531175407938",
            "time": "2022-06-30T07:36:07Z",
            "region": "us-east-1",
            "resources": [],
            "detail": {
                "sbom_bucket": "sbom.bucket.531175407938",
                "sbom_s3_key": "keycloak.json",
            },
        }
    )


def dt_team():

    """
    Easy DT API test functions to see if it's up
    """

    key = os.getenv("DT_API_KEY")
    headers = {"X-Api-Key": key, "Accept": "application/json"}

    response = get(
        DTEndpoints.get_teams_data(),
        headers=headers,
        timeout=20,
    )
    print(response.text)


def get_findings():

    """
    Gets findings and shows them to you
    """

    uuid = "acd68120-3fec-457d-baaa-a456a39984de"

    key = os.getenv("DT_API_KEY")
    headers = {"X-Api-Key": key, "Accept": "application/json"}
    response = get(
        DTEndpoints.get_findings(uuid),
        headers=headers,
        timeout=20,
    )

    print(response.text)


def test_create_project():

    """Tests creating a project"""

    create_project_headers: dict = {
        "Accept": "application/json",
        "Content-Type": "application/json",
    }

    create_proj_body: dict = {
        "author": "EnrichmentLambda",
        "version": "1.0.0",
        "classifier": "APPLICATION",
        "description": "auto generated project",
    }

    create_proj_rsp: Response = put(
        DTEndpoints.create_project(),
        headers=create_project_headers,
        data=create_proj_body,
        timeout=20,
    )

    print(f"Sending request to endpoint: {DTEndpoints.create_project()}")
    print(create_proj_rsp)

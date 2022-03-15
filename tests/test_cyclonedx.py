""" Unit tests """

import os
from json import loads
import importlib.resources as pr
from requests import get
import cyclonedx.api as api
import tests.sboms as sboms
from cyclonedx import core


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


def dt_test():

    """
    Testing uploading a bom into DT
    """

    bom = loads(pr.read_text(sboms, "keycloak.json"))
    response = api.dt_ingress_handler(bom)
    print(response.text)


def dt_team():

    """
    Easy DT API test functions to see if it's up
    """

    key = os.getenv("DT_API_KEY")
    headers = {"X-Api-Key": key, "Accept": "application/json"}

    response = get("http://localhost:8081/api/v1/team", headers=headers)
    print(response.text)

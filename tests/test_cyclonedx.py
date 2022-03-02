import cyclonedx.api
import importlib.resources as pr
import tests.sboms as sboms
from cyclonedx import core
from json import dumps


def test_get_schemas() -> None:
    cdx_core = core.CycloneDxCore()
    schema = cdx_core.get_schema("1.2")
    assert schema is not None


def test_store_handler() -> None:
    pr.read_text(sboms, "bom-1.2.schema.json")
    mock_bom = dumps({"bomFormat": "CycloneDX", "specVersion": "1.4"})
    mock_event = {"requestContext": "TestContext", "body": mock_bom}

    cyclonedx.api.store_handler(mock_event, {})

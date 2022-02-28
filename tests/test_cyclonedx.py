from cyclonedx import core


def test_get_schemas() -> None:
    cdx_core = core.CycloneDxCore()
    schema = cdx_core.get_schema("1.2")
    assert schema != None

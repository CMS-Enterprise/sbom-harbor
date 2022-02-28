import importlib.resources as pr
import cyclonedx.schemas as schemas 
from jsonschema import validate
from jsonschema.exceptions import ValidationError
from json import loads

class CycloneDxCore:
    
    def __init__(self):
        self.sbom_schemas = { 
            "1.2": pr.read_text(schemas, "bom-1.2.schema.json"),
            "1.3": pr.read_text(schemas, "bom-1.3.schema.json"),
            "1.4": pr.read_text(schemas, "bom-1.4.schema.json")
        }

    def get_schema(self, version) -> str:
        return self.sbom_schemas[version]
    
    def validate(self, bom_obj: dict) -> None:

        try:
            bom_format = bom_obj["bomFormat"]
        except KeyError:
            raise ValidationError('Missing "bomFormat" key, is this a BOM you are trying to send?')

        try:
            schema_version = bom_obj["specVersion"]
        except KeyError:
            raise ValidationError('Missing "specVersion" key, is this a BOM you are trying to send?')


        if bom_format != "CycloneDX":
            raise ValidationError("SBOM-API only supports CycloneDX SBOM Formats")

        if schema_version not in self.sbom_schemas:
            raise ValidationError("CycloneDX Schema Version %s is not supported" % schema_version)

        schema_json = self.sbom_schemas[schema_version]
        schema_obj = loads(schema_json)

        validate(instance=bom_obj, schema=schema_obj)
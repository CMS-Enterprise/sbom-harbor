import importlib.resources as pr
import cyclonedx.schemas as schemas 
from jsonschema import validate
from jsonschema.exceptions import ValidationError
from json import loads

class CycloneDxCore:
    
    def __get_value(self, key, bom_obj):
        try:
            return bom_obj[key]
        except KeyError:
            raise ValidationError('Missing "%s" key, is this a BOM you are trying to send?' % key)
        

    def __init__(self):
        self.sbom_schemas = { 
            "1.2": pr.read_text(schemas, "bom-1.2.schema.json"),
            "1.3": pr.read_text(schemas, "bom-1.3.schema.json"),
            "1.4": pr.read_text(schemas, "bom-1.4.schema.json")
        }

    def get_schema(self, version) -> str:
        return self.sbom_schemas[version]
    
    def validate(self, bom_obj: dict) -> None:

        bom_format = self.__get_value('bomFormat', bom_obj)
        if bom_format != "CycloneDX":
            raise ValidationError("SBOM-API only supports CycloneDX SBOM Formats")

        schema_version = self.__get_value('specVersion', bom_obj)
        if schema_version not in self.sbom_schemas:
            raise ValidationError("CycloneDX Schema Version %s is not supported" % schema_version)

        schema_json = self.sbom_schemas[schema_version]
        schema_obj = loads(schema_json)

        validate(instance=bom_obj, schema=schema_obj)
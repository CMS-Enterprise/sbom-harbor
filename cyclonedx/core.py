import importlib.resources as pr
import cyclonedx.schemas as schemas 
from jsonschema import validate
from jsonschema.exceptions import ValidationError
from json import loads


class CycloneDxCore:
    
    """
    This is the main class of the CycloneDx Python Core Library
    """

    @staticmethod
    def __get_value(key: str, bom_obj: dict) -> str:
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

    def get_schema(self, version: str) -> str:

        """
        Test function.  Remove after we have some real unit tests
        """

        return self.sbom_schemas[version]
    
    def validate(self, bom_obj: dict) -> None:

        """
        This function validates the incoming SBOM against the supplied schema.
        The jsonschema.validate() method raises ValidationError and returns a 
        very nice error if the schema is invalid.  I also raise the same error
        during the bom format and schema version pre-checks.  This makes an easy
        interface to return errors back to the caller. 
        """

        # Verify that the bom we are receiving is CycloneDX and not another format.
        bom_format = self.__get_value('bomFormat', bom_obj)
        if bom_format != "CycloneDX":
            raise ValidationError("SBOM-API only supports CycloneDX SBOM Formats")

        # Verify that the user is sending us a version of the CycloneDX
        schema_version = self.__get_value('specVersion', bom_obj)
        if schema_version not in self.sbom_schemas:
            raise ValidationError("CycloneDX Schema Version %s is not supported" % schema_version)

        schema_json = self.sbom_schemas[schema_version]
        schema_obj = loads(schema_json)

        validate(instance=bom_obj, schema=schema_obj)

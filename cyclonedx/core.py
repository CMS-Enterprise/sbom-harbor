import importlib.resources as pr
import cyclonedx.schemas as schemas 


class CycloneDxCore:
    
    def __init__(self):
        self.sbom_schemas = { 
            "1.2": pr.read_text(schemas, "bom-1.2.schema.json"),
            "1.3": pr.read_text(schemas, "bom-1.3.schema.json"),
            "1.4": pr.read_text(schemas, "bom-1.4.schema.json")
        }

    def get_schema(self, version) -> str:
        return self.sbom_schemas[version]
    
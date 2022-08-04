from cyclonedx.core_utils import (
    ICClient
)
from cyclonedx.core_utils.cyclonedx_util import (
    __get_all_s3_obj_data
)


def ic_interface_handler(event: dict = None, context: dict = None):

    """
    Ion Channel Ingress Handler
    This code takes an SBOM in the S3 Bucket and submits it to Ion Channel
    to get findings.
    """

    all_data = __get_all_s3_obj_data(event)
    sbom_str_file = all_data['data']
    team = all_data['team']
    project = all_data['project']
    codebase = all_data['codebase']

    # Here is where the SBOM name, or the Ion Channel Team Name
    # is created.
    sbom_name: str = f"{team}-{project}-{codebase}"

    # Create an Ion Channel Request Factory
    # and import the SBOM
    ic_client = ICClient(sbom_name, True)
    ic_client.import_sbom(sbom_str_file)
    ic_client.analyze_sbom()

    return sbom_name

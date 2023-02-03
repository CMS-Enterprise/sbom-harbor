"""
-> Ion Channel Interface Handler
"""
from json import dumps

from boto3 import resource

from cyclonedx.clients import IonChannelClient
from cyclonedx.constants import SBOM_BUCKET_NAME_KEY, SBOM_S3_KEY
from cyclonedx.handlers.common import _get_sbom, print_values


def ic_interface_handler(event: dict = None, context: dict = None):

    """
    -> Ion Channel Ingress Handler
    -> This code takes an SBOM in the S3 Bucket and submits it to Ion Channel
    -> to get findings.
    """

    print_values(event, context)

    s3_resource = resource("s3")

    # EventBridge 'detail' key has the data we need.
    bucket_name: str = event[SBOM_BUCKET_NAME_KEY]
    key: str = event[SBOM_S3_KEY]

    all_data = _get_sbom(event)
    sbom_str_file = all_data["data"]
    team = all_data["team"]
    project = all_data["project"]
    codebase = all_data["codebase"]

    # Here is where the SBOM name, or the Ion Channel Team Name
    # is created.
    sbom_name: str = f"{team}-{project}-{codebase}"

    try:

        # Create an Ion Channel Request Factory
        # and import the SBOM
        ic_client = IonChannelClient(sbom_name, True)

        # If the SBOM already exists, then the projects must
        # be archived, or they will be duplicated in Ion Channel
        if ic_client.already_exists:
            ic_client.archive_existing_projects()

        ic_client.import_sbom(sbom_str_file)
        ic_client.monitor_sbom_analysis()

        report: dict = ic_client.get_report()

        findings_object: dict = {
            # This key is necessary to set the name of the
            # folder under the summarized data folder
            "meta": {
                "application": "Ion-Channel",
            },
            # Without this key, the summarizer will break
            "project": {},
            # Here is the actual data
            "report": report,
        }

    except Exception as e:

        findings_object: dict = {
            # This key is necessary to set the name of the
            # folder under the summarized data folder
            "meta": {
                "application": "Ion-Channel",
                "error": "ION CHANNEL FAILURE: timeout"
            },
            # Without this key, the summarizer will break
            "project": {},
            # Here is the actual data
            "report": {},
        }

    findings_bytes = bytearray(dumps(findings_object), "utf-8")
    findings_key: str = f"findings-ic-{key}"
    s3_resource.Object(bucket_name, findings_key).put(
        Body=findings_bytes,
    )

    return findings_key

"""
-> Module for the Dependency Track Interface Handler
"""
from io import StringIO
from json import dumps

from boto3 import resource

from cyclonedx.constants import SBOM_BUCKET_NAME_KEY, SBOM_S3_KEY
from cyclonedx.handlers.dependency_track import (
    __create_project,
    __delete_project,
    __get_findings,
    __upload_sbom,
    __validate,
)


def dt_interface_handler(event: dict = None, context: dict = None):

    """Dependency Track Ingress Handler
    This code takes an SBOM in the S3 Bucket and submits it to Dependency Track
    to get findings.  To accomplish this, a project must be created in DT, the
    SBOM submitted under that project, then the project is deleted.
    """

    s3_resource = resource("s3")

    print(f"<Event event='{event}' />")

    # Currently making sure it isn't empty
    __validate(event)

    # EventBridge 'detail' key has the data we need.
    bucket_name = event[SBOM_BUCKET_NAME_KEY]
    key: str = event[SBOM_S3_KEY]

    # Get the SBOM from the bucket and stick it
    # into a string based file handle.
    s3_object = s3_resource.Object(bucket_name, key).get()
    sbom: bytes = s3_object["Body"].read()
    d_sbom: str = sbom.decode("utf-8")
    bom_str_file: StringIO = StringIO(d_sbom)

    # Create a new Dependency Track Project to analyze the SBOM
    project_uuid = __create_project()

    # Upload the SBOM to DT into the temp project
    sbom_token: str = __upload_sbom(project_uuid, bom_str_file)

    # Poll DT to see when the SBOM is finished being analyzed.
    # When it's finished, get the findings returned from DT.
    findings: dict = __get_findings(project_uuid, sbom_token)

    # Clean up the project we made to do the processing
    __delete_project(project_uuid)

    # Dump the findings into a byte array and store them
    # in the S3 bucket along with the SBOM the findings
    # came from.
    findings_bytes = bytearray(dumps(findings), "utf-8")
    findings_key: str = f"findings-dt-{key}"
    s3_resource.Object(bucket_name, findings_key).put(
        Body=findings_bytes,
    )

    print(f"Findings are in the s3 bucket: {bucket_name}/{findings_key}")

    return findings_key

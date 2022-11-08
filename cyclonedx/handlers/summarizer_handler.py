"""This handler takes findings and it's associated SBOM,
and then outputs flattened versions of the files"""
import datetime
from json import dumps, loads

import boto3
from boto3 import resource
from json_normalize import json_normalize

from cyclonedx.clients.db.dynamodb import HarborDBClient
from cyclonedx.constants import (
    S3_META_CODEBASE_KEY,
    S3_META_PROJECT_KEY,
    S3_META_TEAM_KEY,
    S3_META_TIMESTAMP_KEY,
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY,
)
from cyclonedx.model.project import Project


class FileTypes:
    """Simple class to track output file types"""

    sbom = "SBOM"
    findings = "FINDINGS"


def summarizer_handler(event: dict = None, context: dict = None):
    """This handler retrieves the findings file and associated SBOM from the S3 bucket,
     adds some metadata,
    combines them, and then flattens them into a single file that uses dot notation.
     EX: field.subfield.item
    The newly created flattened file is then placed into the S3 bucket with the
    naming scheme of:
    harbor-sbom_name-report-company_name-FISMA_ID-submit_date
    """

    findings_data: list = []
    bucket_name: str = ""
    sbom_name: str = ""

    original_sbom: dict = {}
    original_sbom_metadata: dict = {}

    s3: resource() = resource("s3")

    for result in event:

        if not bucket_name:
            bucket_name = result[SBOM_BUCKET_NAME_KEY]

        if not sbom_name:
            sbom_name = result[SBOM_S3_KEY]

        if not original_sbom:
            sbom_object = get_object_from_s3(s3, bucket_name, sbom_name)
            original_sbom = loads(sbom_object["Body"].read().decode("utf-8"))
            original_sbom_metadata = sbom_object["Metadata"]

        results = result["results"]
        findings_s3_payload = results["Payload"]

        s3_object = get_object_from_s3(s3, bucket_name, findings_s3_payload)
        findings_string = s3_object["Body"].read().decode("utf-8")
        findings_s3_json = loads(findings_string)

        add_metadata_to_finding(findings_s3_json, original_sbom_metadata)

        findings_data.append(findings_s3_json)

    date_folder_path: str = generate_date_folder_path()

    # Normalize the findings for each format and generate a separate file for each
    for finding in findings_data:
        application_name: str = "Unknown"
        finding_metadata = finding.get("meta", "")
        if finding_metadata:
            application_name = finding_metadata.get("application", "Unknown")

        s3.Object(
            bucket_name,
            generate_report_filename(
                application_name, original_sbom_metadata, date_folder_path
            ),
        ).put(
            Body=bytearray(
                dumps(list(json_normalize(finding, combine_lists="chain"))), "utf-8"
            ),
        )

    sbom_data_normalized = json_normalize(original_sbom, combine_lists="chain")
    s3.Object(
        bucket_name,
        generate_report_filename(
            original_sbom.get("bomFormat", "Unknown"),
            original_sbom_metadata,
            date_folder_path,
        ),
    ).put(
        Body=bytearray(dumps(list(sbom_data_normalized)), "utf-8"),
    )


def generate_report_filename(data_type: str, metadata: dict, date_folder_path: str):

    """Creates a filename for the report following the format of:
    harbor-data-summary-{data_type}-{project}-{project_model.fisma}-{timestamp}"""
    db_client: HarborDBClient = HarborDBClient(boto3.resource("dynamodb"))

    timestamp = metadata.get(S3_META_TIMESTAMP_KEY, "")
    project = metadata.get(S3_META_PROJECT_KEY, "")
    team = metadata.get(S3_META_TEAM_KEY, "")

    project_model: Project = db_client.get(
        Project(
            team_id=team,
            project_id=project,
        )
    )

    return (
        f"harbor-data-summary/{data_type}/{date_folder_path}/{project}"
        f"-{project_model.fisma}-{timestamp}"
    )


def generate_date_folder_path() -> str:
    """
    -> Creates the folders for the year, month, and day to be attached
    -> to the filename to help with structuring the data
    """
    date = datetime.datetime.now()
    return f"{date.strftime('%Y')}/{date.strftime('%m')}/{date.strftime('%d')}"


def get_object_from_s3(s3: resource, bucket_name: str, key: str) -> dict:
    """helper for duplicated code"""
    return s3.Object(bucket_name, key).get()


def add_metadata_to_finding(finding_json: dict, metadata: dict):
    """adds metadata to the findings"""
    finding_json["project"]["name"] = metadata[S3_META_PROJECT_KEY]
    finding_json["project"]["team"] = metadata[S3_META_TEAM_KEY]
    finding_json["project"]["codebase"] = metadata[S3_META_CODEBASE_KEY]

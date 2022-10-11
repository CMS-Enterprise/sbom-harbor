from datetime import datetime
from json import (
    loads,
    dumps
)

from boto3 import resource
from json_normalize import json_normalize

from cyclonedx.constants import (
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY
)


def summarizer_handler(event: dict = None, context: dict = None):
    """ This handler retrieves the findings file and associated SBOM from the S3 bucket, adds some metadata,
     combines them, and then flattens them into a single file that uses dot notation. EX: field.subfield.item
     The newly created flattened file is then placed into the S3 bucket with the naming scheme of:
     harbor-sbom_name-report-company_name-FISMA_ID-submit_date
      """
    compiled_results = []
    bucket_name = None
    sbom_name = None

    original_sbom = None
    original_sbom_metadata = None

    s3 = resource("s3")

    for result in event:

        if bucket_name is None:
            bucket_name = result[SBOM_BUCKET_NAME_KEY]

        if sbom_name is None:
            sbom_name = result[SBOM_S3_KEY]

        if original_sbom is None:
            sbom_object = get_object_from_s3(s3, bucket_name, sbom_name)
            original_sbom = sbom_object["Body"].read().decode("utf-8")
            original_sbom_metadata = sbom_object["Metadata"]

        results = result["results"]
        findings_s3_payload = results["Payload"]

        s3_object = get_object_from_s3(s3, bucket_name, findings_s3_payload)
        findings_string = s3_object["Body"].read().decode("utf-8")
        findings_s3_json = loads(findings_string)

        add_metadata_to_finding(findings_s3_json, original_sbom_metadata)

        compiled_results.append(findings_s3_json)

    compiled_results.append(loads(original_sbom))

    # The normalizer field combine_lists takes the values "chain" or "product".
    # "product" may be better, but it is causing memory problems (locally at least)
    normalized_results = json_normalize(compiled_results, combine_lists="chain")

    s3.Object(bucket_name, generate_report_filename(original_sbom_metadata)).put(
        Body=bytearray(dumps(list(normalized_results)), "utf-8"),
    )


def generate_report_filename(metadata: dict):

    # get timestamp value and convert to date time if it exists
    submit_date = ""
    timestamp = metadata.get('x-amz-meta-sbom-api-timestamp', "")
    if timestamp:
        submit_date = datetime.fromtimestamp(float(timestamp)).isoformat()

    project = metadata.get('x-amz-meta-sbom-api-project', "")

    # The customer requested the default for this field should be marked as "unknown"
    fisma = metadata.get('x-amz-meta-sbom-api-fisma', "unknown")

    return f"harbor-data-summary-{project}-{fisma}-{submit_date}"


def get_object_from_s3(s3: resource, bucket_name: str, key: str) -> dict:
    """helper for duplicated code"""
    return s3.Object(bucket_name, key).get()


def add_metadata_to_finding(finding_json: dict, metadata: dict):
    """adds metadata to the findings"""
    finding_json["project"]["name"] = metadata["x-amz-meta-sbom-api-project"]
    finding_json["project"]["team"] = metadata["x-amz-meta-sbom-api-team"]
    finding_json["project"]["codebase"] = metadata["x-amz-meta-sbom-api-codebase"]
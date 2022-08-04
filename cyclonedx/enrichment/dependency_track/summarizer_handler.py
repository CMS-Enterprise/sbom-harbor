from json import loads, dumps

from boto3 import resource

from cyclonedx.constants import (
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY
)


def summarizer_handler(event: dict = None, context: dict = None):

    compiled_results = []

    ( bucket_name, findings_report_name ) = ( "", "" )

    s3 = resource("s3")
    for result in event:

        bucket_name = result[SBOM_BUCKET_NAME_KEY]
        sbom_name = result[SBOM_S3_KEY]
        findings_report_name = f"report-{sbom_name}"

        results = result["results"]
        findings_s3_obj = results["Payload"]

        s3_obj_wrapper = s3.Object(bucket_name, findings_s3_obj)
        s3_object: dict = s3_obj_wrapper.get()
        sbom = s3_object["Body"].read()
        d_sbom = sbom.decode("utf-8")
        compiled_results.append(loads(d_sbom))

    s3.Object(bucket_name, findings_report_name).put(
        Body=bytearray(dumps(compiled_results), "utf-8"),
    )

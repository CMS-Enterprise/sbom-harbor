"""
This module serves as the external API for CycloneDX Python Module
"""

from io import StringIO
from json import dumps
from os import environ
from uuid import uuid4

from boto3 import client, resource
from botocore.exceptions import ClientError
from jsonschema.exceptions import ValidationError

from cyclonedx.constants import (
    DT_QUEUE_URL_EV,
    ENRICHMENT_ID_SQS_KEY,
    ENRICHMENT_ID,
    FINDINGS_QUEUE_URL_EV,
    FINDINGS_SQS_KEY,
    SBOM_BUCKET_NAME_EV,
)

from cyclonedx.core import CycloneDxCore
from cyclonedx.util import (
    __create_project,
    __create_response_obj,
    __delete_project,
    __generate_sbom_api_token,
    __get_body_from_event,
    __get_body_from_event_dt, __get_findings,
    __get_records_from_event,
    __upload_sbom,
    __validate,
)

import http.client as http_client
import logging

# Debug logging
# http_client.HTTPConnection.debuglevel = 1
# logging.basicConfig()
# logging.getLogger().setLevel(logging.DEBUG)
# req_log = logging.getLogger("requests.packages.urllib3")
# req_log.setLevel(logging.DEBUG)
# req_log.propagate = True


def pristine_sbom_ingress_handler(event, context) -> dict:

    """
    This is the Lambda Handler that validates an incoming SBOM
    and if valid, puts the SBOM into the S3 bucket associated
    to the application.
    """

    bom_obj = __get_body_from_event(event)

    s3 = resource("s3")

    # Get the bucket name from the environment variable
    # This is set during deployment
    bucket_name = environ[SBOM_BUCKET_NAME_EV]
    print(f"Bucket name from env(SBOM_BUCKET_NAME_EV): {bucket_name}")

    # Generate the name of the object in S3
    key = f"sbom-{uuid4()}"
    print(f"Putting object in S3 with key: {key}")

    # Create an instance of the Python CycloneDX Core
    core = CycloneDxCore()

    # Create a response object to add values to.
    response_obj = __create_response_obj(bucket_name, key)

    try:

        # Validate the BOM here
        core.validate(bom_obj)

        # Actually put the object in S3
        metadata = {
            # TODO This needs to come from the client
            #   To get this token, there needs to be a Registration process
            #   where a user can get the token and place it in their CI/CD
            #   systems.
            ENRICHMENT_ID: __generate_sbom_api_token()
        }

        # Extract the actual SBOM.
        bom_bytes = bytearray(dumps(bom_obj), "utf-8")
        s3.Object(bucket_name, key).put(
            Body=bom_bytes,
            Metadata=metadata,
        )

    except ValidationError as validation_error:
        response_obj["statusCode"] = 400
        response_obj["body"] = str(validation_error)

    return response_obj


def enrichment_ingress_handler(event=None, context=None):

    """
    Handler that listens for S3 put events and routes the SBOM
    to the enrichment code
    """

    s3 = resource("s3")
    sqs_client = client("sqs")

    if not event:
        raise ValidationError("event should never be none")

    records: list = __get_records_from_event(event)

    print(f"<Records records={records}>")

    queue_url = environ[DT_QUEUE_URL_EV]
    for record in records:

        s3_obj = record["s3"]
        bucket_obj = s3_obj["bucket"]
        bucket_name = bucket_obj["name"]
        sbom_obj = s3_obj["object"]
        key: str = sbom_obj["key"]  # TODO The key name needs to be identifiable

        if key.startswith("sbom"):

            s3_object = s3.Object(bucket_name, key).get()

            try:
                enrichment_id = s3_object["Metadata"][ENRICHMENT_ID]
            except KeyError as key_err:
                print("<s3Object>")
                print(s3_object)
                print("</s3Object>")
                enrichment_id = "ERROR"

            try:
                sqs_client.send_message(
                    QueueUrl=queue_url,
                    MessageAttributes={
                        ENRICHMENT_ID_SQS_KEY: {
                            "DataType": "String",
                            "StringValue": enrichment_id,
                        }
                    },
                    MessageGroupId="dt_enrichment",
                    MessageBody=dumps({
                        "bucket_name": bucket_name,
                        "obj_key": key
                    }),
                )
            except ClientError:
                print(f"Could not send message to the - {queue_url}.")
                raise
        else:
            print(f"Non-BOM{key} added to the s3 bucket.  Don't care.")


def dt_interface_handler(event=None, context=None):

    """
    Developing Dependency Track Ingress Handler
    """

    s3 = resource("s3")

    # Currently making sure it isn't empty
    __validate(event)
    s3_info = __get_body_from_event_dt(event)
    bucket_name = s3_info["bucket_name"]

    s3_object = s3.Object(bucket_name, s3_info["obj_key"]).get()
    sbom = s3_object["Body"].read()
    d_sbom = sbom.decode('utf-8')
    bom_str_file: StringIO = StringIO(d_sbom)
    project_uuid = __create_project()

    sbom_token: str = __upload_sbom(project_uuid, bom_str_file)
    findings: dict = __get_findings(project_uuid, sbom_token)
    
    __delete_project(project_uuid)

    # Extract the actual SBOM.
    findings_bytes = bytearray(dumps(findings), "utf-8")
    findings_key: str = f"findings-{s3_info['obj_key']}"
    s3.Object(bucket_name, findings_key).put(
        Body=findings_bytes,
    )

    print(f"Findings are in the s3 bucket: {bucket_name}/{findings_key}")

    return True

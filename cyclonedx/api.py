"""
This module serves as the external API for CycloneDX Python Module
"""

from io import StringIO
from json import dumps, loads
from os import environ
from uuid import uuid4

import requests
from boto3 import client, resource
from botocore.exceptions import ClientError
from jsonschema.exceptions import ValidationError

from cyclonedx.constants import (
    DT_QUEUE_URL_EV,
    ENRICHMENT_ID_SQS_KEY,
    ENRICHMENT_ID, FINDINGS_QUEUE_URL_EV, FINDINGS_SQS_KEY, SBOM_BUCKET_NAME_EV,
)

from cyclonedx.core import CycloneDxCore
from cyclonedx.dtendpoints import DTEndpoints
from cyclonedx.util import (
    __create_project,
    __create_response_obj,
    __delete_project,
    __generate_sbom_api_token,
    __get_body_from_event,
    __get_findings,
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

    event_obj: dict = loads(event) if event is str else event

    print("<EventObject>")
    print(event_obj)
    print("</EventObject>")

    queue_url = environ[DT_QUEUE_URL_EV]
    for record in event_obj["Records"]:

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

            sbom = s3_object["Body"].read()

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
                    MessageBody=str(sbom),
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

    print(f"<First Thing REST call: get({DTEndpoints.get_dt_version()})")
    try:
        rsp = requests.get(DTEndpoints.get_dt_version(), timeout=5)
        print(f"</First Thing REST call: get({rsp.text})")
    except Exception as exception:
        print(f"</First Thing REST call EXCEPTION: get({exception})")

    # Currently making sure it isn't empty
    __validate(event)
    sbom = __get_body_from_event(event)

    # Get the SBOM in a contrived file handle
    # bom_str_file: StringIO = __get_bom_from_event(event)
    bom_str_file: StringIO = StringIO(dumps(sbom))

    project_uuid = __create_project()

    print("<ProjectCreated -> UUID>")
    print(project_uuid)
    print("</ProjectCreated -> UUID>")

    sbom_token: str = __upload_sbom(project_uuid, bom_str_file)
    findings: dict = __get_findings(project_uuid, sbom_token)
    __delete_project(project_uuid)

    queue_url = environ[FINDINGS_QUEUE_URL_EV]

    sqs_client = client("sqs")

    try:
        sqs_client.send_message(
            QueueUrl=queue_url,
            MessageGroupId="dt_enrichment",
            MessageBody=str(findings),
        )
    except ClientError:
        print(f"Could not send message to the - {queue_url}.")
        raise

    return findings


def enrichment_egress_handler(event=None, context=None):

    s3 = resource("s3")

    findings = __get_body_from_event(event)

    bucket_name = environ[SBOM_BUCKET_NAME_EV]

    key = f"findings-{uuid4()}"

    # Extract the actual SBOM.
    findings_bytes = bytearray(dumps(findings), "utf-8")
    s3.Object(bucket_name, key).put(
        Body=findings_bytes
    )

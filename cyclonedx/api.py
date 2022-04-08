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
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY,
)

from cyclonedx.core import CycloneDxCore
from cyclonedx.util import (
    __create_project,
    __create_response_obj,
    __delete_project,
    __generate_sbom_api_token,
    __get_body_from_event,
    __get_body_from_first_record,
    __get_findings,
    __get_records_from_event,
    __upload_sbom,
    __validate,
)


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
    bucket_name = environ[SBOM_BUCKET_NAME_KEY]
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
        key: str = sbom_obj["key"]

        if key.startswith("sbom"):

            s3_object = s3.Object(bucket_name, key).get()

            try:
                enrichment_id = s3_object["Metadata"][ENRICHMENT_ID]
            except KeyError as key_err:
                print(f"<s3Object object={s3_object} />")
                enrichment_id = f"ERROR: {key_err}"

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
                    MessageBody=dumps(
                        {
                            SBOM_BUCKET_NAME_KEY: bucket_name,
                            SBOM_S3_KEY: key,
                        }
                    ),
                )
            except ClientError:
                print(f"Could not send message to the - {queue_url}.")
                raise
        else:
            print(f"Non-BOM{key} added to the s3 bucket.  Don't care.")


def dt_interface_handler(event=None, context=None):

    """
    Dependency Track Ingress Handler
    This code takes an SBOM in the S3 Bucket and submits it to Dependency Track
    to get findings.  To accomplish this, a project must be created in DT, the
    SBOM submitted under that project, then the project is deleted.
    """

    s3 = resource("s3")

    # Currently making sure it isn't empty
    __validate(event)

    # Extract the body from the first Record in the event.
    # it will contain the S3 Bucket name and the key to
    # the SBOM in the bucket.
    s3_info = __get_body_from_first_record(event)
    bucket_name = s3_info[SBOM_BUCKET_NAME_KEY]
    key: str = s3_info[SBOM_S3_KEY]

    # Get the SBOM from the bucket and stick it
    # into a string based file handle.
    s3_object = s3.Object(bucket_name, key).get()
    sbom = s3_object["Body"].read()
    d_sbom = sbom.decode("utf-8")
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
    findings_key: str = f"findings-{s3_info['obj_key']}"
    s3.Object(bucket_name, findings_key).put(
        Body=findings_bytes,
    )

    print(f"Findings are in the s3 bucket: {bucket_name}/{findings_key}")

    return True

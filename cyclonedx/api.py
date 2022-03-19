"""
This module serves as the external API for CycloneDX Python Module
"""
from os import getenv, environ
from io import StringIO
from time import sleep

from uuid import uuid4
from json import loads, dumps
from requests import post, get, Response
from boto3 import resource, client
from botocore.exceptions import ClientError
from jsonschema.exceptions import ValidationError
from requests_toolbelt.multipart.encoder import MultipartEncoder
from cyclonedx.core import CycloneDxCore
from cyclonedx.dtendpoints import DTEndpoints, PROJECT_UUID
from cyclonedx.constants import SBOM_BUCKET_NAME_EV, DT_TOKEN_KEY, DT_QUEUE_URL_EV

s3 = resource("s3")
sqs = resource("sqs")
sqs_client = client("sqs")


def __generate_token() -> str:
    return f"sbom-api-token-{uuid4()}"


def __get_bom_obj(event) -> dict:

    """
    If the request context exists, then there will
    be a 'body' key, and it will contain the JSON object
    as a **string** that the POST body contained.
    """

    return loads(event["body"]) if "requestContext" in event else event


def __create_response_obj(bucket_name: str, key: str) -> dict:

    """
    Creates a dict that is used as the response from the Lambda
    call.  It has all the necessary elements to satisfy AWS's criteria.
    """

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(
            {
                "valid": True,
                "s3BucketName": bucket_name,
                "s3ObjectKey": key,
            }
        ),
    }


def __findings_ready(key: str, token: str) -> bool:

    headers = {
        "X-Api-Key": key,
        "Accept": "application/json",
    }

    response = get(
        DTEndpoints.get_sbom_status(token),
        headers=headers,
    )

    json_dict = response.json()

    return not json_dict["processing"]


def __get_findings(key: str, json: dict) -> dict:

    headers = {
        "X-Api-Key": key,
        "Accept": "application/json",
    }

    while not __findings_ready(key, json["token"]):
        sleep(0.5)
        print("Not ready...")

    print("Results are in!")

    findings_ep = DTEndpoints.get_findings()
    findings = get(findings_ep, headers=headers)

    return findings.json()


# BEGIN HANDLERS ->


def store_handler(event, context) -> dict:

    """
    This is the Lambda Handler that validates an incoming SBOM
    and if valid, puts the SBOM into the S3 bucket associated
    to the application.
    """

    bom_obj = __get_bom_obj(event)

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
            DT_TOKEN_KEY: __generate_token()
        }

        # Extract the actual SBOM.
        bom_bytes = bytearray(dumps(bom_obj), "utf-8")
        s3.Object(bucket_name, key).put(Body=bom_bytes, Metadata=metadata)

    except ValidationError as validation_error:
        response_obj["statusCode"] = 400
        response_obj["body"] = str(validation_error)

    return response_obj


def enrichment_entry_handler(event=None, context=None):

    """
    Handler that listens for S3 put events and routes the SBOM
    to the enrichment code
    """

    if not event:
        raise ValidationError("event should never be none")

    event_obj: dict = loads(event) if event is str else event
    queue_url = environ[DT_QUEUE_URL_EV]
    for record in event_obj["Records"]:

        s3_obj = record["s3"]
        bucket_obj = s3_obj["bucket"]
        bucket_name = bucket_obj["name"]
        sbom_obj = s3_obj["object"]
        key = sbom_obj["key"]  # TODO The key name needs to be identifiable
        s3_object = s3.Object(bucket_name, key).get()
        dt_project_token = s3_object["Metadata"][DT_TOKEN_KEY]
        bom = s3_object["Body"].read()

        try:
            sqs_client.send_message(
                QueueUrl=queue_url,
                MessageAttributes={
                    DT_TOKEN_KEY: {
                        "DataType": "String",
                        "StringValue": dt_project_token,
                    }
                },
                MessageGroupId="dt_enrichment",
                MessageBody=str(bom),
            )
        except ClientError:
            print(f"Could not send message to the - {queue_url}.")
            raise


def dt_ingress_handler(event=None, context=None):

    """
    Developing Dependency Track Ingress Handler
    """

    if not event:
        raise ValidationError("event should never be none")

    # Make a filehandle out of the JSON String
    event_str: str = dumps(event)
    bom_str_file: StringIO = StringIO(event_str)

    # The API key for the project
    key: str = getenv("DT_API_KEY")

    mpe = MultipartEncoder(
        fields={
            "project": PROJECT_UUID,
            "autoCreate": "false",
            "bom": (
                "filename",
                bom_str_file,
                "multipart/form-data",
            ),
        }
    )

    headers: dict = {
        "X-Api-Key": key,
        "Accept": "application/json",
        "Content-Type": mpe.content_type,
    }

    response: Response = post(
        DTEndpoints.post_sbom(),
        headers=headers,
        data=mpe,
    )

    json_dict: dict = response.json()
    findings: dict = __get_findings(key, json_dict)

    return findings

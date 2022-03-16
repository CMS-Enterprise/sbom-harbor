"""
This module serves as the external API for CycloneDX Python Module
"""
from os import getenv, environ
from io import StringIO
from time import sleep

from uuid import uuid4
from json import loads, dumps
from requests import post, get, Response
from boto3 import resource
from jsonschema.exceptions import ValidationError
from requests_toolbelt.multipart.encoder import MultipartEncoder

from cyclonedx.core import CycloneDxCore
from cyclonedx.endpoints import Endpoints, PROJECT_UUID


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
        "body": dumps({"valid": True, "s3BucketName": bucket_name, "s3ObjectKey": key}),
    }


def __findings_ready(key: str, token: str) -> bool:

    headers = {
        "X-Api-Key": key,
        "Accept": "application/json",
    }

    response = get(
        Endpoints.get_sbom_status(token),
        headers=headers,
    )

    json_dict = response.json()

    return not json_dict["processing"]


def __get_findings(key: str, json: dict) -> dict:

    headers = {"X-Api-Key": key, "Accept": "application/json"}

    while not __findings_ready(key, json["token"]):
        sleep(0.5)
        print("Not ready...")

    print("Results are in!")

    findings_ep = Endpoints.get_findings()
    findings = get(findings_ep, headers=headers)

    return findings.json()


def store_handler(event=None, context=None) -> dict:

    """
    This is the Lambda Handler that validates an incoming SBOM
    and if valid, puts the SBOM into the S3 bucket associated
    to the application.
    """

    bom_obj = __get_bom_obj(event)

    # Get the bucket name from the environment variable
    # This is set during deployment
    bucket_name = environ["SBOM_BUCKET_NAME"]
    print(f"Bucket name from env(SBOM_BUCKET_NAME): {bucket_name}")

    # Generate the name of the object in S3
    key = f"aquia-{uuid4()}"
    print(f"Putting object in S3 with key: {key}")

    # Create an instance of the Python CycloneDX Core
    core = CycloneDxCore()

    # Create a response object to add values to.
    response_obj = __create_response_obj(bucket_name, key)

    try:

        # Validate the BOM here
        core.validate(bom_obj)

        # Get S3 Bucket
        bucket = resource("s3").Bucket(bucket_name)

        # Actually put the object in S3
        bom_bytes = bytearray(dumps(bom_obj), "utf-8")
        bucket.put_object(Key=key, Body=bom_bytes)

    except ValidationError as validation_error:
        response_obj["statusCode"] = 400
        response_obj["body"] = str(validation_error)

    return response_obj


def dt_ingress_handler(event=None, context=None):

    """
    Developing Dependency Track Ingress Handler
    """

    if not event:
        raise ValidationError("event should never be none")

    # Make a filehandle out of the JSON String
    bom_str_file: StringIO = StringIO(dumps(event))

    # The API key for the project
    key: str = getenv("DT_API_KEY")

    mpe = MultipartEncoder(
        fields={
            "project": PROJECT_UUID,
            "autoCreate": "false",
            "bom": ("filename", bom_str_file, "multipart/form-data"),
        }
    )

    headers: dict = {
        "X-Api-Key": key,
        "Accept": "application/json",
        "Content-Type": mpe.content_type,
    }

    response: Response = post(
        Endpoints.post_sbom(),
        headers=headers,
        data=mpe,
    )

    json_dict: dict = response.json()
    findings: dict = __get_findings(key, json_dict)

    return findings

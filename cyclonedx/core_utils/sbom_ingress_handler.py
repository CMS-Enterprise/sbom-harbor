import datetime
from json import dumps
from os import environ
from uuid import uuid4

from boto3 import resource
from jsonschema.exceptions import ValidationError

from cyclonedx.constants import (
    SBOM_BUCKET_NAME_KEY,
    S3_META_TEAM_KEY,
    S3_META_PROJECT_KEY,
    S3_META_CODEBASE_KEY,
    S3_META_TIMESTAMP_KEY
)
from cyclonedx.core_utils.cyclonedx_util import (
    __create_pristine_response_obj,
    __get_body_from_event,
)


def sbom_ingress_handler(event: dict = None, context: dict = None) -> dict:

    """
    This is the Lambda Handler that validates an incoming SBOM
    and if valid, puts the SBOM into the S3 bucket associated
    to the application.
    """

    s3 = resource("s3")

    # Extract the path parameters and get the team
    path_params = event["pathParameters"]
    team = path_params["team"]
    project = path_params["project"]
    codebase = path_params["codebase"]

    bom_obj = __get_body_from_event(event)

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
    response_obj = __create_pristine_response_obj(bucket_name, key)

    try:

        # Validate the BOM here
        core.validate(bom_obj)

        # Extract the actual SBOM.
        bom_bytes = bytearray(dumps(bom_obj), "utf-8")
        timestamp = datetime.datetime.now().timestamp()
        s3.Object(bucket_name, key).put(
            Body=bom_bytes,
            Metadata={
                S3_META_TEAM_KEY: team,
                S3_META_PROJECT_KEY: project,
                S3_META_CODEBASE_KEY: codebase,
                S3_META_TIMESTAMP_KEY: str(timestamp),
            },
        )

    except ValidationError as validation_error:
        response_obj["statusCode"] = 400
        response_obj["body"] = str(validation_error)

    return response_obj

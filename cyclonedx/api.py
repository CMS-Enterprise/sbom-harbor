import boto3
import os

from jsonschema.exceptions import ValidationError
from urllib import response
from uuid import uuid4
from json import loads, dumps

from cyclonedx.core import CycloneDxCore

def lambda_handler(event, context):

    # Extract the body from the event
    bom_obj = event

    # Get the bucket name from the environment variable
    # This is set during deployment
    bucket_name = os.environ["SBOM_BUCKET_NAME"]
    print("Bucket name from env(SBOM_BUCKET_NAME): %s" % bucket_name)

    # Generate the name of the object in S3
    key = "aquia-%s" % uuid4()
    print("Putting object in S3 with key: %s" % key)

    # Create an instance of the Python CycloneDX Core
    core = CycloneDxCore()

    # Create a response object to add values to.
    response_obj = {
        'statusCode': 200,
        'body': {
            'valid': True,
            's3BucketName': bucket_name,
            's3ObjectKey': key
        }
    }

    try:

        # Validate the BOM here
        core.validate(bom_obj)

        # Get S3 Bucket
        s3 = boto3.resource('s3')
        bucket = s3.Bucket(bucket_name)

        # Actually put the object in S3
        bytes = bytearray(dumps(bom_obj), "utf-8")
        bucket.put_object(Key=key, Body=bytes)
        response_obj['body']['s3ObjectKey'] = key

    except ValidationError as e:
        response_obj['statusCode'] = 400
        response_obj['body'] = str(e)


    return response_obj
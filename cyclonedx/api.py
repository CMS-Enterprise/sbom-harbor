import jsonschema
import boto3
import os

from uuid import uuid4
from json import dumps

def lambda_handler(event, context):

    bucket_name = os.environ["SBOM_BUCKET_NAME"]
    print("Bucket name from env(SBOM_BUCKET_NAME): %s" % bucket_name)

    key = "aquia-%s" % uuid4()

    json = dumps(event)
    s3 = boto3.resource('s3')
    bucket = s3.Bucket(bucket_name)
    bytes = bytearray(json, "utf-8")
    bucket.put_object(Key=key, Body=bytes)
    
    return {
        'statusCode': 200,
        'body': json
    }

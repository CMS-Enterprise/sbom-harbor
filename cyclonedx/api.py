import boto3
from uuid import uuid4
from json import dumps

def lambda_handler(event, context):

    bucket_name = "SBOMBucket"
    
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

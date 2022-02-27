import json
import boto3
import uuid

def lambda_handler(event, context):

    bucket_name = "sbomBucket"
    
    key = "aquia-%s" % uuid.uuid4()

    s3 = boto3.resource('s3')
    bucket = s3.Bucket('name')
    bucket.put_object(Key=key, Body=bytearray(event, "utf-8"))
    return {
        'statusCode': 200,
        'body': "Looks like it came from me"
    }

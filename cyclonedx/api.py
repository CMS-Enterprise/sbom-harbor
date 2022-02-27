import json


def lambda_handler(event, context):
    return {
        'statusCode': 200,
        'body': "Looks like it came from me"
    }


def say_hello():
    print("Hello, I'm accessible")

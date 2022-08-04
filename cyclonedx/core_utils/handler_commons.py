import importlib.resources as pr
from json import loads

import boto3
from boto3.dynamodb.types import (
    TypeDeserializer,
    TypeSerializer
)

import cyclonedx.schemas as schemas

cognito_client = boto3.client('cognito-idp')
dynamodb_resource = boto3.resource('dynamodb')
dynamodb_serializer = TypeSerializer()
dynamodb_deserializer = TypeDeserializer()

team_schema = loads(
    pr.read_text(
        schemas, "team.schema.json"
    )
)


def allow_policy(method_arn: str):
    return {
        "principalId": "apigateway.amazonaws.com",
        "policyDocument": {
            "Version": "2012-10-17",
            "Statement": [{
                "Action": "execute-api:Invoke",
                "Effect": "Allow",
                "Resource": method_arn
            },{
                "Action": "cognito-idp:ListUsers",
                "Effect": "Allow",
                "Resource": method_arn
            }]
        }
    }


def deny_policy():
    return {
        "principalId": "*",
        "policyDocument": {
            "Version": "2012-10-17",
            "Statement": [{
                "Action": "*",
                "Effect": "Deny",
                "Resource": "*"
            }]
        }
    }

"""
This module serves as the external API for CycloneDX Python Module
"""

from io import StringIO
from os import environ
from json import loads, dumps
from uuid import uuid4
import importlib.resources as pr
import boto3
from boto3 import client, resource
import datetime
from dateutil.relativedelta import relativedelta
from botocore.exceptions import ClientError
from boto3.dynamodb.types import TypeDeserializer, TypeSerializer
from decimal import Decimal
import cyclonedx.schemas as schemas
from jsonschema import validate
from jsonschema.exceptions import ValidationError

from cyclonedx.constants import (
    DT_QUEUE_URL_EV,
    ENRICHMENT_ID_SQS_KEY,
    ENRICHMENT_ID,
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY,
    TEAM_MEMBER_TABLE_NAME,
    TEAM_TOKEN_TABLE_NAME,
    USER_POOL_CLIENT_ID_KEY,
    USER_POOL_NAME_KEY,
    TEAM_TABLE_NAME,
)

from cyclonedx.core import CycloneDxCore
from cyclonedx.util import (
    __create_project,
    __create_pristine_response_obj,
    __create_team_response,
    __create_user_search_response_obj,
    __delete_project,
    __get_body_from_event,
    __get_body_from_first_record,
    __get_findings,
    __get_login_failed_response,
    __get_login_success_response,
    __get_records_from_event,
    __handle_delete_token_error,
    __token_response_obj,
    __upload_sbom,
    __validate,
)

cognito_client = boto3.client('cognito-idp')
dynamodb_resource = boto3.resource('dynamodb')
dynamodb_serializer = TypeSerializer()
dynamodb_deserializer = TypeDeserializer()

team_schema = loads(
    pr.read_text(
        schemas, "team.schema.json"
    )
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
    response_obj = __create_pristine_response_obj(bucket_name, key)

    try:

        # Validate the BOM here
        core.validate(bom_obj)

        # Extract the actual SBOM.
        bom_bytes = bytearray(dumps(bom_obj), "utf-8")
        s3.Object(bucket_name, key).put(
            Body=bom_bytes,
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
    findings_key: str = f"findings-{s3_info[SBOM_S3_KEY]}"
    s3.Object(bucket_name, findings_key).put(
        Body=findings_bytes,
    )

    print(f"Findings are in the s3 bucket: {bucket_name}/{findings_key}")

    return True


def login_handler(event, context):

    body = __get_body_from_first_record(event)

    username = body["username"]
    password = body["password"]

    try:
        resp = cognito_client.admin_initiate_auth(
            UserPoolId=environ.get(USER_POOL_NAME_KEY),
            ClientId=environ.get(USER_POOL_CLIENT_ID_KEY),
            AuthFlow='ADMIN_NO_SRP_AUTH',
            AuthParameters={
                "USERNAME": username,
                "PASSWORD": password
            }
        )
    except Exception as err:
        return __get_login_failed_response(401, err)

    jwt = resp['AuthenticationResult']['AccessToken']

    print("Log in success")
    print(f"Access token: {jwt}", )
    print(f"ID token: {resp['AuthenticationResult']['IdToken']}")

    return __get_login_success_response(jwt)


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


def verify_token(token: str):
    return True


def jwt_authorizer_handler(event, context):

    print("<EVENT>")
    print(event)
    print("</EVENT>")

    print("<CONTEXT>")
    print(context)
    print("</CONTEXT>")

    method_arn = event["methodArn"]
    token = event["authorizationToken"]

    return allow_policy(method_arn) if verify_token(token) else deny_policy()


def api_key_authorizer_handler(event, context):

    # Extract the Method ARN and the token from the event
    method_arn = event["methodArn"]
    token = event["authorizationToken"]

    # Extract the path parameters and get the team
    path_params = event["pathParameters"]
    team = path_params["team"]

    # Get our Team table from DynamoDB
    table = dynamodb_resource.Table(TEAM_TABLE_NAME)

    # Set the Key for update_item(). The Id is the team name
    key = {"Id": team}

    # Get the team from the table
    get_item_rsp = table.get_item(Key=key)

    # Extract the existing tokens and find the index of the token
    # the user is looking to delete.
    team = get_item_rsp["Item"]
    tokens = team["tokens"]

    # Set the policy to default Deny
    policy = deny_policy()

    # Go through the tokens the team has
    for team_token in tokens:

        # Make sure the token is enabled
        if team_token["token"] == token and team_token["enabled"]:
            now = datetime.datetime.now().timestamp()
            expires = team_token["expires"]

            # Make sure the token is not expired
            if now < float(expires):
                policy = allow_policy(method_arn)

    # If the token exists, is enabled and not expired, then allow
    return policy


def create_token_handler(event=None, context=None):

    """ Handler that creates a token, puts it in
    DynamoDB and returns it to the requester """

    # Get the team from the path parameters
    # and extract the body from the event
    team_id = event["pathParameters"]["team"]
    body = __get_body_from_event(event)

    # Create a new token starting with "sbom-api",
    # create a creation and expiration time
    token = f"sbom-api-{uuid4()}"
    now = datetime.datetime.now()
    later = now + relativedelta(years=1)

    # Get the timestamps to put in the database
    created = now.timestamp()
    expires = later.timestamp()

    # If a token name is given, set that as the name
    # otherwise put in a default
    name = body["name"] if body["name"] else "NoName"

    # Create a data structure representing the token
    # and it's metadata
    token_item = {
        "TeamId": team_id,
        "name": name,
        "created": Decimal(created),
        "expires": Decimal(expires),
        "enabled": True,
        "token": token,
    }

    # Get the dynamodb resource and add the token
    # to the existing team
    dynamodb = boto3.resource('dynamodb')
    table = dynamodb.Table(TEAM_TOKEN_TABLE_NAME)

    try:
        table.put_item(
            Item=token_item
        )
    except Exception as err:

        # If something happened in AWS that made it where the
        # call could not be completed, send an internal service error.
        return __token_response_obj(
            500, token, f"Request Error from boto3: {err}"
        )

    return __token_response_obj(200, token)


def delete_token_handler(event=None, context=None):

    """ Handler for deleting a token belonging to a given team """

    # Grab the team and the token from the path parameters
    team_id = event["pathParameters"]["team"]
    token = event["pathParameters"]["token"]

    # Get our Team table from DynamoDB
    team_token_table = dynamodb_resource.Table(TEAM_TOKEN_TABLE_NAME)

    try:

        # Delete the token
        team_token_table.delete_item(
            Key={
                "TeamId": team_id,
                "token": token
            },
            ConditionExpression="attribute_exists(TeamId)",
        )

    except ClientError as e:
        return __handle_delete_token_error(
            token, team_id, e
        )

    return __token_response_obj(
        status_code=200,
        token=token,
    )


def update_table(
        key: str,
        team_json: dict,
        team_id: str,
        table: dynamodb_resource.Table,
):
    items = team_json[key]
    for item in items:
        item.update({
            "TeamId": team_id,
        })
        table.put_item(
            Item=item
        )

def register_team_handler(event=None, context=None):

    team_json: dict = __get_body_from_event(event)

    try:
        validate(
            instance=team_json,
            schema=team_schema
        )

        team_id = team_json["Id"]

        # Add the tokens to the token table if there are tokens.
        token_table = dynamodb_resource.Table(TEAM_TOKEN_TABLE_NAME)
        update_table("tokens", team_json, team_id, token_table)
        del team_json["tokens"]

        # Update the members
        member_table = dynamodb_resource.Table(TEAM_MEMBER_TABLE_NAME)
        update_table("members", team_json, team_id, member_table)
        del team_json["members"]

        team_table = dynamodb_resource.Table(TEAM_TABLE_NAME)
        team_table.put_item(Item=team_json)

        return __create_team_response(200, "Team Created")

    except ValidationError as err:
        return __create_team_response(
            500, f"Validation Error: {err}")


def replace_members(team_id: str, new_members: list):

    team_table = dynamodb_resource.Table(TEAM_MEMBER_TABLE_NAME)
    team_query_rsp = team_table.query(
        Select="ALL_ATTRIBUTES",
        KeyConditionExpression="TeamId = :TeamId",
        ExpressionAttributeValues={
            ":TeamId": team_id,
        },
    )

    delete_requests = []
    for item in team_query_rsp["Items"]:
        delete_requests.append({
            'DeleteRequest':{
                'Key': {
                    'TeamId': team_id,
                    'email': item["email"]
                }
            }
        })

    put_requests = []
    for member in new_members:
        put_requests.append({
            'PutRequest': {
                'Item': {
                    'TeamId': team_id,
                    'isTeamLead': member["isTeamLead"],
                    'email': member["email"]
                }
            },
        })

    errors = []

    print(f"Delete Requests: {delete_requests}")
    if len(delete_requests) > 0:
        try:
            dynamodb_resource.batch_write_item(
                RequestItems={
                    'SbomTeamMemberTable': delete_requests
                },
            )
        except ClientError as err:
            errors.append(err)

    print(f"Put Requests: {put_requests}")
    if len(put_requests) > 0:
        try:
            dynamodb_resource.batch_write_item(
                RequestItems={
                    'SbomTeamMemberTable': put_requests
                },
            )
        except ClientError as err:
            errors.append(err)

    return errors if len(errors) < 1 else []


def update_team_handler(event=None, context=None):

    team_json: dict = __get_body_from_event(event)

    print(f"Incoming Team JSON: {team_json}")

    try:
        validate(
            instance=team_json,
            schema=team_schema
        )

        team_id = team_json["Id"]

        if "members" in team_json:
            incoming_members = team_json["members"]
            errors = replace_members(team_id, incoming_members)
            del team_json["members"]
            if len(errors) > 0:
                return __create_team_response(
                    status_code=500,
                    err=dumps(errors)
                )

        team_table = dynamodb_resource.Table(TEAM_TABLE_NAME)
        team_table.delete_item(
            Key={
                "Id": team_id
            }
        )

        team_table.put_item(Item=team_json)

        return __create_team_response(
            status_code=200,
            msg="Team Updated"
        )

    except ValidationError as err:
        return __create_team_response(
            status_code=500,
            err=f"Validation Error: {err}",
        )


def get_team_handler(event=None, context=None):

    team_id = event["pathParameters"]["team"]

    team_table = dynamodb_resource.Table(TEAM_TABLE_NAME)
    team_query_rsp = team_table.query(
        Select="ALL_ATTRIBUTES",
        KeyConditionExpression="Id = :Id",
        ExpressionAttributeValues={
            ":Id": team_id,
        },
    )

    # There will be only one team that matches dur to the uniqueness
    # constraint on the partition value.
    team = team_query_rsp["Items"][0]

    team_members_table = dynamodb_resource.Table(TEAM_MEMBER_TABLE_NAME)
    team_members_query_rsp = team_members_table.query(
        Select="SPECIFIC_ATTRIBUTES",
        ProjectionExpression='email,isTeamLead',
        KeyConditionExpression="TeamId = :Id",
        ExpressionAttributeValues={
            ":Id": team_id,
        },
    )

    team_members = team_members_query_rsp["Items"]
    team["members"] = team_members

    return __create_team_response(
        status_code=200,
        msg=team
    )

## </WORKING>

def user_search_handler(event=None, context=None):

    body = __get_body_from_event(event)

    filter_str = body["filter"]
    user_filter = f"email ^= \"{filter_str}\""


    response = cognito_client.list_users(
        UserPoolId=environ.get(USER_POOL_NAME_KEY),
        AttributesToGet=[
            'email',
        ],
        Limit=60, # Max is 60
        Filter=user_filter,
    )

    users = response["Users"]
    emails = []
    for user in users:
        attr = user["Attributes"]
        emails.append(attr[0]["Value"])

    return __create_user_search_response_obj(200, dumps(emails))

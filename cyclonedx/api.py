"""
This module serves as the external API for CycloneDX Python Module
"""
from os import environ
from io import StringIO
from time import sleep

from uuid import uuid4
from json import loads, dumps

from botocore.client import BaseClient
from requests import post, get, Response, put
from boto3 import resource, client
from botocore.exceptions import ClientError
from jsonschema.exceptions import ValidationError
from requests_toolbelt.multipart.encoder import MultipartEncoder
from cyclonedx.core import CycloneDxCore
from cyclonedx.dtendpoints import DTEndpoints, PROJECT_UUID
from cyclonedx.constants import (
    SBOM_BUCKET_NAME_EV,
    DT_TOKEN_KEY,
    DT_QUEUE_URL_EV,
    DT_API_KEY,
)


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


def __validate_event(event):
    if not event:
        raise ValidationError("event should never be none")

    print("<SBOM>")
    print(event)
    print("</SBOM>")


def __create_project():
    create_project_headers: dict = {
        "Accept": "application/json",
    }

    create_proj_body = {
        "author": "EnrichmentLambda",
        "version": "1.0.0",
        "classifier": "APPLICATION",
        "description": "auto generated project",
    }

    create_proj_rsp: Response = put(
        DTEndpoints.create_project(),
        headers=create_project_headers,
        data=create_proj_body,
    )

    print("<CreateDTProjectResp>")
    print(create_proj_rsp)
    print("</CreateDTProjectResp>")

    return create_proj_rsp


def __upload_sbom(key, bom_str_file):
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

    bom_upload_headers: dict = {
        "X-Api-Key": key,
        "Accept": "application/json",
        "Content-Type": mpe.content_type,
    }

    upload_sbom_rsp: Response = post(
        DTEndpoints.post_sbom(),
        headers=bom_upload_headers,
        data=mpe,
    )

    return upload_sbom_rsp.json()


def __get_bom_from_event(event):

    # Make a filehandle out of the JSON String
    event_str: str = dumps(event)
    return StringIO(event_str)


# BEGIN HANDLERS ->


def store_handler(event, context) -> dict:

    """
    This is the Lambda Handler that validates an incoming SBOM
    and if valid, puts the SBOM into the S3 bucket associated
    to the application.
    """

    bom_obj = __get_bom_obj(event)

    s3 = resource("s3")

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

    s3 = resource("s3")
    sqs_client = client("sqs")

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

        try:
            dt_project_token = s3_object["Metadata"][DT_TOKEN_KEY]
        except KeyError as key_err:
            print("<s3Object>")
            print(s3_object)
            print("</s3Object>")
            raise key_err

        sbom = s3_object["Body"].read()

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
                MessageBody=str(sbom),
            )
        except ClientError:
            print(f"Could not send message to the - {queue_url}.")
            raise


def __extract_api_key():
    __change_admin_pwd()


def __change_admin_pwd():

    pwd: str = "admin"
    new_pwd: str = str(uuid4())

    headers: dict = {
        "Content-Type": "application/x-www-form-urlencoded",
        "Accept": "text/plain",
    }

    params: dict = {
        "username": pwd,
        "password": pwd,
        "newPassword": new_pwd,
        "confirmPassword": new_pwd,
    }

    post(
        DTEndpoints.force_chg_pwd(),
        headers=headers,
        data=params,
    )

    return new_pwd


def __log_in_as_admin(new_pwd: str):

    pwd: str = "admin"

    headers: dict = {
        "Content-Type": "application/x-www-form-urlencoded",
        "Accept": "text/plain",
    }

    params: dict = {
        "username": pwd,
        "password": new_pwd,
    }

    response = post(
        DTEndpoints.do_login(),
        headers=headers,
        data=params,
    )

    return response.text


def __get_teams(jwt: str):

    headers = {
        "Authorization": f"Bearer {jwt}",
        "Accept": "application/json",
    }

    response = get(
        DTEndpoints.get_teams_data(),
        headers=headers,
    )

    print("<TeamsResponse>")
    print(response.text)
    print("<TeamsResponse>")

    return response.text


def __get_api_key_from_teams(teams_str: str):
    teams = loads(teams_str)

    for team in teams:
        if team["name"] == "Automation":
            return team["uuid"], team["apiKeys"][0]["key"]


def __set_team_permissions(uuid, jwt):

    permissions: list = [
        "ACCESS_MANAGEMENT",
        "POLICY_MANAGEMENT",
        "POLICY_VIOLATION_ANALYSIS",
        "PORTFOLIO_MANAGEMENT",
        "PROJECT_CREATION_UPLOAD",
        "SYSTEM_CONFIGURATION",
        "VIEW_PORTFOLIO",
        "VIEW_VULNERABILITY",
        "VULNERABILITY_ANALYSIS",
    ]

    headers = {
        "Authorization": f"Bearer {jwt}",
        "Accept": "application/json",
    }

    for perm in permissions:
        post(
            DTEndpoints.add_permission_to_team(perm, uuid),
            headers=headers,
        )


def __set_key_as_ssm_param(ssm: BaseClient):

    """This function only runs when there is no API Key in SSM.
    Its job is to set up DT to be used without a UI."""

    # The Admin Password on DT must be changed so we can use the
    # Admin account to hit REST interfaces without an API Key
    new_pwd = __change_admin_pwd()

    # Once we set the password, we can use it to log into DT
    # This will produce a JWT token we can further use to hit
    # other endpoints
    jwt = __log_in_as_admin(new_pwd)

    # Use the JWT to get the Teams DT starts with in its database
    teams_data = __get_teams(jwt)

    # Dig through the JSON returned by hitting the Teams endpoint
    # to extract the uuid of the 'Automation' Team and the REST API Key
    # that is generated by default.
    (uuid, api_key) = __get_api_key_from_teams(teams_data)

    # Update the permissions of the Automation Team to allow it to process SBOMs
    __set_team_permissions(uuid, jwt)

    # Put the API Key into SSM
    ssm.put_parameter(
        Name=DT_API_KEY,
        Description="Dependency Track API key from internal DT Automation Team",
        Value=api_key,
        Type="String",
        Overwrite=False,
        Tier="Standard",
        DataType="text",
    )

    return api_key


def __get_api_key(ssm: BaseClient):

    try:
        return ssm.get_parameter(
            Name=DT_API_KEY,
            WithDecryption=False,
        )
    except ssm.exceptions.ParameterNotFound:
        return __set_key_as_ssm_param(ssm)


def dt_ingress_handler(event=None, context=None):

    """
    Developing Dependency Track Ingress Handler
    """

    ssm: BaseClient = client("ssm")

    # Currently making sure it isn't empty
    __validate_event(event)

    # Get the SBOM in a contrived file handle
    bom_str_file: StringIO = __get_bom_from_event(event)

    # The API key for the project
    key: str = __get_api_key(ssm)

    print("<ApiKey>")
    print(key)
    print("</ApiKey>")

    # response = __create_project()
    # json_dict = __upload_sbom(key, bom_str_file)
    # findings: dict = __get_findings(key, json_dict)
    #
    # return findings

from json import dumps, loads
from time import sleep
from uuid import uuid4

from botocore.client import BaseClient
from boto3 import client
from jsonschema.exceptions import ValidationError
from requests import Response, get, post, put
from requests_toolbelt.multipart.encoder import MultipartEncoder

from cyclonedx.constants import (
    DT_API_KEY,
    DT_ROOT_PWD,
    DT_DEFAULT_ADMIN_PWD,
    EMPTY_VALUE,
)
from cyclonedx.dtendpoints import DTEndpoints


def __change_dt_root_pwd():

    ssm: BaseClient = client("ssm")

    pwd: str = DT_DEFAULT_ADMIN_PWD
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

    print(f">>>>> POSTING To : {DTEndpoints.force_chg_pwd()} <<<<<")
    rsp = post(
        DTEndpoints.force_chg_pwd(),
        headers=headers,
        data=params,
    )
    print(f">>>>> POSTED To : {DTEndpoints.force_chg_pwd()}, Rsp: {rsp.text} <<<<<")

    # Put the API Key into SSM
    ssm.put_parameter(
        Name=DT_ROOT_PWD,
        Description="Root Password for Dependency Track",
        Value=new_pwd,
        Type="String",
        Overwrite=True,
        Tier="Standard",
        DataType="text",
    )

    return new_pwd


def __get_root_password():

    ssm: BaseClient = client("ssm")

    try:

        ssm_param: dict = ssm.get_parameter(
            Name=DT_ROOT_PWD,
            WithDecryption=False,
        )

        value = ssm_param["Parameter"]["Value"]

        if EMPTY_VALUE == value:
            __change_dt_root_pwd()
            return __get_root_password()

        return value

    except ssm.exceptions.ParameterNotFound:
        __change_dt_root_pwd()
        return __get_root_password()


def __get_jwt(root_pwd=None):

    if root_pwd is None:
        root_pwd = __get_root_password()

    user: str = "admin"

    headers: dict = {
        "Content-Type": "application/x-www-form-urlencoded",
        "Accept": "text/plain",
    }

    params: dict = {
        "username": user,
        "password": root_pwd,
    }

    response = post(
        DTEndpoints.do_login(),
        headers=headers,
        data=params,
    )

    return response.text


def __get_teams():

    jwt = __get_jwt()

    headers = {
        "Authorization": f"Bearer {jwt}",
        "Accept": "application/json",
    }

    response = get(
        DTEndpoints.get_teams_data(),
        headers=headers,
    )

    return response.json()


def __get_automation_team_data_from_dt():

    for team in __get_teams():
        if team["name"] == "Automation":
            return team["uuid"], team["apiKeys"][0]["key"]


def __set_team_permissions(team_uuid):

    jwt = __get_jwt()

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
            DTEndpoints.add_permission_to_team(perm, team_uuid),
            headers=headers,
        )


def __generate_sbom_api_token() -> str:
    return f"sbom-api-token-{uuid4()}"


def __get_bom_from_event(event) -> dict:

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


def __findings_ready(key: str, sbom_token: str) -> bool:

    headers = {
        "X-Api-Key": key,
        "Accept": "application/json",
    }

    response = get(
        DTEndpoints.get_sbom_status(sbom_token),
        headers=headers,
    )

    json_dict = response.json()

    return not json_dict["processing"]


def __get_findings(project_uuid: str, sbom_token: str) -> dict:

    key = __get_api_key()

    headers = {
        "X-Api-Key": key,
        "Accept": "application/json",
    }

    while not __findings_ready(key, sbom_token):
        sleep(0.5)
        print("Not ready...")

    print("Results are in!")

    findings = get(DTEndpoints.get_findings(project_uuid), headers=headers)

    return findings.json()


def __validate(event):
    if not event:
        raise ValidationError("event should never be none")


def __rotate_api_key():

    old_key = __get_api_key()

    ssm: BaseClient = client("ssm")
    jwt = __get_jwt()

    headers = {
        "Authorization": f"Bearer {jwt}",
        "Accept": "application/json",
    }

    resp = post(DTEndpoints.rotate_api_key(old_key), headers=headers)

    json = resp.json()
    new_api_key = json["key"]

    # Put the API Key into SSM
    ssm.put_parameter(
        Name=DT_API_KEY,
        Description="Dependency Track API key from internal DT Automation Team",
        Value=new_api_key,
        Type="String",
        Overwrite=True,
        Tier="Standard",
        DataType="text",
    )


def __create_project():

    key = __get_api_key()

    create_project_headers: dict = {
        "X-Api-Key": key,
        "Content-Type": "application/json",
        "Accept": "application/json",
    }

    create_proj_body: dict = {
        "author": "EnrichmentLambda",
        "version": "1.0.0",
        "classifier": "APPLICATION",
        "name": str(uuid4()),
        "description": "auto generated project",
    }

    create_proj_rsp: Response = put(
        DTEndpoints.create_project(),
        headers=create_project_headers,
        json=create_proj_body,
    )

    if create_proj_rsp.status_code == 401:
        __rotate_api_key()
        return __create_project()

    proj = create_proj_rsp.json()
    project_uuid = proj["uuid"]

    return project_uuid


def __delete_project(project_uuid: str):

    api_key = __get_api_key()

    create_project_headers: dict = {
        "X-Api-Key": api_key,
        "Content-Type": "application/json",
        "Accept": "application/json",
    }

    put(
        DTEndpoints.delete_project(project_uuid),
        headers=create_project_headers,
    )


def __upload_sbom(project_uuid, bom_str_file):

    api_key = __get_api_key()

    mpe = MultipartEncoder(
        fields={
            "project": project_uuid,
            "autoCreate": "false",
            "bom": (
                "filename",
                bom_str_file,
                "multipart/form-data",
            ),
        }
    )

    bom_upload_headers: dict = {
        "X-Api-Key": api_key,
        "Accept": "application/json",
        "Content-Type": mpe.content_type,
    }

    upload_sbom_rsp: Response = post(
        DTEndpoints.post_sbom(),
        headers=bom_upload_headers,
        data=mpe,
    )

    json: dict = upload_sbom_rsp.json()

    return json["token"]


def __get_api_key():

    ssm: BaseClient = client("ssm")

    try:

        # Will attempt to get the Key from SSM params
        ssm_param: dict = ssm.get_parameter(
            Name=DT_API_KEY,
            WithDecryption=False,
        )

        value = ssm_param["Parameter"]["Value"]

        if EMPTY_VALUE == value:
            return __set_initial_api_key_in_ssm()

        return value

    # If the Parameter isn't found, then we
    # need to set the initial api key from DT
    except ssm.exceptions.ParameterNotFound:
        print("<__get_api_key -> Loc 05>")
        return __set_initial_api_key_in_ssm()


def __set_initial_api_key_in_ssm():

    """This function only runs when there is no API Key in SSM.
    Its job is to set up DT to be used without a UI."""

    ssm: BaseClient = client("ssm")

    # Dig through the JSON returned by hitting the Teams endpoint
    # to extract the automation_team_uuid of the 'Automation' Team and the REST API Key
    # that is generated by default.
    (automation_team_uuid, api_key) = __get_automation_team_data_from_dt()

    # Update the permissions of the Automation Team to allow it to process SBOMs
    __set_team_permissions(automation_team_uuid)

    # Put the API Key into SSM
    ssm.put_parameter(
        Name=DT_API_KEY,
        Description="Dependency Track API key from internal DT Automation Team",
        Value=api_key,
        Type="String",
        Overwrite=True,
        Tier="Standard",
        DataType="text",
    )

    return api_key
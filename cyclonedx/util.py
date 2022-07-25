""" This Module has all the utility functions
necessary to interoperate with Dependency Track."""
from io import StringIO
from typing import IO

import boto3

from json import dumps, loads
from time import sleep
from uuid import uuid4

import requests
from botocore.client import BaseClient
from boto3 import client
from jsonschema.exceptions import ValidationError
from requests import Response, get, post, put
from requests_toolbelt.multipart.encoder import MultipartEncoder
from urllib3.exceptions import ResponseError

from cyclonedx.constants import (
    DT_API_KEY,
    DT_ROOT_PWD,
    DT_DEFAULT_ADMIN_PWD,
    EMPTY_VALUE,
    IC_API_BASE,
    IC_API_KEY,
    IC_RULESET_TEAM_ID,
    S3_META_CODEBASE_KEY,
    S3_META_PROJECT_KEY,
    S3_META_TEAM_KEY,
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY,
    TEAM_MEMBER_TABLE_NAME,
    TEAM_TABLE_NAME,
)
from cyclonedx.dtendpoints import DTEndpoints


def __change_dt_root_pwd():

    print("@START __change_dt_root_pwd()")

    ssm: BaseClient = client("ssm")

    pwd: str = DT_DEFAULT_ADMIN_PWD
    new_pwd: str = str(uuid4())

    print(f"@NEW PASSWORD __change_dt_root_pwd({new_pwd})")

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

    print(f"@END __change_dt_root_pwd() -> new Pwd({new_pwd})")

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

    print("@START __get_jwt()")

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

    print(
        f"<GetJwtRequest ep={DTEndpoints.do_login()} headers={headers} params={params} />"
    )

    response = post(
        DTEndpoints.do_login(),
        headers=headers,
        data=params,
    )

    jwt = response.text

    print(f"@END __get_jwt({jwt})")

    return jwt


def __get_teams():

    print("@START __get_teams()")

    jwt = __get_jwt()

    headers = {
        "Authorization": f"Bearer {jwt}",
        "Accept": "application/json",
    }

    response = get(
        DTEndpoints.get_teams_data(),
        headers=headers,
    )

    teams = response.json()

    print(f"@END __get_teams({teams})")

    return teams


def __get_automation_team_data_from_dt():

    print("@START __get_automation_team_data_from_dt()")

    for team in __get_teams():
        if team["name"] == "Automation":
            uuid = team["uuid"]
            api_key = team["apiKeys"][0]["key"]
            print(f"@END __get_automation_team_data_from_dt({uuid}, {api_key})")
            return uuid, api_key

    raise Exception("Unable to find Automation team in DT")


def __set_team_permissions(team_uuid):

    print("@START __set_team_permissions()")

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

    print("@END __set_team_permissions()")


def __get_body_from_event(event) -> dict:

    """
    If the request context exists, then there will
    be a 'body' key, and it will contain the JSON object
    as a **string** that the POST body contained.
    """

    print(f"Incoming Event: {event}")
    print(f"Incoming Event Type: {type(event)}")

    event_dict: dict = {}

    if isinstance(event, dict):
        event_dict = event
    elif isinstance(event, str):
        event_dict = loads(event)

    if "Records" in event_dict:
        event_dict = event_dict["Records"][0]

    body = event_dict["body"]
    body = body.decode("utf-8") if isinstance(body, bytes) else body
    print(f"Extracted Body: {body}")
    print(f"Extracted Body Type: {type(body)}")

    return loads(body)


def __get_body_from_first_record(event) -> dict:

    """
    If the request context exists, then there will
    be a 'body' key, and it will contain the JSON object
    as a **string** that the POST body contained.
    """

    event_dict: dict = event

    if "Records" in event_dict:
        event_dict = event_dict["Records"][0]

    body = event_dict["body"]

    return loads(body)


def __get_query_string_params_from_event(event) -> dict:

    """
    If the request context exists, then there will
    be a 'body' key, and it will contain the JSON object
    as a **string** that the POST body contained.
    """


    print(f"Incoming Event: {event}")
    print(f"Incoming Event Type: {type(event)}")

    event_dict: dict = {}

    if isinstance(event, dict):
        event_dict = event
    elif isinstance(event, str):
        event_dict = loads(event)

    if "Records" in event_dict:
        event_dict = event_dict["Records"][0]

    body = event_dict["queryStringParameters"]
    body = body.decode("utf-8") if isinstance(body, bytes) else body
    print(f"Extracted queryStringParameters: {body}")
    print(f"Extracted queryStringParameters Type: {type(body)}")

    return body


def __get_records_from_event(event) -> list:

    """
    If the request context exists, then there will
    be a 'body' key, and it will contain the JSON object
    as a **string** that the POST body contained.
    """

    print(f"Incoming Event: {event}")
    print(f"Incoming Event Type: {type(event)}")

    event_dict: dict = {}

    if isinstance(event, dict):
        event_dict = event
    elif isinstance(event, str):
        event_dict = loads(event)

    if "Records" in event_dict:
        return event_dict["Records"]

    raise KeyError("No 'Records' Key in event")


def __create_pristine_response_obj(bucket_name: str, key: str) -> dict:

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


def __token_response_obj(
        status_code: int, token: str, msg=None) -> dict:

    """
    Creates a dict that is used as the response from the Lambda
    call.  It has all the necessary elements to satisfy AWS's criteria.
    """

    body = {
        "token": token,
    }

    if msg:
        body["error"] = msg

    return {
        "statusCode": status_code,
        "isBase64Encoded": False,
        "body": dumps(body),
    }


def __handle_delete_token_error(token: str, team_id: str, e):

    ec = e.response['Error']['Code']
    if ec == 'ConditionalCheckFailedException':
        error = f"Specified token({token}) does not belong team({team_id})"
    else:
        error = f"Unknown error deleting: token({token}) from team({team_id})"

    return __token_response_obj(
        status_code=500,
        token=token,
        msg=error,
    )

def __create_team_response(status_code: int, msg: str=None, err: str=None) -> dict:

    """
    Creates a dict that is used as the response from the Lambda
    call.  It has all the necessary elements to satisfy AWS's criteria.
    """

    body = {}

    if msg:
        body["response"] = msg
    elif err:
        body["error"] = err

    return {
        "statusCode": status_code,
        "isBase64Encoded": False,
        "body": dumps(body)
    }


def __create_user_search_response_obj(status_code: int, msg: str) -> dict:

    """
    Creates a dict that is used as the response from the Lambda
    call.  It has all the necessary elements to satisfy AWS's criteria.
    """

    return {
        "statusCode": status_code,
        "isBase64Encoded": False,
        "body": msg
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

    findings = get(DTEndpoints.get_findings(project_uuid), headers=headers)
    json = findings.json()

    print("<Results are in!>")
    print(json)
    print("</Results are in!>")

    return json


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

    print(f"<ProjectCreated uuid={project_uuid}>")

    return project_uuid


def __delete_project(project_uuid: str):

    api_key = __get_api_key()

    create_project_headers: dict = {
        "X-Api-Key": api_key,
        "Content-Type": "application/json",
        "Accept": "application/json",
    }

    print("<DeletingProject>")
    print(project_uuid)
    print("</DeletingProject>")

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

    print("<BomUploadHeaders>")
    print(bom_upload_headers)
    print("</BomUploadHeaders>")

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


def __get_login_failed_response(status_code: int, err: Exception):

    return {
        "statusCode": status_code,
        "isBase64Encoded": False,
        "body": dumps(
            {
                "error": str(err),
            }
        ),
    }


def __get_login_success_response(jwt: str):

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(
            {
                "token": jwt,
            }
        ),
    }

def __get_all_s3_obj_data(event: dict):

    """ Extracts the Data and Metadata from
        the S3 Object containing the SBOM """

    from boto3 import resource

    s3 = resource("s3")

    # Currently making sure it isn't empty
    __validate(event)

    # EventBridge 'detail' key has the data we need.
    bucket_name: str = event[SBOM_BUCKET_NAME_KEY]
    key: str = event[SBOM_S3_KEY]

    # Get the SBOM from the bucket and stick it
    # into a string based file handle.
    s3_obj_wrapper = s3.Object(bucket_name, key)

    # Extract the Metadata from the sbom object in S3.
    metadata: dict = s3_obj_wrapper.metadata

    # Get the SBOM's actual data out of the S3 Object
    # and put it into a file handle
    s3_object: dict = s3_obj_wrapper.get()
    sbom = s3_object["Body"].read()
    d_sbom = sbom.decode("utf-8")
    sbom_str_file: StringIO = StringIO(d_sbom)

    return {
        'data': sbom_str_file,
        's3_obj_name': key,
        'bucket_name': bucket_name,
        'team': metadata[S3_META_TEAM_KEY],
        'project': metadata[S3_META_PROJECT_KEY],
        'codebase': metadata[S3_META_CODEBASE_KEY],
    }

def __get_team_by_team_id(team_id: str):

    print(f"<team function='__get_team_by_team_id(team_id: str)' id={team_id} />")

    dynamodb_resource = boto3.resource('dynamodb')

    team_table = dynamodb_resource.Table(TEAM_TABLE_NAME)
    team_query_rsp = team_table.query(
        Select="ALL_ATTRIBUTES",
        KeyConditionExpression="Id = :Id",
        ExpressionAttributeValues={
            ":Id": team_id,
        },
    )

    # There will be only one team that matches due to the uniqueness
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

    return team

class ICClient(object):

    """ The ICClient is a client for Ion Channel
        Create a client by passing in the name of the SBOM/IC Team in Ion Channel.
    """

    @staticmethod
    def __get_parameter(parameter_name: str):

        """ Extracts a value from AWS Parameter Store """

        ssm: BaseClient = client("ssm")

        return ssm.get_parameter(
            Name=parameter_name,
            WithDecryption=False,
        )['Parameter']['Value']

    def __get_ssm_parameters(self):

        """ Extracts the Ion Channel parameters in the AWS Parameter Store """

        # The URL that we use for all Ion Channel REST calls
        self.api_base: dict = self.__get_parameter(IC_API_BASE)

        # Get the API key we defined inIon Channel so we can
        # make authorized RESTful calls.
        self.api_key: dict = self.__get_parameter(IC_API_KEY)

        # Get the team id we use to get the ruleset
        # we use for requests
        self.ruleset_team_id: dict = self.__get_parameter(IC_RULESET_TEAM_ID)


    def __get_ic_request_headers(self):

        """ Create the headers we will use when making RESTful Calls to Ion Channel
            The headers need the JWT token we created initially.
        """

        self.headers = {
            'Authorization': f"Bearer {self.api_key}"
        }

    def __get_org_id(self):

        """ Get the Organization id from Ion Channel. """

        ic_org_api_path = f"{self.api_base}/v1/organizations"
        goo_path = f"{ic_org_api_path}/getOwnOrganizations"
        goo_url = f"https://{goo_path}"
        self.org_id: str = self.__get_id(
            obj_name="organization",
            url=goo_url,
        )

    def __get_ruleset_id(self):

        """ Gets the id of the ruleset associated to the CMS1 Team
            We may need to look harder at what rule set we use going forward,
            but for now this default ruleset works.
        """

        ic_ruleset_api_path = f"{self.api_base}/v1/ruleset"
        get_rs_url = f"https://{ic_ruleset_api_path}/getRulesets?team_id={self.ruleset_team_id}"
        self.ruleset_id: str = self.__get_id(
            url=get_rs_url,
        )

    def __get_sbom_id(self, team_name: str, create_if_missing: bool):

        project_path = f"{self.api_base}/v1/project"
        get_sboms_url = f"https://{project_path}/getSBOMs?org_id={self.org_id}"

        print(f"Sending To: GET:{get_sboms_url}")
        get_sboms_rsp = requests.get(
            get_sboms_url,
            headers=self.headers,
        )

        response: dict = get_sboms_rsp.json()

        print(f"<__get_sbom_id response={response} />")

        response_data = response['data']
        software_lists = response_data['softwareLists']

        def filter_func(item): # TODO Change it back to the Lambda
            print(f"<(SBOM|TEAM) item={item} looking for: name={team_name} />")
            return item['name'] == team_name

        ic_sbom_list = list(filter(
            # TODO Change it back to the Lambda
            #lambda item: item['name'] == team_name,
            filter_func,
            software_lists
        ))

        if len(ic_sbom_list) == 1:
            ic_sbom = ic_sbom_list.pop()
            ( self.sbom_id, self.sbom_team_id ) = ( ic_sbom['id'], ic_sbom["team_id"] )
        elif create_if_missing:
            ( self.sbom_id, self.sbom_team_id ) = self.create_sbom(team_name)
        else:
            self.sbom_id = "ERROR"
            raise ValueError(f"Team: {team_name} does not exist in Ion Channel")

        # print(dumps(ic_sbom_list, indent=2, sort_keys=True))

    def __get_id(self, url: str, obj_name: str = None):

        rsp = requests.get(
            url,
            headers=self.headers,
        )

        rsp_json = rsp.json()
        data_arr = rsp_json["data"]

        if len(data_arr) > 1:
            raise ResponseError(f"Too many data(s), should only be one: {data_arr}")

        obj = data_arr[0][obj_name] if obj_name else data_arr[0]

        return obj["id"]


    def __get_all_active_project_ids(self):

        # Get Projects
        ic_project_api_path = f"{self.api_base}/v1/report"
        get_projects_url = f"https://{ic_project_api_path}/getProjects?team_id={self.sbom_team_id}"

        print(f"Sending To: GET:{get_projects_url}")
        get_projects_rsp: Response = requests.get(
            get_projects_url,
            headers=self.headers,
        )

        response = get_projects_rsp.json()
        response_data = response["data"]

        return [
            p["id"] for p in filter(
                lambda p: p['active'],
                response_data
            )
        ]


    def  __init__(self, team_name: str, create_if_missing: bool=False):

        self.__get_ssm_parameters()
        self.__get_ic_request_headers()
        self.__get_org_id()
        self.__get_ruleset_id()
        self.__get_sbom_id(team_name, create_if_missing)


    def archive_existing_projects(self):

        project_ids = self.__get_all_active_project_ids()
        print(f"<NumIds order='1' value='{len(project_ids)}' />")

        # Report API
        update_projects_path = f"{self.api_base}/v1/project"
        update_projects_url = f"https://{update_projects_path}/updateProjects?archive=true"

        print(f"Sending To: PUT:{update_projects_url}")
        update_projects_rsp = requests.put(
            update_projects_url,
            headers=self.headers,
            json={
                'project_ids': project_ids
            }
        )

        response = update_projects_rsp.json()
        response_data = response["data"]
        failures = response_data["failed"]

        print(f"<UpdateProjectsResponse #succeeded='{len(response_data['succeeded'])}' />")
        print(f"<UpdateProjectsResponse #failed='{len(response_data['failed'])}' />")

        return len(failures) == 0


    def create_sbom(self, name: str):

        """ Creates an SBOM in Ion Channel
            This should only be done when registering a Team in the system.
            After an SBOM is created, it will render as a "Team" in Ion Channel.
            After a team is created by creating an SBOM, one must import subsequent
            SBOMs into that team using the import endpoint.
        """

        ic_proj_api_path = f"{self.api_base}/v1/project"
        create_sbom_url = f"https://{ic_proj_api_path}/createSBOM"

        print(f"Sending To: POST:{create_sbom_url}")
        create_rsp = requests.post(
            create_sbom_url,
            headers=self.headers,
            json={
                'name': name,
                'version': '0.0.1', # TODO
                'supplier_name': 'aquia', # TODO
                'contact_name': 'qtpeters', # TODO
                'contact_email': 'quinn.peters@aquia.io', # TODO
                'monitor_frequency': 'daily',
                'org_id': self.org_id,
                'ruleset_id': self.ruleset_id,
            },
        )

        create_rsp_json = create_rsp.json()
        print(f"Create Response: {create_rsp_json}")

        sbom_id = create_rsp_json["data"]["id"]
        sbom_team_id = create_rsp_json["data"]["team_id"]

        return sbom_id, sbom_team_id


    def get_analysis_id(self, project_id: str):


        # /?team_id=[TEAM_ID]&project_id=[PROJECT_ID]
        ic_ruleset_api_path = f"{self.api_base}/v1/animal"
        latest_analysis_url = f"https://{ic_ruleset_api_path}/getLatestAnalysis?team_id={self.sbom_team_id}&project_id={project_id}"

        print(f"Sending To: GET:{latest_analysis_url}")
        project_history_rsp = requests.get(
            latest_analysis_url,
            headers=self.headers,
        )

        return project_history_rsp.json()


    def import_sbom(self, sbom_fh: IO):

        ic_proj_api_path = f"{self.api_base}/v1/project"
        import_sbom_url = f"https://{ic_proj_api_path}/importSBOM?sbom_id={self.sbom_id}"

        print(f"Sending To: POST:{import_sbom_url}")
        import_rsp = requests.post(
            import_sbom_url,
            headers=self.headers,
            files={
                'file': sbom_fh
            },
        )

        irj = import_rsp.json()
        irj_data = irj['data']


        print(f"<SBOMImportResponse sbom-id='{irj_data['id']}' />")
        print(f"<SBOMImportResponse sbom-name='{irj_data['name']}' />")

        save_sbom_url = f"https://{ic_proj_api_path}/saveConfirmSBOM?id={self.sbom_id}"

        print(f"Sending To: POST:{save_sbom_url}")
        save_rsp = requests.post(
            save_sbom_url,
            headers=self.headers,
        )

        save_rsp_json = save_rsp.json()
        print(f"Save Response: {save_rsp_json}")


    def analyze_sbom(self):

        project_ids = self.__get_all_active_project_ids()
        individually_wrapped_project_ids = list(
            map(
                lambda p_id: { 'project_id': p_id },
                project_ids
            )
        )

        # # Scanner API
        ic_scanner_api_path = f"{self.api_base}/v1/scanner"

        print(f"<NumIds order='2' value='{len(project_ids)}' />")

        analyze_proj_url = f"https://{ic_scanner_api_path}/analyzeProjects"
        print(f"Sending To: POST:{analyze_proj_url}")
        analyze_projects_rsp: Response = requests.post(
            analyze_proj_url,
            headers=self.headers,
            json=individually_wrapped_project_ids,
        )

        response_dict: dict = analyze_projects_rsp.json()
        print(f"<AnalysisRequestStatusCode response={analyze_projects_rsp.status_code} />")
        print(f"<AnalysisRequestResponse response={response_dict} />")

        analysis_successes: list = response_dict["data"]["succeeded"]
        analysis_status_url = f"https://{ic_scanner_api_path}/getAnalysisStatus"

        complete = False

        while not complete:

            # We hope it's done.
            complete = True

            for analysis in analysis_successes:

                analysis_id: str = analysis["id"]
                team_id: str = analysis["team_id"]
                project_id: str = analysis["project_id"]

                url_with_static_qp = f"{analysis_status_url}?id={analysis_id}&team_id={team_id}"
                url = f"{url_with_static_qp}&project_id={project_id}"
                print(f"Sending To: POST:{url}")
                status_rsp: Response = requests.get(
                    url, headers=self.headers,
                )

                status_rsp_dict: dict = status_rsp.json()

                print(f"<GetAnalysisStatus value='{status_rsp_dict}' />")

                # Can at least be 'finished' or 'queued'
                analysis: str = status_rsp_dict["data"]["status"]

                if analysis == "queued":

                    # We find out that it isn't done
                    complete = False

            if not complete:
                print("Not complete.  Queued projects exist, sleeping for 3s before checking again")
                sleep(3)


        print(f"Analysis Response: {response_dict}")


    def report_ready(self):

        # Report API
        ic_report_api_path = f"{self.api_base}/v1/report"
        analysis_status_url = f"https://{ic_report_api_path}/getAnalysisStatus?id="
        analyze_proj_url = f"https://{ic_report_api_path}/getAnalysis"

        print(f"Sending To: GET:{analyze_proj_url}")
        get_projects_rsp = requests.get(
            analyze_proj_url,
            headers=self.headers,
        )

        print(get_projects_rsp.json())

    def get_report(self):

        # Report API
        ic_report_api_path = f"{self.api_base}/v1/report"
        analyze_proj_url = f"https://{ic_report_api_path}/getAnalysis"

        print(f"Sending To: GET:{analyze_proj_url}")
        get_projects_rsp = requests.get(
            analyze_proj_url,
            headers=self.headers,
        )

        print(get_projects_rsp.json())
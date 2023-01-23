from json import loads
from os import listdir
from subprocess import Popen, PIPE

import boto3

from cyclonedx.clients import HarborDBClient


def get_boto_session():

    """
    -> Get a Boto Session to the CMS AWS Dev Account
    """

    process: Popen = Popen(["/usr/local/bin/get-cms-creds.sh"], stdout=PIPE)
    creds = process.stdout
    cred_json = loads(creds.read())

    return boto3.Session(
        aws_access_key_id=cred_json["AccessKeyId"],
        aws_session_token=cred_json["SessionToken"],
        aws_secret_access_key=cred_json["SecretAccessKey"]
    )


def get_current_environment():

    """
    -> Get Environment we last deployed
    """

    cdk_out_dir: str = f"../../cdk.out"

    files: list[str] = listdir(cdk_out_dir)
    for file in files:
        if "harbor" in file and ".json" in file:
            return file.split("-")[0]


def get_harbor_table_name():
    environment: str = get_current_environment()
    return f"{environment}-HarborTeams-use1"

def get_harbor_client():
    session = get_boto_session()
    resource = session.resource("dynamodb")
    htn: str = get_harbor_table_name()

    print(f"Working with Harbor Table: (> {htn} <)")

    return HarborDBClient(resource, htn)

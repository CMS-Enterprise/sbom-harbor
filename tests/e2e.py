""" End-to-End Test for the system """
import importlib.resources as pr
import tests.sboms as sboms
from json import loads
from uuid import uuid4

import tests.data as data
import requests
from optparse import OptionParser

API_ID = "b8vvubhi9e"
REGION = "us-east-1"
STAGE = "prod"

USER = "sbomadmin@aquia.us"
PASS = "L0g1nTe5tP@55!"

CF_HOST = "dpr2csfynca34"
CF_URL = f"https://{CF_HOST}.cloudfront.net"

APIGW_HOST = "v085bnkx4m"
APIGW_URL = f"https://{APIGW_HOST}.execute-api.us-east-1.amazonaws.com"

LOGIN_URL = f"{CF_URL}/api/login"
TEAM_URL = f"{CF_URL}/api/team"

team = "abc123"
project = "AwesomeProj"
codebase = "Website"

SBOM_UPLOAD_URL = f"{CF_URL}/api/{team}/{project}/{codebase}/sbom"

SBOM = loads(pr.read_text(sboms, "cms_npm_package.json"))
parser = OptionParser("usage: %prog [options]")
parser.add_option('--fail', dest="fail", help='fail flag', action="store")


def __get_token_url(team: str, token=None):
    url = f"{CF_URL}/api/{team}/token"

    if token:
        url = f"{url}/{token}"

    return url


def team_test():

    team_json = loads(
        pr.read_text(
            data, "team.correct.json"
        )
    )

    team_json["Id"] = str(uuid4())

    print(f"Sending To: POST:{LOGIN_URL}")
    login_rsp = requests.post(LOGIN_URL, json={
        "username": USER,
        "password": PASS
    })

    login_rsp_json = login_rsp.json()
    print(f"Response: {login_rsp_json}")
    jwt = login_rsp_json["token"]

    print(f"Sending To: POST:{TEAM_URL}")
    team_rsp = requests.post(
        TEAM_URL,
        json=team_json,
        headers={
            'Authorization': jwt
        }
    )

    team_rsp = team_rsp.text
    print(f"Response: {team_rsp}")


def token_test():

    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    (options, args) = parser.parse_args()

    login_fail = False
    create_fail = False
    delete_fail = False
    if options.fail:
        if options.fail == "login":
            login_fail = True
        elif options.fail == "create":
            create_fail = True
        elif options.fail == "delete":
            delete_fail = True
        else:
            print(f"{options.fail} is not a failure option")

    print(f"Sending To: POST:{LOGIN_URL}")
    login_rsp = requests.post(LOGIN_URL, json={
        "username": USER,
        "password": "wrong_password" if login_fail else PASS
    })

    login_rsp_json = login_rsp.json()
    print(f"Response: {login_rsp_json}")

    if not login_fail:

        jwt = login_rsp_json["token"]

        create_token_url = __get_token_url("Team_DNE" if create_fail else team)
        print(f"Sending To: POST:{create_token_url}")
        create_token_rsp = requests.post(
            create_token_url,
            headers={
                'Authorization': jwt
            },
            json={
                "name": "Test Token from e2e"
            }
        )

        token_json = create_token_rsp.json()
        print(token_json)

        if not create_fail:

            token = token_json['token']
            delete_url = __get_token_url(team, "not_real_token" if delete_fail else token)
            print(f"Sending To: DELETE:{delete_url}")
            delete_token_rsp = requests.delete(
                delete_url,
                headers={
                    'Authorization': jwt
                },
            )

            print(delete_token_rsp.text)


def sbom_upload_test():
    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    working_token = "8d191d16-467e-4150-8416-f51fc7ca1b93"
    made_up_token = "8d191d16-467e-4150-8416-f51fc7ca1b69"
    disabled_token = "8d191d16-467e-4150-8416-f51fc7ca1b94"
    expired_token = "8d191d16-467e-4150-8416-f51fc7ca1b95"

    print("Sending To: %s" % SBOM_UPLOAD_URL)
    print("<SBOM>")
    print(SBOM)
    print("</SBOM>")

    rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={
            'Authorization': working_token
        },
    )

    if rsp.status_code == 200:
        print("Correct token test passed")
    else:
        print(f"Correct token test failed, received: {rsp.status_code}")

    made_up_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={
            'Authorization': made_up_token
        },
    )

    if made_up_token_rsp.status_code == 403:
        print("Bad Token test passed")
    else:
        print(f"Bad Token test failed, received: {made_up_token_rsp.status_code}")

    disabled_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={
            'Authorization': disabled_token
        },
    )

    if disabled_token_rsp.status_code == 403:
        print("Disabled Token test passed")
    else:
        print(f"Disabled Token test failed, received: {disabled_token_rsp.status_code}")

    expired_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={
            'Authorization': expired_token
        },
    )

    if expired_token_rsp.status_code == 403:
        print("Expired token test passed")
    else:
        print(f"Expired Token test failed, received: {expired_token_rsp.status_code}")
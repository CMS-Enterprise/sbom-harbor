""" End-to-End Test for the system """
import importlib.resources as pr
from json import loads

import requests
from optparse import OptionParser

API_ID = "b8vvubhi9e"
REGION = "us-east-1"
STAGE = "prod"

USER = "sbomadmin@aquia.us"
PASS = "L0g1nTe5tP@55!"

CF_HOST = "d1cql9t0re94ga"
CF_URL = f"https://{CF_HOST}.cloudfront.net"

APIGW_HOST = "9wknt2khb9"
APIGW_URL = f"https://{APIGW_HOST}.execute-api.us-east-1.amazonaws.com"

LOGIN_URL = f"{CF_URL}/api/login"

# SBOM = loads(pr.read_text(sboms, "cms_npm_package.json"))
parser = OptionParser("usage: %prog [options]")
parser.add_option('--fail', dest="fail", help='fail flag', action="store")


def __get_token_url(team: str, token=None):
    url = f"{CF_URL}/api/{team}/token"

    if token:
        url = f"{url}/{token}"

    return url


def token_test():

    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    (options, args) = parser.parse_args()

    team = "abc123"
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

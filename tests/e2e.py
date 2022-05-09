""" End-to-End Test for the system """
import importlib.resources as pr
from json import loads

import requests

import tests.sboms as sboms

API_ID = "b8vvubhi9e"
REGION = "us-east-1"
STAGE = "prod"

CF_BASE_URL = "https://d10fb2gc5r4vcd.cloudfront.net"
PATH = "api/login"
USER = "sbomadmin@aquia.us"
PASS = "L0g1nTe5tP@55!"

INVOKE_URL = f"{CF_BASE_URL}/{PATH}"

# SBOM = loads(pr.read_text(sboms, "cms_npm_package.json"))


def post_test():

    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    print("Sending To: %s" % INVOKE_URL)

    rsp = requests.post(INVOKE_URL, json={
        "username": USER,
        "password": PASS
    })

    print(rsp.text)

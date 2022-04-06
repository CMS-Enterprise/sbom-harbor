""" End-to-End Test for the system """
import importlib.resources as pr
from json import loads

import requests

import tests.sboms as sboms

BASE_URL = "https://ajhwhoyzv8.execute-api.us-east-1.amazonaws.com/prod/"
INVOKE_URL = f"{BASE_URL}/store"
SBOM = loads(pr.read_text(sboms, "cms_npm_package.json"))


def post_test():

    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    print("Sending To: %s" % INVOKE_URL)
    print("<SBOM>")
    print(SBOM)
    print("</SBOM>")

    rsp = requests.post(INVOKE_URL, json=SBOM)
    print(rsp.text)

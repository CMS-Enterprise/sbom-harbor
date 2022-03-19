""" End-to-End Test for the system """
import importlib.resources as pr
from json import loads

import requests

import tests.sboms as sboms

INVOKE_URL = "https://cho73qixjc.execute-api.us-east-1.amazonaws.com/prod/store"
SAAS_BOM = loads(pr.read_text(sboms, "SaasBOM.json"))
BIG_BOM = loads(pr.read_text(sboms, "cern.json"))


def post_test():

    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    print("Sending To: %s" % INVOKE_URL)
    print("<SaasBOM>")
    print(SAAS_BOM)
    print("</SaasBOM>")

    rsp = requests.post(INVOKE_URL, json=SAAS_BOM)
    print(rsp.text)

    print("Sending To: %s" % INVOKE_URL)
    print("<BigBOM>")
    print(BIG_BOM)
    print("</BigBOM>")

    rsp = requests.post(INVOKE_URL, json=BIG_BOM)
    print(rsp.text)

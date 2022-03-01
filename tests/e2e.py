import requests
import importlib.resources as pr
import tests.sboms as sboms
from json import loads

INVOKE_URL = 'https://bqetkv0bbh.execute-api.us-east-1.amazonaws.com/prod/store'
SASS_BOM = loads(pr.read_text(sboms, "SaasBOM.json"))

def post_test():

    print('Sending...')
    print("<SaasBOM>")
    print(SASS_BOM)
    print("</SaasBOM>")
    print("To: %s" % INVOKE_URL)

    rsp = requests.post(INVOKE_URL, json=SASS_BOM)
    print(rsp.text)

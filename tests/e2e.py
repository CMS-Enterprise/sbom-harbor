""" End-to-End Test for the system """
import boto3
import os
import importlib.resources as pr
import tests.sboms as sboms
import tests.data as data
import requests

from json import loads
from uuid import uuid4
from optparse import OptionParser

from cyclonedx.enrichment import des_interface_handler
from cyclonedx.core_utils import ICClient
from cyclonedx.constants import (
    S3_META_CODEBASE_KEY,
    S3_META_PROJECT_KEY,
    S3_META_TEAM_KEY,
)

client = boto3.client('cloudfront')
distributions = client.list_distributions()
distribution_list = distributions["DistributionList"]

# only one right now
sbom_api_distribution = distribution_list["Items"][0]
cf_domain_name = sbom_api_distribution["DomainName"]

origins = sbom_api_distribution["Origins"]["Items"]

apigw_domain_name = ""
for origin in origins:
    domain_name: str = origin["DomainName"]
    if "execute-api" in domain_name:
        apigw_domain_name = domain_name

CF_URL = f"https://{cf_domain_name}"
APIGW_URL = f"https://{apigw_domain_name}"

URL = CF_URL

REGION = "us-east-1"
STAGE = "prod"

USER = "sbomadmin@aquia.io"
PASS = "L0g1nTe5tP@55!"

LOGIN_URL = f"{URL}/api/login"
TEAM_URL = f"{URL}/api/team"

team = "abc123"
project = "AwesomeProj"
codebase = "Website"

SBOM_UPLOAD_URL = f"{URL}/api/{team}/{project}/{codebase}/sbom"
USER_SEARCH_URL = f"{URL}/api/user/search"

SBOM = loads(pr.read_text(sboms, "keycloak.json"))
parser = OptionParser("usage: %prog [options]")
parser.add_option('--fail', dest="fail", help='fail flag', action="store")


def __get_token_url(team_name: str, token=None):
    url = f"{URL}/api/{team_name}/token"

    if token:
        url = f"{url}/{token}"

    return url


def __login():

    print(f"Sending To: POST:{LOGIN_URL}, With: {USER}, {PASS}")
    login_rsp = requests.post(
        LOGIN_URL,
        json={
            "username": USER,
            "password": PASS
        }
    )

    print(f"Login Rsp: {login_rsp.text}")

    login_rsp_json = login_rsp.json()
    return login_rsp_json["token"]


def test_team():

    team_json = loads(
        pr.read_text(
            data, "team.correct.json"
        )
    )

    team_id = str(uuid4())
    team_json["Id"] = team_id

    jwt = __login()

    print(f"Sending To: POST:{TEAM_URL}")
    team_rsp = requests.post(
        TEAM_URL,
        json=team_json,
        headers={
            'Authorization': jwt
        }
    )

    team_rsp_txt = team_rsp.text
    print(f"Response: {team_rsp_txt}")

    print(f"Sending To: GET:{TEAM_URL}")
    get_team_rsp = requests.get(
        f"{TEAM_URL}/{team_id}",
        headers={
            'Authorization': jwt
        }
    )

    team_rsp = get_team_rsp.json()
    print(f"Response: {team_rsp}")

    if "response" in team_rsp:
        new_team = team_rsp["response"]
        new_team["projects"] = []
    else:
        print(f"Failure: {team_rsp}")
        new_team = { "projects": [] }
        exit()

    new_proj_name = str(uuid4())
    new_project = {
        "projectName": new_proj_name,
        "codebases": []
    }

    new_team["projects"].append(new_project)

    print(f"New Team: {new_team}")

    print(f"Sending To: PUT:{TEAM_URL}")
    get_team_update_rsp = requests.put(
        TEAM_URL,
        headers={
            'Authorization': jwt
        },
        json=new_team
    )

    team_rsp = get_team_update_rsp.json()
    print(f"Response: {team_rsp}")

def test_token():

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


def test_sbom_upload():
    """
    Posts some SBOMS to the Endpoint currently running in AWS
    """

    working_token = "8d191d16-467e-4150-8416-f51fc7ca1b93"
    made_up_token = "8d191d16-467e-4150-8416-f51fc7ca1b69"
    disabled_token = "8d191d16-467e-4150-8416-f51fc7ca1b94"
    expired_token = "8d191d16-467e-4150-8416-f51fc7ca1b95"

    print("Sending To: %s" % SBOM_UPLOAD_URL)

    good_token_rsp = requests.post(
        SBOM_UPLOAD_URL,
        json=SBOM,
        headers={
            'Authorization': working_token
        },
    )

    if good_token_rsp.status_code == 200:
        print("Correct token test passed")
    else:
        print(f"Correct token test failed, received: {good_token_rsp.status_code}")
        print(good_token_rsp.text)

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
        print(made_up_token_rsp.text)

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
        print(disabled_token_rsp.text)

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
        print(expired_token_rsp.text)

def test_user_search():

    jwt = __login()

    user_mar = "mar"
    url = f"{USER_SEARCH_URL}?filter={user_mar}"
    print(f"Sending To: GET:{url}")
    user_search_rsp = requests.get(
        url,
        headers={
            'Authorization': jwt
        },
    )

    mar_result = user_search_rsp.json()
    if 'maria@aquia.io' in mar_result and 'martha@aquia.io'in mar_result:
        print("Passed using 'mar' filter")
    else:
        print("Failed using 'mar' filter")

    user_qui = "qui"
    url = f"{USER_SEARCH_URL}?filter={user_qui}"
    print(f"Sending To: GET:{url}")
    user_search_rsp = requests.get(
        url, headers={
            'Authorization': jwt
        },
    )

    qui_result = user_search_rsp.json()
    if 'quinn@aquia.io' in qui_result \
        and 'quinton@aquia.io'in qui_result \
            and'quison@aquia.io'in qui_result:
        print("Passed using 'qui' filter")
    else:
        print("Failed using 'qui' filter")


def test_get_teams_for_id():

    jwt = __login()

    url = f"{URL}/api/user/teams?user_id=bill@aquia.io"
    print(f"Sending To: GET:{url}")
    user_search_rsp = requests.get(
        url,
        headers={
            'Authorization': jwt
        },
    )

    rsp = [ t["Id"] for t in user_search_rsp.json() ]

    print(rsp)

    assert 'abc123' in rsp
    assert 'def456' in rsp


def test_get_teams_for_id_no_user():

    jwt = __login()

    url = f"{CF_URL}/api/user/teams?user_id=willy.wonka@aquia.io"
    print(f"Sending To: GET:{url}")
    user_search_rsp = requests.get(
        url,
        headers={
            'Authorization': jwt
        },
    )

    rsp = user_search_rsp.json()

    assert len(rsp) == 0

def test_put_sbom_in_s3():

    dirname = os.path.dirname(__file__)
    filename = os.path.join(dirname, 'sboms/keycloak.json')

    s3 = boto3.resource('s3')
    s3_obj = s3.Object(
        'sbom.bucket.531175407938',
        'sbom-keycloak.json',
    )
    s3_obj.put(
        Body=open(filename, 'rb'),
        Metadata={
            S3_META_TEAM_KEY: 'EVAL TestTeam',
            S3_META_PROJECT_KEY: 'TestProject',
            S3_META_CODEBASE_KEY: "TestCodebase",
        }
    )

def test_get_analysis_id():

    from json import dumps

    try:
        name: str = "TEST STeam-SProject-SCodebase"
        ic_client = ICClient(name, True)
        response = ic_client.get_analysis_id('6c4ce5c9-fcf4-43af-9c8e-f42f06267cd4')

        print(dumps(response, indent=2))

    except ValueError as ex:
        print(f"Problem starting ICClient: {ex}")

def test_nvd_lookup():

    # cpe:2.3: part : vendor : product : version : update : edition :
    #    language : sw_edition : target_sw : target_hw : other
    # https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir7695.pdf

    des_interface_handler({
      "version": "0",
      "id": "53fa0c0f-dbc7-e64b-a003-a5b7e8cdb600",
      "detail-type": "test_detail_type_string",
      "source": "enrichment.lambda_objects",
      "account": "531175407938",
      "time": "2022-06-30T07:36:07Z",
      "region": "us-east-1",
      "resources": [],
      "detail": {
        "sbom_bucket": "sbom.bucket.531175407938",
        "sbom_s3_key": "keycloak.json"
      }
    })

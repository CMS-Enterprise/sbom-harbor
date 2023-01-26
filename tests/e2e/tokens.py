"""
-> End-to-End Test for the tokens
"""
import datetime
from datetime import datetime
from time import sleep
from uuid import uuid4

import pytest
import pytz
import requests
from dateutil.relativedelta import relativedelta
from requests import Response, get, post, put

from cyclonedx.model.token import Token
from tests.e2e import (
    create_team_with_projects,
    get_cloudfront_url,
    get_team_url,
    login,
    print_response,
)


def test_update_token():

    """
    -> Update Token
    """

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    # Create a team with 2 projects
    team_name: str = "1Team1"
    proj1_name: str = "1Project1"
    proj2_name: str = "2Project2"

    team_url: str = get_team_url(cf_url)
    create_json: dict = create_team_with_projects(
        team_name=team_name,
        project_names=[
            proj1_name,
            proj2_name,
        ],
        team_url=team_url,
        jwt=jwt,
    )

    team_id: str = create_json.get("id")

    token_name: str = "INIT TOKEN NAME"

    token_post_url: str = f"{cf_url}/api/v1/token?teamId={team_id}"
    print(f"Sending To: POST:{token_post_url}")
    post_rsp: Response = post(
        token_post_url,
        headers={
            "Authorization": jwt,
        },
        json={
            Token.Fields.NAME: token_name,
        },
    )
    print_response(post_rsp)

    post_json: dict = post_rsp.json()
    token_id: str = post_json.get("id")

    new_token_name: str = "FINAL TOKEN NAME"

    token_put_url: str = f"{cf_url}/api/v1/token/{token_id}?teamId={team_id}"
    print(f"Sending To: PUT:{token_put_url}")
    put_rsp: Response = put(
        token_put_url,
        headers={
            "Authorization": jwt,
        },
        json={
            Token.Fields.NAME: new_token_name,
            Token.Fields.ENABLED: False,
        },
    )

    # There needs to be a sleep here because DynamoDB does not
    # update fast enough to get the new data if it's not.
    sleep(10)

    print_response(put_rsp)

    token_get_url: str = f"{cf_url}/api/v1/token/{token_id}?teamId={team_id}"
    print(f"Sending To: GET:{token_get_url}")
    get_rsp: Response = get(
        token_get_url,
        headers={
            "Authorization": jwt,
        },
    )
    print_response(get_rsp)

    get_rsp_json: dict = get_rsp.json()
    assert get_rsp_json.get(Token.Fields.NAME) == new_token_name
    assert not get_rsp_json.get(Token.Fields.ENABLED)


def test_tokens_use_iso_date_string():

    """
    -> Test using the ISO Date String
    """
    cf_url: str = get_cloudfront_url()
    team_id: str = str(uuid4())

    tomorrow = datetime.now(pytz.utc) + relativedelta(days=1)
    expires: str = tomorrow.isoformat()

    url: str = f"{cf_url}/api/v1/token?teamId={team_id}"
    rsp: Response = requests.post(
        url=url,
        headers={
            "Authorization": login(cf_url),
        },
        json={
            "name": "Test Token",
            "expires": expires,
        },
    )

    print(f"Expires: {expires}")
    print_response(rsp)
    json_dict: dict = rsp.json()

    assert expires == json_dict["expires"]

    try:
        datetime.fromisoformat(json_dict["created"])
    except ValueError:
        pytest.fail()


def test_token_creation_fail_if_expiration_date_is_before_creation():

    """
    -> Test that the expiration date comes after the creations date
    """
    cf_url: str = get_cloudfront_url()
    team_id: str = str(uuid4())

    yesterday = datetime.now() - relativedelta(days=1)
    expires: str = yesterday.isoformat()

    url: str = f"{cf_url}/api/v1/token?teamId={team_id}"
    rsp: Response = requests.post(
        url=url,
        headers={
            "Authorization": login(cf_url),
        },
        json={
            "name": "Test Token",
            "expires": expires,
        },
    )

    print(f"Expires: {expires}")
    print_response(rsp)

    assert rsp.status_code == 400

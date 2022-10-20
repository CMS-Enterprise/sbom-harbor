"""
-> End-to-End Test for the tokens
"""
import datetime
from uuid import uuid4
from datetime import datetime

import pytest

import requests
from dateutil.relativedelta import relativedelta
from requests import Response

from tests.e2e import get_cloudfront_url, login, print_response

cf_url: str = get_cloudfront_url()


def test_tokens_use_iso_date_string():

    """
    -> Test using the ISO Date String
    """

    team_id: str = str(uuid4())

    tomorrow = datetime.now() + relativedelta(days=1)
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
    token_id: str = list(json_dict.keys()).pop()
    values: dict = json_dict[token_id]

    assert expires == values["expires"]

    try:
        datetime.fromisoformat(values["created"])
    except ValueError:
        pytest.fail()


def test_token_creation_fail_if_expiration_date_is_before_creation():

    """
    -> Test that the expiration date comes after the creations date
    """

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

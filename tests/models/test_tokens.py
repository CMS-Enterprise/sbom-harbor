"""
-> Module for the code that tests Token model domain methods.
"""
from datetime import datetime, timedelta

from cyclonedx.model.token import Token

# from js: new Date().toISOString()
JAVASCRIPT_TO_ISO_STRING_TEMPLATE = "{year}-{month}-{day}T17:47:11.872Z"


def yesterday_naive():
    """
    Generates a python date for yesterday without timezone information.
    """
    return "{date}".format(date=datetime.now() - timedelta(days=1))


def yesterday_tz():
    """
    Generates a python date for yesterday with timezone information.
    """
    return "{date}".format(date=datetime.now().astimezone() - timedelta(days=1))


def tomorrow_naive():
    """
    Generates a python date for tomorrow without timezone information.
    """
    return "{date}".format(date=datetime.now() + timedelta(days=1))


def tomorrow_tz():
    """
    Generates a python date for tomorrow with timezone information.
    """
    return "{date}".format(date=datetime.now().astimezone() + timedelta(days=1))


def yesterday_iso_naive():
    """
    Generates a python iso 8601 formatted date for yesterday without timezone information.
    """
    return (datetime.now() - timedelta(days=1)).isoformat()


def yesterday_iso_tz():
    """
    Generates a python iso 8601 formatted date for yesterday with timezone information.
    """
    return (datetime.now().astimezone() - timedelta(days=1)).isoformat()


def tomorrow_iso_naive():
    """
    Generates a python iso 8601 formatted date for tomorrow without timezone information.
    """
    return (datetime.now() + timedelta(days=1)).isoformat()


def tomorrow_iso_tz():
    """
    Generates a python iso 8601 formatted date for tomorrow with timezone information.
    """
    return (datetime.now().astimezone() + timedelta(days=1)).isoformat()


def yesterday_javascript():
    """
    Generates a javascript toISOString formatted date for yesterday.
    By default, toISOString assumes UTC.
    """
    dt = datetime.now() - timedelta(days=1)
    return JAVASCRIPT_TO_ISO_STRING_TEMPLATE.format(
        year=dt.year, month=f"{dt.month:02d}", day=f"{dt.day:02d}"
    )


def tomorrow_javascript():
    """
    Generates a javascript toISOString formatted date for tomorrow.
    By default, toISOString assumes UTC.
    """
    dt = datetime.now() + timedelta(days=1)
    return JAVASCRIPT_TO_ISO_STRING_TEMPLATE.format(
        year=dt.year, month=f"{dt.month:02d}", day=f"{dt.day:02d}"
    )


def test_parse_expires():
    """
    -> Tests that we can compare an ISO 8601 date string to an rfc3339 date string.
    """

    test_cases = {
        "yesterday_javascript": {
            "expires": yesterday_javascript(),
            "is_expired": True,
        },
        "yesterday_tz": {
            "expires": yesterday_tz(),
            "is_expired": True,
        },
        "yesterday_naive": {
            "expires": yesterday_naive(),
            "is_expired": True,
        },
        "yesterday_iso_tz": {
            "expires": yesterday_iso_tz(),
            "is_expired": True,
        },
        "yesterday_iso_naive": {
            "expires": yesterday_iso_naive(),
            "is_expired": True,
        },
        "tomorrow_javascript": {
            "expires": tomorrow_javascript(),
            "is_expired": False,
        },
        "tomorrow_tz": {
            "expires": tomorrow_tz(),
            "is_expired": False,
        },
        "tomorrow_naive": {
            "expires": tomorrow_naive(),
            "is_expired": False,
        },
        "tomorrow_iso_tz": {
            "expires": tomorrow_iso_tz(),
            "is_expired": False,
        },
        "tomorrow_iso_naive": {
            "expires": tomorrow_iso_naive(),
            "is_expired": False,
        },
    }

    for key in test_cases:
        token = Token(
            team_id="test",
            token_id="test",
        )

        token.expires = test_cases[key]["expires"]
        assert token.is_expired() == test_cases[key]["is_expired"]


def test_can_validate_token():
    """
    -> Tests that we can properly calculate token validity.
    """

    test_cases = {
        "yesterday_javascript": {
            "expires": yesterday_javascript(),
            "enabled": True,
            "is_valid": False,
        },
        "yesterday_tz": {
            "expires": yesterday_tz(),
            "enabled": True,
            "is_valid": False,
        },
        "yesterday_naive": {
            "expires": yesterday_naive(),
            "enabled": True,
            "is_valid": False,
        },
        "yesterday_iso_tz": {
            "expires": yesterday_iso_tz(),
            "enabled": True,
            "is_valid": False,
        },
        "yesterday_iso_naive": {
            "expires": yesterday_iso_naive(),
            "enabled": True,
            "is_valid": False,
        },
        "tomorrow_javascript": {
            "expires": tomorrow_javascript(),
            "enabled": True,
            "is_valid": True,
        },
        "tomorrow_tz": {
            "expires": tomorrow_tz(),
            "enabled": True,
            "is_valid": True,
        },
        "tomorrow_naive": {
            "expires": tomorrow_naive(),
            "enabled": True,
            "is_valid": True,
        },
        "tomorrow_iso_tz": {
            "expires": tomorrow_iso_tz(),
            "enabled": True,
            "is_valid": True,
        },
        "tomorrow_iso_naive": {
            "expires": tomorrow_iso_naive(),
            "enabled": True,
            "is_valid": True,
        },
        "not_enabled": {
            "expires": tomorrow_iso_naive(),
            "enabled": False,
            "is_valid": False,
        },
    }

    for key in test_cases:
        token = Token(
            team_id="test",
            token_id="test",
        )

        token.expires = test_cases[key]["expires"]
        token.enabled = test_cases[key]["enabled"]

        assert token.is_valid() == test_cases[key]["is_valid"]

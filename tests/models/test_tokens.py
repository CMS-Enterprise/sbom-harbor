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

    yesterdays = {
        "yesterday_javascript": yesterday_javascript(),
        "yesterday_tz": yesterday_tz(),
        "yesterday_naive": yesterday_naive(),
        "yesterday_iso_tz": yesterday_iso_tz(),
        "yesterday_iso_naive": yesterday_iso_naive(),
    }

    tomorrows = {
        "tomorrow_javascript": tomorrow_javascript(),
        "tomorrow_tz": tomorrow_tz(),
        "tomorrow_naive": tomorrow_naive(),
        "tomorrow_iso_tz": tomorrow_iso_tz(),
        "tomorrow_iso_naive": tomorrow_iso_naive(),
    }

    successes = {}
    exceptions = {}

    print("\n")

    for key in yesterdays:
        try:
            token = Token(
                team_id="test",
                token_id="test",
            )
            print("input  {key} {expires}".format(key=key, expires=yesterdays[key]))
            token.expires = yesterdays[key]
            print("output {key} {expires}".format(key=key, expires=token.expires))
            assert token.is_expired()
            successes[key] = {
                "input": yesterdays[key],
                "output": token.expires,
            }
        # pylint: disable = W0703
        except Exception as e:
            exceptions[key] = e

    for key in tomorrows:
        try:
            token = Token(
                team_id="test",
                token_id="test",
            )
            print("input  {key} {expires}".format(key=key, expires=tomorrows[key]))
            token.expires = tomorrows[key]
            print("output {key} {expires}".format(key=key, expires=token.expires))
            assert not token.is_expired()
            successes[key] = {
                "input": tomorrows[key],
                "output": token.expires,
            }
        # pylint: disable = W0703
        except Exception as e:
            exceptions[key] = e

    for key in exceptions:
        print(
            "{key} failed with {exception}".format(key=key, exception=exceptions[key])
        )

    for key in successes:
        print(
            "{key} succeeded for\n input {input} \n output {output}".format(
                key=key, input=successes[key]["input"], output=successes[key]["output"]
            ),
        )

    assert len(exceptions) == 0


# def test_can_validate_token():
#     """
#     -> Tests that we can properly calculate token validity.
#     """
#
#     test_cases = []

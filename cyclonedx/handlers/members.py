"""
-> Unimplemented Handler functions for Members
"""
from cyclonedx.handlers.common import _print_values


def members_handler(event: dict, context: dict) -> dict:

    """
    -> Members handler.
    """

    _print_values(event, context)
    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": "",
    }


def member_handler(event: dict, context: dict) -> dict:

    """
    -> Member handler.
    """

    _print_values(event, context)
    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": "",
    }

"""
-> Module for OpenSSF Scorecard handler
"""


def openssf_scorecard_handler(event: dict = None, context: dict = None) -> dict:
    """
    This is the Lambda Handler that creates an openssf scorecard report from a specified git url
    """

    # TODO integrate the openssf tools
    # TODO how are we getting the giturl? Is it inside SBOM? Metadata?
    # TODO return json scorecard value.. somewhere, prob similar to dependency track

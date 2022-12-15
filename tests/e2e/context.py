"""
E2E Test micro-framework
"""
import os

# import subprocess
from json import loads
from uuid import uuid4

# from cyclonedx.model.team import Team
from tests.e2e import get_cloudfront_url, login


# pylint: disable = R0902
class TestContext:
    """
    TestContext that supports E2E tests being run independently or from CI.
    """

    def __init__(self, fixture: str, with_team: True):
        self.cf_url: str = get_cloudfront_url()
        self.jwt: str = login(self.cf_url)
        self.team_url = f"{self.cf_url}/api/v1/team"

        self.with_team = with_team
        self.context_id = str(uuid4())
        self.fixture = open(
            os.path.join(os.path.dirname(__file__), "fixtures", f"{fixture}.json"), "r"
        ).read()
        self.test_cases = loads(self.fixture)

        # if self.with_team:
        #     self.team: Team = self.create_team()

    # def create_team(self):
    #
    #     """
    #     -> Create a team
    #     """
    #
    #     body = self.test_cases["create-team"]
    #     body["name"] = f"{body['name']}-{self.context_id}"
    #     # resp = self.post(self.team_url, body)

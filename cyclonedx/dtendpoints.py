"""
Module to hold constant values for the API
"""

import os
from cyclonedx.constants import DT_API_BASE

# Found in DT: Projects -> <project> -> "View Details" (Tiny as hell)
# At the bottom it says: "Object Identifier".  That's it.
PROJECT_UUID = "acd68120-3fec-457d-baaa-a456a39984de"


class DTEndpoints:

    """
    Class to generate endpoints
    """

    # Must be configurable
    base = f"http://{os.getenv(DT_API_BASE)}"

    @staticmethod
    def get_sbom_status(token):

        """
        Returns Endpoint to check if an SBOM has been analyzed and the findings are ready
        """

        return f"{DTEndpoints.base}/v1/bom/token/{token}"

    @staticmethod
    def post_sbom():

        """
        Returns Endpoint used to upload an SBOM
        """

        return f"{DTEndpoints.base}/v1/bom"

    @staticmethod
    def get_findings():

        """
        Returns Endpoint used to retrieve finds for a project
        """

        return f"{DTEndpoints.base}/v1/finding/project/{PROJECT_UUID}/export"

    @staticmethod
    def create_project():

        """
        Returns Endpoint used to create projects in DT
        """

        return f"{DTEndpoints.base}/v1/project"

    @staticmethod
    def force_chg_pwd():

        """
        Returns Endpoint used to force change a password
        """

        return f"{DTEndpoints.base}/v1/user/forceChangePassword"

    @staticmethod
    def do_login():

        """
        Returns Endpoint used to log into DT
        """

        return f"{DTEndpoints.base}/v1/user/login"

    @staticmethod
    def add_permission_to_team(perm, uuid):

        """
        Returns Endpoint used to list all the data DT has for teams
        """

        return f"{DTEndpoints.base}/v1/{perm}/addPermissionToTeam/{uuid}"

    @staticmethod
    def get_teams_data():

        """
        Returns Endpoint used to list all the data DT has for teams
        """

        return f"{DTEndpoints.base}/v1/team"

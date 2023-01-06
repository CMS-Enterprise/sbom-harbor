"""
Module to hold constant values for the API
"""

import os
from deploy.constants import DT_API_BASE, DT_API_PORT


class DTEndpoints:

    """
    Class to generate endpoints
    """

    # Must be configurable
    base = f"http://{os.getenv(DT_API_BASE)}:{DT_API_PORT}/api"

    @staticmethod
    def get_sbom_status(sbom_token):

        """
        Returns Endpoint to check if an SBOM has been analyzed and the findings are ready
        """

        return f"{DTEndpoints.base}/v1/bom/token/{sbom_token}"

    @staticmethod
    def post_sbom():

        """
        Returns Endpoint used to upload an SBOM
        """

        return f"{DTEndpoints.base}/v1/bom"

    @staticmethod
    def get_findings(project_uuid: str):

        """
        Returns Endpoint used to retrieve finds for a project
        """

        return f"{DTEndpoints.base}/v1/finding/project/{project_uuid}/export"

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

        return f"{DTEndpoints.base}/v1/permission/{perm}/team/{uuid}"

    @staticmethod
    def get_teams_data():

        """
        Returns Endpoint used to list all the data DT has for teams
        """

        return f"{DTEndpoints.base}/v1/team"

    @staticmethod
    def rotate_api_key(api_key):

        """
        Returns Endpoint used to rotate the api keys
        """

        return f"{DTEndpoints.base}/v1/team/key/{api_key}"

    @staticmethod
    def delete_project(project_uuid: str):

        """
        Deletes Project
        """

        return f"{DTEndpoints.base}/v1/project/{project_uuid}"

    @staticmethod
    def get_dt_version():

        """
        Deletes Project
        """

        return f"{DTEndpoints.base}/version"

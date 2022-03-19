"""
Module to hold constant values for the API
"""

import os

# Found in DT: Projects -> <project> -> "View Details" (Tiny as hell)
# At the bottom it says: "Object Identifier".  That's it.
PROJECT_UUID = "acd68120-3fec-457d-baaa-a456a39984de"


class DTEndpoints:

    """
    Class to generate endpoints
    """

    # Must be configurable
    dt_api_base = os.getenv("TD_API_BASE")

    @staticmethod
    def get_sbom_status(token):

        """
        Returns Endpoint to check if an SBOM has been analyzed and the findings are ready
        """

        return f"{DTEndpoints.dt_api_base}/v1/bom/token/{token}"

    @staticmethod
    def post_sbom():

        """
        Returns Endpoint used to upload an SBOM
        """

        return f"{DTEndpoints.dt_api_base}/v1/bom"

    @staticmethod
    def get_findings():

        """
        Returns Endpoint used to retrieve finds for a project
        """

        return f"{DTEndpoints.dt_api_base}/v1/finding/project/{PROJECT_UUID}/export"

    @staticmethod
    def create_project():

        """
        Returns Endpoint used to create projects in DT
        """

        return f"{DTEndpoints.dt_api_base}/v1/project"

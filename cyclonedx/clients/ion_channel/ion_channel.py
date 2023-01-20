"""
-> A module to house the Ion Channel Client and supporting functions
"""
import logging
from logging import config
from time import sleep
from typing import IO, Union

import boto3
import requests
from botocore.client import BaseClient
from botocore.exceptions import ClientError
from requests import Response

from cyclonedx.constants import IC_API_KEY, IC_RULESET_TEAM_ID, PYTHON_LOGGING_CONFIG
from cyclonedx.exceptions.ion_channel import IonChannelError

config.fileConfig(PYTHON_LOGGING_CONFIG)
logger = logging.getLogger(__name__)

# pylint: disable = R0902


# pylint: disable = E1136
def get_ic_urls() -> tuple:

    """
    -> All the Ion Channel Endpoints
    """

    export: list = []

    api_base: str = "api.ionchannel.io"
    api_base_url: str = f"https://{api_base}"

    org_path: str = f"{api_base_url}/v1/organizations"
    export.append(f"{org_path}/getOwnOrganizations")

    ruleset_path = f"{api_base_url}/v1/ruleset"
    export.append(f"{ruleset_path}/getRulesets")

    project_path: str = f"{api_base_url}/v1/project"
    export.append(f"{project_path}/getSBOMs")
    export.append(f"{project_path}/createSBOM")
    export.append(f"{project_path}/importSBOM")
    export.append(f"{project_path}/saveConfirmSBOM")
    export.append(f"{project_path}/updateProjects")

    report_path: str = f"{api_base_url}/v1/report"
    export.append(f"{report_path}/getProjects")
    export.append(f"{report_path}/getAnalysis")
    export.append(f"{report_path}/getVulnerabilityList")

    scanner_path: str = f"{api_base_url}/v1/scanner"
    export.append(f"{scanner_path}/getAnalysisStatus")

    animal_path: str = f"{api_base_url}/v1/animal"
    export.append(f"{animal_path}/getLatestAnalysis")

    return tuple(export)


class IonChannelClient:

    """
    -> The ICClient is a client for Ion Channel
    -> Create a client by passing in the name of the SBOM/IC Team in Ion Channel.
    """

    @staticmethod
    def __get_parameter(parameter_name: str) -> str:

        """
        -> Extracts a value from AWS Parameter Store
        """

        ssm_client: BaseClient = boto3.client("ssm")

        try:
            gp_resp: dict = ssm_client.get_parameter(
                Name=parameter_name,
                WithDecryption=False,
            )
            return gp_resp["Parameter"]["Value"]
        except ClientError as ce:
            err: str = "Error talking to AWS to get parameters"
            raise IonChannelError(err) from ce
        except KeyError as ke:
            err: str = f"Error extracting parameter values for {parameter_name}"
            raise IonChannelError(err) from ke

    def __get(self, url: str) -> dict:

        """
        -> Execute an http get request
        """

        logger.info("Sending To: GET:%s", url)
        response: Response = requests.get(
            url,
            headers=self.headers,
        )

        try:
            rsp_json: dict = response.json()
        except Exception as e:
            print(f"Error trying to GET({url}).  Reply is: {response.text}")
            raise ValueError from e

        return rsp_json

    def __put(self, url: str, json: dict) -> dict:

        """
        -> Execute an http put request
        """

        logger.info("Sending To: PUT:%s", url)
        response: Response = requests.put(
            url,
            headers=self.headers,
            json=json,
        )

        return response.json()

    def __post(
        self,
        url: str,
        json: Union[list, dict] = None,
        files: dict = None,
    ) -> dict:

        """
        -> Execute an http post request
        """

        logger.info("Sending To: POST:%s", url)
        if json:
            response: Response = requests.post(
                url,
                headers=self.headers,
                json=json,
            )
        elif files:
            response: Response = requests.post(
                url,
                headers=self.headers,
                files=files,
            )
        else:
            response: Response = requests.post(
                url,
                headers=self.headers,
            )

        return response.json()

    def __create_ic_urls(self):

        """
        -> All the Ion Channel Endpoints
        """

        (
            self.get_own_org_url,
            self.get_rulesets_url,
            self.get_sboms_url,
            self.create_sbom_url,
            self.import_sbom_url,
            self.save_sbom_url,
            self.update_projects_url,
            self.get_projects_url,
            self.get_analysis_url,
            self.get_vuln_list_url,
            self.get_analysis_status_url,
            self.get_last_analysis_url,
            *self.nothing,
        ) = get_ic_urls()

    def __get_ssm_parameters(self):

        """Extracts the Ion Channel parameters in the AWS Parameter Store"""

        # Get the API key we defined in Ion Channel, so we can
        # make authorized RESTful calls.
        self.api_key: str = self.__get_parameter(IC_API_KEY)

        # Get the team id we use to get the ruleset
        # we use for requests
        self.ruleset_team_id: str = self.__get_parameter(IC_RULESET_TEAM_ID)

    def __set_ic_request_headers(self):

        """Create the headers we will use when making RESTful Calls to Ion Channel
        The headers need the JWT token we created initially.
        """

        self.headers = {
            "Authorization": f"Bearer {self.api_key}",
        }

    def __get_org_id(self):

        """
        -> Get the Organization id from Ion Channel.
        """

        self.org_id: str = self.__get_id(
            obj_name="organization",
            url=self.get_own_org_url,
        )

    def __get_ruleset_id(self):

        """Gets the id of the ruleset associated to the CMS1 Team
        We may need to look harder at what rule set we use going forward,
        but for now this default ruleset works.
        """

        self.ruleset_id: str = self.__get_id(
            url=f"{self.get_rulesets_url}?team_id={self.ruleset_team_id}",
        )

    def __get_sbom_data(self, team_name: str, create_if_missing: bool):

        # Get all of our existing SBOMs in Ion Channel
        get_sboms_rsp: dict = self.__get(
            f"{self.get_sboms_url}?org_id={self.org_id}"
        )

        response_data: dict = get_sboms_rsp["data"]
        software_lists: list = response_data["softwareLists"]

        ic_sbom_list = list(
            filter(
                lambda item: item["name"] == team_name,
                software_lists,
            )
        )

        if len(ic_sbom_list) == 0:
            self._already_exists: bool = False
            if create_if_missing:
                self.sbom_data = self.create_sbom(team_name)
        elif len(ic_sbom_list) == 1:
            self._already_exists: bool = True
            self.sbom_data = ic_sbom_list.pop()
        else:
            self.sbom_data = "ERROR"
            raise ValueError(f"Team: {team_name} exists more than once in Ion Channel")

    def __get_id(self, url: str, obj_name: str = None):

        print(f"Attempting to get id with url: {url}")

        rsp_json = self.__get(url)

        try:
            data_arr = rsp_json["data"]
        except KeyError as ke:
            print(f"<json-response data='{rsp_json}' />")
            raise ValueError from ke

        if len(data_arr) < 1:
            raise IonChannelError(
                f"The user used to log in has no teams in ion channel: {data_arr}"
            )

        if len(data_arr) > 1:
            raise IonChannelError(
                f"The user used to log in has too many teams in ion channel: {data_arr}"
            )

        obj = data_arr[0][obj_name] if obj_name else data_arr[0]

        return obj["id"]

    def __get_projects_ids(self):

        # Get Projects
        team_id: str = self.sbom_data["team_id"]
        get_projects_url: str = f"{self.get_projects_url}?team_id={team_id}"
        get_projects_json: dict = self.__get(get_projects_url)
        response_data: list[dict] = get_projects_json["data"]

        active_projects: list[dict] = list(
            filter(
                lambda project: project["active"],
                response_data,
            )
        )

        project_ids: list[dict] = []
        for active_project in active_projects:
            team_id: str = active_project["team_id"]
            project_id: str = active_project["id"]
            analysis_summary = active_project["analysis_summary"]
            analysis_id: str = (
                "" if analysis_summary is None else analysis_summary["analysis_id"]
            )
            project_ids.append(
                {
                    "team_id": team_id,
                    "project_id": project_id,
                    "analysis_id": analysis_id,
                }
            )

        return project_ids

    def __get_analysis_id(self, project_id: str):

        """
        -> Get the analysis ID from Ion Channel
        """

        # The team ID is from the initial request
        team_id: str = self.sbom_data["team_id"]

        # Add the Team ID to the URL
        la_url_plus_team_id: str = f"{self.get_last_analysis_url}?team_id={team_id}"

        # Add the project ID
        la_url_complete: str = f"{la_url_plus_team_id}&project_id={project_id}"

        # Make the request
        project_history_rsp: dict = self.__get(la_url_complete)

        try:
            data: dict = project_history_rsp["data"]
            return data["analysis_id"]
        except KeyError as ke:
            raise ValueError("Unable to get analysis id") from ke

    def __get_analysis(self, project_id: str):

        analysis_id: str = self.__get_analysis_id(project_id)

        # The team ID is from the initial request
        team_id: str = self.sbom_data["team_id"]

        analysis_url_pti: str = f"{self.get_analysis_url}?team_id={team_id}"
        analysis_url_ppi: str = f"{analysis_url_pti}&project_id={project_id}"
        analysis_url_final: str = f"{analysis_url_ppi}&analysis_id={analysis_id}"
        report_json: dict = self.__get(analysis_url_final)

        try:
            report_data: dict = report_json["data"]
            report_analysis: dict = report_data["analysis"]
            component_name: str = report_analysis["name"]
            sbom_source: str = report_analysis["source"]
            passing: bool = report_analysis["passed"]

            results: list = [
                [analysis["results"]["type"], analysis["results"]["data"]]
                for analysis in report_analysis["scan_summaries"]
            ]

            return {
                "name": component_name,
                "source": sbom_source,
                "passing": passing,
                "results": results,
            }
        except KeyError as ke:
            logger.info("ERROR, no 'name' key: %s, %s", report_json, ke)
            return (
                f"Error:analysis_id({analysis_id})",
                f"Error:project_id({project_id})",
            )

    def __init__(self, team_name: str, create_if_missing: bool = False):

        # The URL that we use for all Ion Channel REST calls
        self.__create_ic_urls()

        try:
            self.__get_ssm_parameters()
        except ClientError as ce:
            raise IonChannelError from ce

        self.__set_ic_request_headers()
        self.__get_org_id()
        self.__get_ruleset_id()
        self.__get_sbom_data(team_name, create_if_missing)

    @property
    def already_exists(self):

        """
        -> True if already exists
        """

        return self._already_exists

    def archive_existing_projects(self):

        """
        -> Archive existing projects in Ion Channel to prepare for a new
        -> SBOM to be uploaded and create new projects of the same name.
        """

        project_ids = [pid["project_id"] for pid in self.__get_projects_ids()]

        response: dict = self.__put(
            f"{self.update_projects_url}?archive=true",
            {
                "project_ids": project_ids,
            },
        )

        response_data = response["data"]
        failures = response_data["failed"]

        return len(failures) == 0

    def create_sbom(self, name: str):

        """
        -> Creates an SBOM in Ion Channel
        -> This should only be done when registering a Team in the system.
        -> After an SBOM is created, it will render as a "Team" in Ion Channel.
        -> After a team is created by creating an SBOM, one must import subsequent
        -> SBOMs into that team using the import endpoint.
        """

        create_rsp_json = self.__post(
            self.create_sbom_url,
            {
                "name": name,
                "version": "0.0.1",
                "supplier_name": "CMS",
                "contact_name": "Quinn Peters",  # TODO
                "contact_email": "quinn.peters@aquia.io",  # TODO
                "monitor_frequency": "daily",
                "org_id": self.org_id,
                "ruleset_id": self.ruleset_id,
            },
        )

        return create_rsp_json["data"]

    def import_sbom(self, sbom_fh: IO):

        """
        -> Import an SBOM into Ion Channel
        """

        sbom_id: str = self.sbom_data["id"]

        self.__post(
            f"{self.import_sbom_url}?sbom_id={sbom_id}",
            files={"file": sbom_fh},
        )

        self.__post(f"{self.save_sbom_url}?id={sbom_id}")

    def monitor_sbom_analysis(self):

        """
        -> Observe when Ion Channel is finished analyzing the SBOM
        """

        analyses: list[dict] = self.__get_projects_ids()

        complete = False
        finished_projects: list[str] = []
        errored_projects: list[str] = []
        while not complete:

            # We hope it's done.
            complete = True

            analysis: dict
            for analysis in analyses:

                team_id: str = analysis["team_id"]
                project_id: str = analysis["project_id"]
                analysis_id: str = analysis["analysis_id"]

                # Do not analyze this project if it has
                # already completed or failed
                if project_id in finished_projects or project_id in errored_projects:
                    continue

                gas_url_add_aid: str = (
                    f"{self.get_analysis_status_url}?id={analysis_id}"
                )
                gas_url_add_tid = f"{gas_url_add_aid}&team_id={team_id}"
                gas_url_complete = f"{gas_url_add_tid}&project_id={project_id}"
                status_rsp_dict: dict = self.__get(gas_url_complete)
                analysis: str = status_rsp_dict["data"]["status"]

                if analysis == "finished":
                    finished_projects.append(project_id)
                elif analysis == "errored":
                    errored_projects.append(project_id)
                else:
                    complete = False
                    logger.info(status_rsp_dict["data"]["message"])

                    # Try not to hammer the Ion Channel API
                    sleep(1)

            logger.info("Extracting data from %s projects", len(finished_projects))
            logger.info("%s projects are failing", len(errored_projects))

    def get_report(self):

        """
        -> Extract the report from Ion Channel
        """

        analyses: list = [
            self.__get_analysis(project_id)
            for project_id in [pid["project_id"] for pid in self.__get_projects_ids()]
        ]

        team_id: str = self.sbom_data["team_id"]

        vuln_url: str = f"{self.get_vuln_list_url}?id={team_id}"
        vuln_data: dict = self.__get(vuln_url)

        report: dict = {
            "analyses": analyses,
            "vulnerabilities": vuln_data,
        }

        return report

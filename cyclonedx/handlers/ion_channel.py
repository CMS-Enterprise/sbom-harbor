"""
-> A module to house the Ion Channel Client and supporting functions
"""
from time import sleep
from typing import IO

import boto3
import requests
from botocore.client import BaseClient
from requests import Response
from urllib3.exceptions import ResponseError

from cyclonedx.constants import (
    IC_API_BASE,
    IC_API_KEY,
    IC_RULESET_TEAM_ID,
)

# pylint: disable = R0902
class ICClient:

    """
    -> The ICClient is a client for Ion Channel
    -> Create a client by passing in the name of the SBOM/IC Team in Ion Channel.
    """

    @staticmethod
    def __get_parameter(parameter_name: str):

        """Extracts a value from AWS Parameter Store"""

        ssm: BaseClient = boto3.client("ssm")

        return ssm.get_parameter(Name=parameter_name, WithDecryption=False,)[
            "Parameter"
        ]["Value"]

    def __get_ssm_parameters(self):

        """Extracts the Ion Channel parameters in the AWS Parameter Store"""

        # The URL that we use for all Ion Channel REST calls
        self.api_base: dict = self.__get_parameter(IC_API_BASE)

        # Get the API key we defined inIon Channel so we can
        # make authorized RESTful calls.
        self.api_key: dict = self.__get_parameter(IC_API_KEY)

        # Get the team id we use to get the ruleset
        # we use for requests
        self.ruleset_team_id: dict = self.__get_parameter(IC_RULESET_TEAM_ID)

    def __get_ic_request_headers(self):

        """Create the headers we will use when making RESTful Calls to Ion Channel
        The headers need the JWT token we created initially.
        """

        self.headers = {"Authorization": f"Bearer {self.api_key}"}

    def __get_org_id(self):

        """Get the Organization id from Ion Channel."""

        ic_org_api_path = f"{self.api_base}/v1/organizations"
        goo_path = f"{ic_org_api_path}/getOwnOrganizations"
        goo_url = f"https://{goo_path}"
        self.org_id: str = self.__get_id(
            obj_name="organization",
            url=goo_url,
        )

    def __get_ruleset_id(self):

        """Gets the id of the ruleset associated to the CMS1 Team
        We may need to look harder at what rule set we use going forward,
        but for now this default ruleset works.
        """

        ic_ruleset_api_path = f"{self.api_base}/v1/ruleset"
        get_rs_url = (
            f"https://{ic_ruleset_api_path}/getRulesets?team_id={self.ruleset_team_id}"
        )
        self.ruleset_id: str = self.__get_id(
            url=get_rs_url,
        )

    def __get_sbom_id(self, team_name: str, create_if_missing: bool):

        project_path = f"{self.api_base}/v1/project"
        get_sboms_url = f"https://{project_path}/getSBOMs?org_id={self.org_id}"

        print(f"Sending To: GET:{get_sboms_url}")
        get_sboms_rsp = requests.get(
            get_sboms_url,
            headers=self.headers,
        )

        response: dict = get_sboms_rsp.json()

        print(f"<__get_sbom_id response={response} />")

        response_data = response["data"]
        software_lists = response_data["softwareLists"]

        def filter_func(item):  # TODO Change it back to the Lambda
            print(f"<(SBOM|TEAM) item={item} looking for: name={team_name} />")
            return item["name"] == team_name

        ic_sbom_list = list(
            filter(
                # TODO Change it back to the Lambda
                # lambda item: item['name'] == team_name,
                filter_func,
                software_lists,
            )
        )

        if len(ic_sbom_list) == 1:
            ic_sbom = ic_sbom_list.pop()
            (self.sbom_id, self.sbom_team_id) = (ic_sbom["id"], ic_sbom["team_id"])
        elif create_if_missing:
            (self.sbom_id, self.sbom_team_id) = self.create_sbom(team_name)
        else:
            self.sbom_id = "ERROR"
            raise ValueError(f"Team: {team_name} does not exist in Ion Channel")

        # print(dumps(ic_sbom_list, indent=2, sort_keys=True))

    def __get_id(self, url: str, obj_name: str = None):

        rsp = requests.get(
            url,
            headers=self.headers,
        )

        rsp_json = rsp.json()
        data_arr = rsp_json["data"]

        if len(data_arr) > 1:
            raise ResponseError(f"Too many data(s), should only be one: {data_arr}")

        obj = data_arr[0][obj_name] if obj_name else data_arr[0]

        return obj["id"]

    def __get_all_active_project_ids(self):

        # Get Projects
        ic_project_api_path = f"{self.api_base}/v1/report"
        get_projects_url = (
            f"https://{ic_project_api_path}/getProjects?team_id={self.sbom_team_id}"
        )

        print(f"Sending To: GET:{get_projects_url}")
        get_projects_rsp: Response = requests.get(
            get_projects_url,
            headers=self.headers,
        )

        response = get_projects_rsp.json()
        response_data = response["data"]

        return [p["id"] for p in filter(lambda p: p["active"], response_data)]

    def __init__(self, team_name: str, create_if_missing: bool = False):

        self.__get_ssm_parameters()
        self.__get_ic_request_headers()
        self.__get_org_id()
        self.__get_ruleset_id()
        self.__get_sbom_id(team_name, create_if_missing)

    def archive_existing_projects(self):

        """
        -> Archive existing projects in Ion Channel to prepare for a new
        -> SBOM to be uploaded and create new projects of the same name.
        """

        project_ids = self.__get_all_active_project_ids()
        print(f"<NumIds order='1' value='{len(project_ids)}' />")

        # Report API
        update_projects_path = f"{self.api_base}/v1/project"
        update_projects_url = (
            f"https://{update_projects_path}/updateProjects?archive=true"
        )

        print(f"Sending To: PUT:{update_projects_url}")
        update_projects_rsp = requests.put(
            update_projects_url, headers=self.headers, json={"project_ids": project_ids}
        )

        response = update_projects_rsp.json()
        response_data = response["data"]
        failures = response_data["failed"]

        print(
            f"<UpdateProjectsResponse #succeeded='{len(response_data['succeeded'])}' />"
        )
        print(f"<UpdateProjectsResponse #failed='{len(response_data['failed'])}' />")

        return len(failures) == 0

    def create_sbom(self, name: str):

        """
        -> Creates an SBOM in Ion Channel
        -> This should only be done when registering a Team in the system.
        -> After an SBOM is created, it will render as a "Team" in Ion Channel.
        -> After a team is created by creating an SBOM, one must import subsequent
        -> SBOMs into that team using the import endpoint.
        """

        ic_proj_api_path = f"{self.api_base}/v1/project"
        create_sbom_url = f"https://{ic_proj_api_path}/createSBOM"

        print(f"Sending To: POST:{create_sbom_url}")
        create_rsp = requests.post(
            create_sbom_url,
            headers=self.headers,
            json={
                "name": name,
                "version": "0.0.1",  # TODO
                "supplier_name": "aquia",  # TODO
                "contact_name": "qtpeters",  # TODO
                "contact_email": "quinn.peters@aquia.io",  # TODO
                "monitor_frequency": "daily",
                "org_id": self.org_id,
                "ruleset_id": self.ruleset_id,
            },
        )

        create_rsp_json = create_rsp.json()
        print(f"Create Response: {create_rsp_json}")

        sbom_id = create_rsp_json["data"]["id"]
        sbom_team_id = create_rsp_json["data"]["team_id"]

        return sbom_id, sbom_team_id

    def get_analysis_id(self, project_id: str):

        """
        -> Get the analysis ID from Ion Channel
        """

        # /?team_id=[TEAM_ID]&project_id=[PROJECT_ID]
        ic_ruleset_api_path = f"{self.api_base}/v1/animal"
        url: str = f"https://{ic_ruleset_api_path}/getLatestAnalysis"
        latest_analysis_url = (
            f"{url}?team_id={self.sbom_team_id}&project_id={project_id}"
        )

        print(f"Sending To: GET:{latest_analysis_url}")
        project_history_rsp = requests.get(
            latest_analysis_url,
            headers=self.headers,
        )

        return project_history_rsp.json()

    def import_sbom(self, sbom_fh: IO):

        """
        -> Import an SBOM into Ion Channel
        """

        ic_proj_api_path = f"{self.api_base}/v1/project"
        import_sbom_url = (
            f"https://{ic_proj_api_path}/importSBOM?sbom_id={self.sbom_id}"
        )

        print(f"Sending To: POST:{import_sbom_url}")
        import_rsp = requests.post(
            import_sbom_url,
            headers=self.headers,
            files={"file": sbom_fh},
        )

        irj = import_rsp.json()
        irj_data = irj["data"]

        print(f"<SBOMImportResponse sbom-id='{irj_data['id']}' />")
        print(f"<SBOMImportResponse sbom-name='{irj_data['name']}' />")

        save_sbom_url = f"https://{ic_proj_api_path}/saveConfirmSBOM?id={self.sbom_id}"

        print(f"Sending To: POST:{save_sbom_url}")
        save_rsp = requests.post(
            save_sbom_url,
            headers=self.headers,
        )

        save_rsp_json = save_rsp.json()
        print(f"Save Response: {save_rsp_json}")

    def analyze_sbom(self):

        """
        -> Tell Ion Channel to get to work analyzing the SBOM
        """

        project_ids = self.__get_all_active_project_ids()
        individually_wrapped_project_ids = list(
            map(lambda p_id: {"project_id": p_id}, project_ids)
        )

        # # Scanner API
        ic_scanner_api_path = f"{self.api_base}/v1/scanner"

        print(f"<NumIds order='2' value='{len(project_ids)}' />")

        analyze_proj_url = f"https://{ic_scanner_api_path}/analyzeProjects"
        print(f"Sending To: POST:{analyze_proj_url}")
        analyze_projects_rsp: Response = requests.post(
            analyze_proj_url,
            headers=self.headers,
            json=individually_wrapped_project_ids,
        )

        response_dict: dict = analyze_projects_rsp.json()
        print(
            f"<AnalysisRequestStatusCode response={analyze_projects_rsp.status_code} />"
        )
        print(f"<AnalysisRequestResponse response={response_dict} />")

        analysis_successes: list = response_dict["data"]["succeeded"]
        analysis_status_url = f"https://{ic_scanner_api_path}/getAnalysisStatus"

        complete = False

        while not complete:

            # We hope it's done.
            complete = True

            for analysis in analysis_successes:

                analysis_id: str = analysis["id"]
                team_id: str = analysis["team_id"]
                project_id: str = analysis["project_id"]

                url_with_static_qp = (
                    f"{analysis_status_url}?id={analysis_id}&team_id={team_id}"
                )
                url = f"{url_with_static_qp}&project_id={project_id}"
                print(f"Sending To: POST:{url}")
                status_rsp: Response = requests.get(
                    url,
                    headers=self.headers,
                )

                status_rsp_dict: dict = status_rsp.json()

                print(f"<GetAnalysisStatus value='{status_rsp_dict}' />")

                # Can at least be 'finished' or 'queued'
                analysis: str = status_rsp_dict["data"]["status"]

                if analysis == "queued":

                    # We find out that it isn't done
                    complete = False

            if not complete:
                print(
                    "Not complete.  Queued projects exist, sleeping for 3s before checking again"
                )
                sleep(3)

        print(f"Analysis Response: {response_dict}")

    def report_ready(self):

        """
        -> Verify whether a report is ready in Ion Channel
        """

        # Report API
        ic_report_api_path = f"{self.api_base}/v1/report"
        analyze_proj_url = f"https://{ic_report_api_path}/getAnalysis"

        print(f"Sending To: GET:{analyze_proj_url}")
        get_projects_rsp = requests.get(
            analyze_proj_url,
            headers=self.headers,
        )

        print(get_projects_rsp.json())

    def get_report(self):

        """
        -> Extract the report from Ion Channel
        """

        # Report API
        ic_report_api_path = f"{self.api_base}/v1/report"
        analyze_proj_url = f"https://{ic_report_api_path}/getAnalysis"

        print(f"Sending To: GET:{analyze_proj_url}")
        get_projects_rsp = requests.get(
            analyze_proj_url,
            headers=self.headers,
        )

        print(get_projects_rsp.json())

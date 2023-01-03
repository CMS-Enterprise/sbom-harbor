"""
-> Module for the Default Enrichment Source Handler
"""
import logging
from json import dumps, loads
from logging import config
from time import sleep

import requests
from boto3 import resource

from cyclonedx.constants import PYTHON_LOGGING_CONFIG
from cyclonedx.handlers.common import _get_sbom

config.fileConfig(PYTHON_LOGGING_CONFIG)
logger = logging.getLogger(__name__)


def des_interface_handler(event: dict = None, context: dict = None):

    """
    -> Handler for the Default Enrichment Source
    """

    s3_resource = resource("s3")

    logger.info("event value= %s", event)
    all_data = _get_sbom(event)
    sbom = loads(all_data["data"].read())
    sbom_name = all_data["s3_obj_name"]
    bucket_name = all_data["bucket_name"]
    findings_file_name = f"findings-des-{sbom_name}"

    nvd_base_url = "https://services.nvd.nist.gov"
    nvd_api_path = "/rest/json/cpes/1.0"

    findings = []

    components: list = sbom["components"]
    components_seen = 0

    api_keys = [
        "7e762116-c587-4a4b-9eb4-f7b5fef84024",
        "3f501a51-373a-4a11-9a5d-6f691b522adc",
        "2d2a6475-2f19-4876-9a03-ba8de575d477",
        "99eaef67-ca08-423f-92ac-6e79a4a4bae9",
        "d0844f1d-ff53-4a6b-82ab-3ee143198311",
    ]

    for component in components[:10]:  # TODO Remove Slice

        components_seen += 1
        logger.info("Looking at component# %s of %s", components_seen, len(components))

        vendor = "*"
        product = component["name"]
        version = component["version"]

        key = api_keys[components_seen % len(api_keys)]
        logger.info("Request Key: %s", key)
        cpe_search_str = f"cpe:2.3:a:{vendor}:{product}:{version}"
        nvd_query_params = f"?addOns=cves&cpeMatchString={cpe_search_str}&apiKey={key}"
        nvd_url = f"{nvd_base_url}/{nvd_api_path}/{nvd_query_params}"

        nvd_response = requests.get(nvd_url)

        if nvd_response.status_code == 403:
            logger.info("Hit NVD Administrative limit, backing off for 10 seconds.")
            components.append(component)
            sleep(10)
            continue

        nvd_rsp_json = nvd_response.json()
        num_results = nvd_rsp_json["totalResults"]

        if num_results > 0:
            logger.info("Results: %s", num_results)
            findings.append(nvd_rsp_json)
        else:
            logger.info("No Results")

    logger.info("Made it out of the loop!!!")

    # logger.info(dumps(response.json(), indent=2))
    # Dump the findings into a byte array and store them
    # in the S3 bucket along with the SBOM the findings
    # came from.
    findings_bytes = bytearray(dumps(findings), "utf-8")
    s3_resource.Object(bucket_name, findings_file_name).put(
        Body=findings_bytes,
    )

    return findings_file_name

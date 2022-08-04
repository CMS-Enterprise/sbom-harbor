from json import loads, dumps
from time import sleep

import requests
from boto3 import resource

from cyclonedx.core_utils.cyclonedx_util import __get_all_s3_obj_data


def des_interface_handler(event: dict = None, context: dict = None):

    s3_resource = resource("s3")

    print(f"<event value='{event}' />")
    all_data = __get_all_s3_obj_data(event)
    sbom = loads(all_data['data'].read())
    sbom_name = all_data['s3_obj_name']
    bucket_name = all_data['bucket_name']
    findings_file_name = f"findings-des-{sbom_name}"

    nvd_base_url = "https://services.nvd.nist.gov"
    nvd_api_path = "/rest/json/cpes/1.0"

    findings = []

    components: list = sbom["components"]
    components_seen = 0

    api_keys = [
        '7e762116-c587-4a4b-9eb4-f7b5fef84024',
        '3f501a51-373a-4a11-9a5d-6f691b522adc',
        '2d2a6475-2f19-4876-9a03-ba8de575d477',
        '99eaef67-ca08-423f-92ac-6e79a4a4bae9',
        'd0844f1d-ff53-4a6b-82ab-3ee143198311',
    ]

    for component in components[:100]: # TODO Remove Slice

        components_seen += 1
        print(f"Looking at component# {components_seen} of {len(components)}")

        vendor = "*"
        product = component["name"]
        version = component["version"]

        key = api_keys[ components_seen % len(api_keys) ]
        print(f"Request Key: {key}")
        cpe_search_str = f"cpe:2.3:a:{vendor}:{product}:{version}"
        nvd_query_params = f"?addOns=cves&cpeMatchString={cpe_search_str}&apiKey={key}"
        nvd_url = f"{nvd_base_url}/{nvd_api_path}/{nvd_query_params}"

        nvd_response = requests.get(nvd_url)

        if nvd_response.status_code == 403:
            print("Hit NVD Administrative limit, backing off for 10 seconds.")
            components.append(component)
            sleep(10)
            continue

        nvd_rsp_json = nvd_response.json()
        num_results = nvd_rsp_json['totalResults']

        if num_results > 0:
            print(f"# Results: {num_results}")
            findings.append(nvd_rsp_json)
        else:
            print("No Results")

    print("Made it out of the loop!!!")

    # print(dumps(response.json(), indent=2))
    # Dump the findings into a byte array and store them
    # in the S3 bucket along with the SBOM the findings
    # came from.
    findings_bytes = bytearray(dumps(findings), "utf-8")
    s3_resource.Object(bucket_name, findings_file_name).put(
        Body=findings_bytes,
    )

    return findings_file_name

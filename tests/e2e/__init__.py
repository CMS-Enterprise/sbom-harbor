"""
-> init file for e2e tests module
"""
from json import dumps

import boto3
import requests
from requests import Response


def print_response(response: Response):

    """
    -> Uniform response so we know what to look for
    """

    status_code: int = response.status_code
    dump: str = dumps(response.json(), indent=2)
    print(f"Response: ({status_code}) {dump}")


def get_cloudfront_url():

    """
    -> Extracts the CloudFront url using boto3
    """

    client = boto3.client("cloudfront")
    distributions = client.list_distributions()
    distribution_list = distributions["DistributionList"]

    try:
        sbom_api_distribution = distribution_list["Items"][0]
        cf_domain_name = sbom_api_distribution["DomainName"]
        cf_url = f"https://{cf_domain_name}"
        print(f"CloudFront url is: {cf_url}")
        return cf_url
    except KeyError:
        ...


def login(cf_url: str) -> str:

    """
    -> Gets a JWT so we can make requests
    """

    login_url = f"{cf_url}/api/v1/login"
    user = "sbomadmin@aquia.io"
    password = "L0g1nTe5tP@55!"

    print(f"Sending To: POST:{login_url}, With: {user}, {password}")
    login_rsp = requests.post(
        login_url,
        json={
            "username": user,
            "password": password,
        },
    )

    login_rsp_json = login_rsp.json()
    print(f"Login Response: {dumps(login_rsp_json, indent=2)}")
    return login_rsp_json["token"]

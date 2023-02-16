"""
-> Module to test the SBOM Upload
"""
from importlib.resources import files
from json import loads

import botocore.exceptions
import pytest
from botocore.waiter import Waiter
from importlib_resources.abc import Traversable
from requests import Response, post

from cyclonedx.model import HarborModel
from tests import sboms, get_boto_session
from tests.e2e import (
    cleanup,
    create_codebase,
    create_team_with_projects,
    get_cloudfront_url,
    get_team_url,
    get_upload_token,
    get_upload_url,
    login,
    print_response,
)


def create_infrastructure(cf_url: str, jwt: str):

    """
    -> Create the infrastructure to support an upload
    """

    create_rsp: dict = create_team_with_projects(
        team_name="test_sbom_ingress Team",
        project_names=["test_sbom_ingress Project"],
        team_url=get_team_url(cf_url),
        jwt=jwt,
    )

    team_id: str = create_rsp.get(HarborModel.Fields.ID)

    projects: list[dict] = create_rsp.get("projects")
    project: dict = projects[0]
    project_id: str = project.get(HarborModel.Fields.ID)

    tokens: list[dict] = create_rsp.get("tokens")
    token: dict = tokens[0]
    upload_token: str = token["token"]

    codebase_id: str = create_codebase(
        team_id=team_id,
        project_id=project_id,
        cf_url=cf_url,
        jwt=jwt,
    )

    return team_id, project_id, codebase_id, upload_token


# pylint: disable = R0915
def test_sbom_ingress():

    """
    -> Main Test
    """

    retest: bool = False
    retest_team_id: str = ""
    retest_project_id: str = ""
    retest_codebase_id: str = ""

    missing_ids: bool = "" in (
        retest_team_id,
        retest_project_id,
        retest_codebase_id,
    )

    if retest:
        if missing_ids:
            raise ValueError("Need all the ids for a retest")

    sbom_folder: Traversable = files(sboms)
    sbom_obj: Traversable = sbom_folder.joinpath("panther_python.json")
    sbom: dict = loads(sbom_obj.read_text())

    cf_url: str = get_cloudfront_url()
    jwt: str = login(cf_url)

    if not retest:
        (
            team_id,
            project_id,
            codebase_id,
            upload_token,
        ) = create_infrastructure(cf_url, jwt)
    else:
        team_id: str = retest_team_id
        project_id: str = retest_project_id
        codebase_id: str = retest_codebase_id
        upload_token: str = get_upload_token(cf_url, jwt, team_id)

    sbom_upload_url: str = get_upload_url(
        cf_url=cf_url,
        team_id=team_id,
        project_id=project_id,
        codebase_id=codebase_id,
    )

    # Sending the SBOM here
    print(f"Sending To: POST:{sbom_upload_url}")
    sbom_upload_rsp: Response = post(
        sbom_upload_url,
        headers={
            "Authorization": upload_token,
        },
        json=sbom,
    )
    print_response(sbom_upload_rsp)

    # Get the bucket and the name of the S3 object containing the SBOM
    sbom_upload_rsp_json: dict = sbom_upload_rsp.json()
    s3_bucket_name: str = sbom_upload_rsp_json.get("s3BucketName")
    sbom_s3_object_key: str = sbom_upload_rsp_json.get("s3ObjectKey")

    # Check to see if the SBOM Arrived
    session = get_boto_session()
    s3_client = session.client("s3")
    waiter: Waiter = s3_client.get_waiter("object_exists")

    print(f"Looking for S3 object containing SBOM: {sbom_s3_object_key}")

    try:
        waiter.wait(
            Bucket=s3_bucket_name,
            Key=sbom_s3_object_key,
            WaiterConfig={
                "Delay": 2,
                "MaxAttempts": 5,
            },
        )
    except botocore.exceptions.ClientError as ce:
        print(f"No SBOM in S3 after 10 seconds: {ce}")
        pytest.fail()

    print(f"Found S3 object containing SBOM: {sbom_s3_object_key}")

    sbom_s3_object_key = sbom_s3_object_key[sbom_s3_object_key.rfind("/")+1:]
    dt_findings_s3_object_key: str = f"raw/findings/dt/findings-dt-{sbom_s3_object_key}"

    print(f"Looking for S3 object containing Dependency Track findings: {dt_findings_s3_object_key}")

    try:
        waiter.wait(
            Bucket=s3_bucket_name,
            Key=dt_findings_s3_object_key,
            WaiterConfig={
                "Delay": 6,
                "MaxAttempts": 10,
            },
        )
    except botocore.exceptions.ClientError as ce:
        print(f"No Dependency Track in S3 after one minute: {ce}")
        pytest.fail()

    print(
        f"Found S3 object containing Dependency Track findings: {dt_findings_s3_object_key}"
    )

    # ic_findings_s3_object_key: str = f"findings-ic-{sbom_s3_object_key}"
    #
    # try:
    #     waiter.wait(
    #         Bucket=s3_bucket_name,
    #         Key=ic_findings_s3_object_key,
    #         WaiterConfig={
    #             "Delay": 64,
    #             "MaxAttempts": 10,
    #         },
    #     )
    # except botocore.exceptions.ClientError as ce:
    #     print(f"No Ion Channel findings in S3 after 15 minutes: {ce}")
    #     pytest.fail()
    #
    # print(
    #     f"Found S3 object containing Ion Channel findings: {dt_findings_s3_object_key}"
    # )

    # s3_resource = session.resource("s3")
    #
    # # Delete the files in S3
    # s3_resource.Object(s3_bucket_name, sbom_s3_object_key).delete()
    # s3_resource.Object(s3_bucket_name, dt_findings_s3_object_key).delete()
    # s3_resource.Object(s3_bucket_name, ic_findings_s3_object_key).delete()
    #
    # # Clean up the database
    # cleanup(
    #     team_id=team_id,
    #     team_url=get_team_url(cf_url),
    #     jwt=jwt,
    # )

    print(
        f"IDS for retest: team_id({team_id}), project_id({project_id}), codebase_id({codebase_id})"
    )

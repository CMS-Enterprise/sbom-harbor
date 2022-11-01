"""
-> Test module for the Summarizer Handler
"""

import fnmatch
import glob
import os

# pylint: disable-msg=E0611
from importlib.resources import files, path, read_text
from json import dumps, loads
from typing import Callable

import pytest

import tests.data as test_data
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.enrichment.dependency_track import summarizer_handler
from cyclonedx.enrichment.dependency_track.summarizer_handler import FileTypes
from cyclonedx.model.project import Project
from tests.data import summary_samples


def set_summary_event_sbom_name(sbom_name: str, event: dict):

    """
    -> Set the Summary Event with the SBOM Name
    """

    for findings in event:
        findings["sbom_s3_key"] = sbom_name


def create_dynamodb_project_entry(test_dynamo_db_resource):
    """
    -> Creates a test dynamodb entry
    """
    team_id = "testTeam"
    project_id = "TestProject"
    fisma_id = "testFisma"

    HarborDBClient(test_dynamo_db_resource).create(
        Project(
            team_id=team_id,
            project_id=project_id,
            name=project_id,
            fisma=fisma_id,
        )
    )


def summarize_data(
    upload_to_test_bucket: Callable,
    s3_test_bucket,
    upload_to_ingress: Callable,
    test_dynamo_db_resource,
) -> []:

    """
    -> Runs the data through the summarizer and returns a list of files in the s3 bucket
    """

    create_dynamodb_project_entry(test_dynamo_db_resource)
    event: dict = loads(read_text(test_data, "summarizer_event.json"))
    sbom_file = read_text(test_data, "sbom-keycloak.json")
    results = upload_to_ingress(bytearray(sbom_file.encode()))

    with path(test_data, "findings-dt-sbom-keycloak.json") as file_path:
        upload_to_test_bucket(str(file_path), "findings-dt-sbom-keycloak.json")

    # Get the correct name of the SBOM we uploaded through ingress
    sbom_name = loads(results["body"])["s3ObjectKey"]

    # Set the correct name for the uploaded sbom into the event object
    set_summary_event_sbom_name(sbom_name, event)

    # Send event into summarizer
    summarizer_handler(event)

    files_in_bucket = []

    # Collect list of all the files currently in the bucket
    for bucket_objects in s3_test_bucket.objects.all():
        files_in_bucket.append(bucket_objects.key)

    return files_in_bucket


def test_summarizer_has_results(
    upload_to_test_bucket: Callable,
    s3_test_bucket,
    upload_to_ingress: Callable,
    test_dynamo_db_resource,
    test_harbor_teams_table,
):

    """
    -> Verifies the summarizer returns a report file
    -> that has the data requested in the event value
    """
    list_of_files = summarize_data(
        upload_to_test_bucket,
        s3_test_bucket,
        upload_to_ingress,
        test_dynamo_db_resource,
    )

    # There should only be 4 files in the bucket: orignal_sbom,
    # original_findings, normalized_sbom, normalized_findings
    assert len(list_of_files) == 4

    # Verify the expected files are in the bucket
    project = "TestProject"
    fisma = "testFisma"

    flattened_file_pattern = f"harbor-data-summary-{FileTypes.sbom}-{project}-{fisma}-*"
    assert len(fnmatch.filter(list_of_files, flattened_file_pattern)) >= 1

    flattened_file_pattern = (
        f"harbor-data-summary-{FileTypes.findings}-{project}-{fisma}-*"
    )
    assert len(fnmatch.filter(list_of_files, flattened_file_pattern)) >= 1


@pytest.mark.skip(reason="Only used for debugging summary files")
def test_get_summary_output(
    upload_to_test_bucket: Callable,
    s3_test_bucket,
    upload_to_ingress: Callable,
    test_dynamo_db_resource,
    test_harbor_teams_table,
):
    """
    -> Helpful utility to refresh and update the sample summary files.
    -> Intentionally disabled by default
    """
    directory_files = glob.glob(f"{files(summary_samples)}/*.json", recursive=True)

    for f in directory_files:
        try:
            os.remove(f)
        except OSError as e:
            print("Error: %s : %s" % (f, e.strerror))

    list_of_files = summarize_data(
        upload_to_test_bucket,
        s3_test_bucket,
        upload_to_ingress,
        test_dynamo_db_resource,
    )

    for file in list_of_files:
        s3_obj_wrapper = s3_test_bucket.Object(file)
        s3_object: dict = s3_obj_wrapper.get()
        report_file = s3_object["Body"].read().decode("utf-8")
        json_report = loads(report_file)

        filehandle = open(f"{files(summary_samples)}/{file}.json", "w")
        filehandle.writelines(dumps(json_report, indent=4))
        filehandle.close()

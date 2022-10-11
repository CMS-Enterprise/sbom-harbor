"""
-> Test module for the Summarizer Handler
"""

import importlib.resources as pr
from json import loads
from typing import Callable
from cyclonedx.enrichment.dependency_track import summarizer_handler
import tests.test_data as test_data


def set_summary_event_sbom_name(sbom_name: str, event: dict):

    """
    -> Set the Summary Event with the SBOM Name
    """

    for findings in event:
        findings["sbom_s3_key"] = sbom_name


def test_summarizer_has_results(
    upload_to_test_bucket: Callable,
    s3_test_bucket,  # botocore.client.BaseClient
    upload_to_ingress: Callable,
):

    """
    -> Verifies the summarizer returns a report file
    -> that has the data requested in the event value
    """

    event: dict = loads(pr.read_text(test_data, "summarizer_event.json"))
    sbom_file = pr.read_text(test_data, "sbom-keycloak.json")
    results = upload_to_ingress(bytearray(sbom_file.encode()))

    with pr.path(test_data, "findings-dt-sbom-keycloak.json") as file_path:
        upload_to_test_bucket(str(file_path), "findings-dt-sbom-keycloak.json")

    # Get the correct name of the SBOM we uploaded through ingress
    sbom_name = loads(results["body"])["s3ObjectKey"]

    # Set the correct name for the uploaded sbom into the event object
    set_summary_event_sbom_name(sbom_name, event)

    # Send event into summarizer
    summarizer_handler(event)

    list_of_files = []

    # Collect list of all the files currently in the bucket
    for bucket_objects in s3_test_bucket.objects.all():
        list_of_files.append(bucket_objects.key)

    # Verify the new report file is in the bucket
    # TODO update these once we know how to get them properly
    company_name = "Company_Name(missing)"
    fisma_id = "fisma_id(missing)"
    submit_date = "submit_date(missing)"
    expected_file_name = (
        f"harbor-{sbom_name}-report-{company_name}-{fisma_id}-{submit_date}"
    )
    assert expected_file_name in list_of_files

    # TODO delete the following or move it to another file, this only
    #  exists as an easy way to debug output
    # s3_obj_wrapper = s3_test_bucket.Object(expected_file_name)
    # s3_object: dict = s3_obj_wrapper.get()
    # report_file = s3_object["Body"].read().decode("utf-8")
    # json_report = json.loads(report_file)
    #
    # filehandle = open(f"/home/m32956/workspace/cyclonedx-python/tests
    #   /handlers/summarizer_handler/{expected_file_name}.json", "w")
    # filehandle.writelines(json.dumps(json_report, indent=4))
    # filehandle.close()

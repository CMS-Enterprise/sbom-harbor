import json
import os.path

from cyclonedx.enrichment.dependency_track import summarizer_handler


def setSummaryEventSbomName(sbom_name, event):
    for findings in event:
        findings["sbom_s3_key"] = sbom_name


def test_summarizer_has_results(upload_to_test_bucket, s3_test_bucket, upload_to_ingress, test_data_path):

    """Verifies the summarizer returns a report file that has the data requested in the event value"""
    event = json.load(open(os.path.join(test_data_path, 'test_data/summarizer_event.json')))
    findings_file = open(os.path.join(test_data_path, 'test_data/findings-dt-sbom-keycloak.json'))
    sbom_file = open(os.path.join(test_data_path, 'test_data/sbom-keycloak.json'))

    results = upload_to_ingress(sbom_file)
    upload_to_test_bucket(findings_file.name, "findings-dt-sbom-keycloak.json")

    # Get the correct name of the SBOM we uploaded through ingress
    sbom_name = json.loads(results["body"])["s3ObjectKey"]

    # Set the correct name for the uploaded sbom into the event object
    setSummaryEventSbomName(sbom_name, event)

    # Send event into summarizer
    summarizer_handler(event)

    list_of_files = []

    # Collect list of all the files currently in the bucket
    for bucket_objects in s3_test_bucket.objects.all():
        list_of_files.append(bucket_objects.key)

    # Verify the new report file is in the bucket
    #TODO update these once we know how to get them properly
    company_name = "Company_Name(missing)"
    fisma_id = "fisma_id(missing)"
    submit_date = "submit_date(missing)"
    expected_file_name = f"harbor-{sbom_name}-report-{company_name}-{fisma_id}-{submit_date}"
    assert expected_file_name in list_of_files

    # TODO delete the following or move it to another file, this only exists as an easy way to debug output
    # s3_obj_wrapper = s3_test_bucket.Object(expected_file_name)
    # s3_object: dict = s3_obj_wrapper.get()
    # report_file = s3_object["Body"].read().decode("utf-8")
    # json_report = json.loads(report_file)
    #
    # filehandle = open(f"/home/m32956/workspace/cyclonedx-python/tests/handlers/summarizer_handler/{expected_file_name}.json", "w")
    # filehandle.writelines(json.dumps(json_report, indent=4))
    # filehandle.close()
    
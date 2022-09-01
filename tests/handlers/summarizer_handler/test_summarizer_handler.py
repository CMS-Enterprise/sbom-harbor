import json

from cyclonedx.enrichment.dependency_track import summarizer_handler

event_file = json.load(open('../../test_data/summarizer_event.json'))
findingsFile = open('../../test_data/findings-dt-sbom-keycloak.json')


def test_summarizer_has_results(test_s3_bucket, test_bucket_name):
    """Verifies the summarizer returns a report file that has the data requested in the event value"""
    test_s3_bucket.upload_file(findingsFile.name, test_bucket_name, "findings-dt-sbom-keycloak.json")

    summarizer_handler(event_file)

    response = test_s3_bucket.list_objects(Bucket=test_bucket_name)
    data = [object["Key"] for object in response["Contents"]]

    assert 'report-sbom-keycloak.json' in data

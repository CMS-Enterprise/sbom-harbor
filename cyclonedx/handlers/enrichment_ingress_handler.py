"""
-> Module for the Enrichment Ingress Handler
"""
import logging
from json import dumps
from logging import config

import boto3
from jsonschema.exceptions import ValidationError

from cyclonedx.clients.dependency_track.dependency_track import __get_records_from_event
from cyclonedx.constants import (
    EVENT_BUS_DETAIL_TYPE,
    EVENT_BUS_NAME,
    EVENT_BUS_SOURCE,
    PYTHON_LOGGING_CONFIG,
    SBOM_BUCKET_NAME_KEY,
    SBOM_S3_KEY,
)

config.fileConfig(PYTHON_LOGGING_CONFIG)
logger = logging.getLogger(__name__)


def enrichment_ingress_handler(event: dict = None, context: dict = None):

    """
    Handler that listens for S3 put events and routes the SBOM
    to the enrichment code
    """

    if not event:
        raise ValidationError("event should never be none")

    records: list = __get_records_from_event(event)

    logger.info("Records records= %s", records)

    for record in records:

        s3_obj = record["s3"]
        bucket_obj = s3_obj["bucket"]
        bucket_name = bucket_obj["name"]
        sbom_obj = s3_obj["object"]
        key: str = sbom_obj["key"]

        eb_client = boto3.client("events")

        # s3_object = s3.Object(bucket_name, key).get()

        # try:
        #     enrichment_id = s3_object["Metadata"][ENRICHMENT_ID]
        # except KeyError as key_err:
        #     logger.info("s3Object object= %s", s3_object)} />")
        #     enrichment_id = f"ERROR: {key_err}"

        response = eb_client.put_events(
            Entries=[
                {
                    "Source": EVENT_BUS_SOURCE,
                    "DetailType": EVENT_BUS_DETAIL_TYPE,
                    "Detail": dumps(
                        {
                            SBOM_BUCKET_NAME_KEY: bucket_name,
                            SBOM_S3_KEY: key,
                            "results": {},
                            "output": {},
                        },
                    ),
                    "EventBusName": EVENT_BUS_NAME,
                },
            ],
        )

        logger.info("PutEventsResponse response= %s", response)

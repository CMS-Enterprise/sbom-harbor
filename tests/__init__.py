
""" Module to set up the necessary resources to run tests """

import boto3
import docker
from docker.errors import NotFound
from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_NAME,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)

DYNAMODB_HOST = "localhost"
DYNAMODB_PORT = 6699
DYNAMODB_URL = f"http://{DYNAMODB_HOST}:{DYNAMODB_PORT}"
DYNAMODB_DOCKER_CONTAINER_NAME = "dynamodb_test_container"

dynamodb_test_resources = {
    "dynamodb": {
        "resource":  boto3.resource(
            'dynamodb',
            endpoint_url=DYNAMODB_URL
        ),
        "client": boto3.client(
            "dynamodb",
            endpoint_url=DYNAMODB_URL
        ),
    },
    "docker": {
        "image": "amazon/dynamodb-local",
        "client": docker.from_env(),
    }
}

dynamodb_resource = dynamodb_test_resources["dynamodb"]["resource"]
dynamodb_client = dynamodb_test_resources["dynamodb"]["client"]
dynamodb_docker_image = dynamodb_test_resources["docker"]["image"]
docker_client = dynamodb_test_resources["docker"]["client"]
harbor_teams_table = dynamodb_test_resources["dynamodb"]["table"] \
    = dynamodb_resource.Table(HARBOR_TEAMS_TABLE_NAME)

def get_ek(et: str, model_id: str):
    return "{}#{}".format(et, model_id)

# Use the AWS CLI to test that the container is working
# aws dynamodb list-tables --endpoint-url http://DYNAMODB_HOST:DYNAMODB_PORT
def setup_database_tests():

    """
        Sets up the DynamoDB Docker Image and Database
        Check out the data with the AWS CLI:

        Show Docker Image:
        docker ps

        Show Databases:
        $ aws dynamodb list-tables --endpoint-url http://localhost:6699

        Get Items:
        $ aws dynamodb scan --table-name HarborTeamsTable \
            --endpoint-url http://localhost:6699

        A couple of helpful commands for example.
    """

    try:
        # See if the container is still running somehow
        # If so, then make sure it's dead
        docker_client.containers.get(DYNAMODB_DOCKER_CONTAINER_NAME)
        teardown_database_tests()
    except NotFound:
        ...

    # Pull Docker image and install it locally
    docker_client.images.pull(dynamodb_docker_image)

    # This command has to be overwritten because if you don't include
    # the -sharedDB flag is not present then dynamodb-local will use
    # access keys+region as namespaces to separate tables
    # as if they were under different aws accounts and you can't
    # see them with the aws cli.
    cmd = "-jar DynamoDBLocal.jar -sharedDb"

    # Start the DynamoDB Docker image running at DYNAMODB_PORT
    docker_client.containers.run(
        dynamodb_docker_image,
        name=DYNAMODB_DOCKER_CONTAINER_NAME,
        command=cmd,
        ports={
            8000: DYNAMODB_PORT
        },
        detach=True
    )

    try:
        dynamodb_client.create_table(
            TableName=HARBOR_TEAMS_TABLE_NAME,
            AttributeDefinitions=[
                {
                    "AttributeName": HARBOR_TEAMS_TABLE_PARTITION_KEY,
                    "AttributeType": "S"
                },
                {
                    "AttributeName": HARBOR_TEAMS_TABLE_SORT_KEY,
                    "AttributeType": "S"
                }
            ],
            KeySchema=[
                {
                    "AttributeName": HARBOR_TEAMS_TABLE_PARTITION_KEY,
                    "KeyType": "HASH"
                },
                {
                    "AttributeName": HARBOR_TEAMS_TABLE_SORT_KEY,
                    "KeyType": "RANGE"
                }
            ],
            ProvisionedThroughput={
                "ReadCapacityUnits": 1,
                "WriteCapacityUnits": 1
            }
        )
    except Exception:
        teardown_database_tests()
        setup_database_tests()

def teardown_database_tests():
    container = docker_client.containers.get(
        DYNAMODB_DOCKER_CONTAINER_NAME
    )
    container.stop()
    container.remove()

def database_smoke_test():

    """
        This is a smoke test to verify we are connected
        to the database before we start testing
    """

    team_id = "abc123Test"

    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "team",
            "name": "TEST TEAM",
        }
    )

    team = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "team",
        }
    )

    item = team["Item"]

    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "team" == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert "TEST TEAM" == item["name"]
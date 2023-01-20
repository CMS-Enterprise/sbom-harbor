"""
-> This is intended to add the test team data to dynamodb.
-> We need this test data we need before the e2e tests can run
"""

import logging
import uuid
from datetime import datetime, timedelta

import boto3

from cyclonedx.clients.db.dynamodb import HarborDBClient
from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.model import EntityType
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.member import Member
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token
from tests import get_boto_session, get_harbor_table_name, get_harbor_client

harbor_table: str = get_harbor_table_name()

session = get_boto_session()

boto3.set_stream_logger(name="botocore", level=logging.DEBUG)

ddb_resource = session.resource("dynamodb")
ddb_client = session.client("dynamodb")
teams_table = ddb_resource.Table(harbor_table)

project_id = "test-project-id"
project_name = "Test Project Name"

codebase_id = "back-end"
codebase_name = "Back End"

member_id = "keyser-soze"
email = "keyser.soze@aquia.io"

token_id = "working-token"

language = "TYPESCRIPT"
build_tool = "YARN"

now = datetime.now()
created = datetime.now()
expires = now + timedelta(days=100)
token_val = str(uuid.uuid4())

def test_write_to_db():

    """
    -> This is a test to see if we can write to the database.
    -> If this test fails, then we will be unable to add the test team.
    """

    print(f"Attempting to write to {harbor_table}")
    teams_table.put_item(
        TableName=harbor_table,
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: "dawn-patrol",
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
            "name": "Test Team",
        },
    )


def test_add_test_team():

    """
    -> This function adds a test team to
    -> DyanmoDB so the e2e tests can run
    """

    for team_id in ["dawn-patrol", "dusk-patrol"]:

        try:
            get_harbor_client().delete(
                Team(team_id=team_id),
                recurse=True,
            )
        except DatabaseError:
            ...

        get_harbor_client().create(
            Team(
                team_id=team_id,
                name=team_id,
                projects=[
                    Project(
                        team_id=team_id,
                        project_id=project_id,
                        name=project_name,
                        codebases=[
                            CodeBase(
                                team_id=team_id,
                                codebase_id=codebase_id,
                                project_id=project_id,
                                name=codebase_name,
                                language=language,
                                build_tool=build_tool,
                            )
                        ],
                    )
                ],
                members=[
                    Member(
                        team_id=team_id,
                        member_id=member_id,
                        email=email,
                        is_team_lead=True,
                    ),
                ],
                tokens=[
                    Token(
                        team_id=team_id,
                        token_id=token_id,
                        name=f"{team_id}-{token_id}",
                        enabled=True,
                        created=created.isoformat(),
                        expires=expires.isoformat(),
                        token=token_val,
                    )
                ],
            ),
            recurse=True,
        )

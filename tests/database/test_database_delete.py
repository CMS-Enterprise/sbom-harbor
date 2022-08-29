
""" Database and Model tests for Deleting objects in the HarborTeamsTable """

import uuid

import pytest

from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.model import EntityType
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.member import Member
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token
from decimal import Decimal

from tests import (
    dynamodb_test_resources,
    setup_database_tests,
    teardown_database_tests,
    database_smoke_test,
)

dynamodb_resources = dynamodb_test_resources["dynamodb"]
dynamodb_resource = dynamodb_resources["resource"]
dynamodb_client = dynamodb_resources["client"]
harbor_teams_table = dynamodb_resources["table"]
setup_function = setup_database_tests
teardown_function = teardown_database_tests

# This is a smoke test designed to be sure we have
# database connectivity before launching into the
# actual tests
test_database = database_smoke_test

def test_delete_team_only():

    team_id = str(uuid.uuid4())
    entity_type = EntityType.TEAM.value
    team_name = "Dawn Patrol"

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_type,
            "name": team_name,
        }
    )

    # Verify that the item is there
    team = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    item = team["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_type == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert team_name == item["name"]

    # Delete the item using the API
    HarborDBClient(dynamodb_resource).delete(
        Team(
            team_id=team_id,
            name=team_name,
        )
    )

    # Verify the item is NOT there
    team = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {team['Item']}")


def test_delete_project_only():

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())

    pet = EntityType.PROJECT.value
    range_key = "{}#{}".format(pet, project_id)

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
            "name": project_id,
        }
    )

    project = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    item = project["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert range_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert project_id == item["name"]

    # Delete the item using the API
    HarborDBClient(dynamodb_resource).delete(
        Project(
            team_id=team_id,
            project_id=project_id,
            name=project_id,
        )
    )

    # Verify the item is NOT there
    project = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {project['Item']}")


def test_delete_codebase_only():

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    codebase_id = str(uuid.uuid4())

    cet = EntityType.CODEBASE.value
    range_key = "{}#{}".format(cet, codebase_id)

    language = "JAVASCRIPT"
    build_tool = "YARN"

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
            "name": codebase_id,
            "project_id": project_id,
            "language": language,
            "build_tool": build_tool,
        }
    )

    codebase = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    item = codebase["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert range_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert codebase_id == item["name"]
    assert project_id == item["project_id"]
    assert language == item["language"]
    assert build_tool == item["build_tool"]

    HarborDBClient(dynamodb_resource).delete(
        CodeBase(
            team_id=team_id,
            project_id=project_id,
            codebase_id=codebase_id,
            name=codebase_id,
            language=language,
            build_tool=build_tool,
        )
    )

    codebase = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {codebase['Item']}")


def test_delete_member_only():

    team_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())
    email = "test.user@aquia.io"

    met = EntityType.MEMBER.value
    range_key = "{}#{}".format(met, member_id)

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
            "email": email,
            "isTeamLead": True,
        }
    )

    member = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    item = member["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert range_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert email == item["email"]
    assert item["isTeamLead"]

    HarborDBClient(dynamodb_resource).delete(
        Member(
            team_id=team_id,
            member_id=member_id,
            email=member_id,
            is_team_lead=True,
        )
    )

    member = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {member['Item']}")


def test_delete_token_only():

    team_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())
    token_val = str(uuid.uuid4())
    created = Decimal(507482179.234)
    expires = Decimal(507492179.234)

    tet = EntityType.TOKEN.value
    range_key = "{}#{}".format(tet, token_id)

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
            Token.Fields.CREATED: created,
            Token.Fields.EXPIRES: expires,
            Token.Fields.ENABLED: True,
            Token.Fields.TOKEN: token_val,
        }
    )

    token = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    item = token["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert range_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert item[Token.Fields.ENABLED]
    assert created == item[Token.Fields.CREATED]
    assert expires == item[Token.Fields.EXPIRES]
    assert token_val == item[Token.Fields.TOKEN]

    HarborDBClient(dynamodb_resource).delete(
        Token(
            team_id=team_id,
            token_id=token_id,
            enabled=True,
            created=created,
            expires=expires,
            token=token_val,
        )
    )

    token = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {token['Item']}")


def test_delete_team_with_a_child_of_each_type():

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    codebase_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())

    team_name = "Dawn Patrol"
    project_name = "SBOM Harbor"
    codebase_name = "Backend"

    email = "fancypants@hotmail.com"
    language = "JAVASCRIPT"
    build_tool = "YARN"

    created = Decimal(507482179.234)
    expires = Decimal(507492179.234)
    token_val = str(uuid.uuid4())

    # Put the Team
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
            Team.Fields.NAME: team_name,
        }
    )

    # Put the Project
    pet = EntityType.PROJECT.value
    project_range_key = "{}#{}".format(pet, project_id)
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: project_range_key,
            Project.Fields.NAME: project_name,
        }
    )

    # Put the Codebase
    cet = EntityType.CODEBASE.value
    codebase_range_key = "{}#{}".format(cet, codebase_id)
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: codebase_range_key,
            CodeBase.Fields.NAME: codebase_name,
            CodeBase.Fields.LANGUAGE: language,
            CodeBase.Fields.BUILD_TOOL: build_tool,
        }
    )

    # Put the Member
    met = EntityType.MEMBER.value
    member_range_key = "{}#{}".format(met, member_id)
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: member_range_key,
            Member.Fields.EMAIL: email,
        }
    )

    # Put the Token
    tet = EntityType.TOKEN.value
    token_range_key = "{}#{}".format(tet, token_id)
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: token_range_key,
            Token.Fields.ENABLED: True,
            Token.Fields.CREATED: created,
            Token.Fields.EXPIRES: expires,
            Token.Fields.TOKEN: token_val,
        }
    )

    team = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    project = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: project_range_key,
        }
    )

    codebase = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: codebase_range_key,
        }
    )

    member = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: member_range_key,
        }
    )

    token = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: token_range_key,
        }
    )

    team_item = team["Item"]
    assert team_id == team_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert EntityType.TEAM.value == team_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert team_name == team_item["name"]

    project_item = project["Item"]
    assert team_id == project_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert project_range_key == project_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert project_name == project_item["name"]

    codebase_item = codebase["Item"]
    assert team_id == codebase_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert codebase_range_key == codebase_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert codebase_name == codebase_item["name"]

    member_item = member["Item"]
    assert team_id == member_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert member_range_key == member_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert email == member_item["email"]

    token_item = token["Item"]
    assert team_id == token_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert token_range_key == token_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert token_item["enabled"]
    assert created == token_item["created"]
    assert expires == token_item["expires"]
    assert token_val == token_item["token"]

    HarborDBClient(dynamodb_resource).delete(
        Team(
            team_id=team_id,
            name=team_id,
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name=project_id,
                    codebases=[
                        CodeBase(
                            team_id=team_id,
                            codebase_id=codebase_id,
                            project_id=project_id,
                            name=codebase_id,
                            language=language,
                            build_tool=build_tool,
                        )
                    ]
                )
            ],
            members=[
                Member(
                    team_id=team_id,
                    member_id=member_id,
                    email=member_id,
                    is_team_lead=True,
                ),
            ],
            tokens=[
                Token(
                    team_id=team_id,
                    token_id=token_id,
                    enabled=True,
                    created=created,
                    expires=expires,
                    token=token_val,
                )
            ],
        ),
        recurse=True,
    )

    team = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {team['Item']}")

    project = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: project_range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {project['Item']}")

    codebase = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: codebase_range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {codebase['Item']}")

    member = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: member_range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {member['Item']}")


    token = harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: token_range_key,
        }
    )

    with pytest.raises(KeyError):
        print(f"Exception here: {token['Item']}")
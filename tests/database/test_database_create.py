
""" Database and Model tests for Creating objects in the HarborTeamsTable """

import uuid
from decimal import Decimal

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


def test_create_team_only(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    entity_type = "team"
    team_name = "Dawn Patrol"

    HarborDBClient(test_dynamo_db_resource).create(
        Team(
            team_id=team_id,
            name=team_name,
        )
    )

    team = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    item = team["Item"]

    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_type == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert team_name == item["name"]


def test_create_project_only(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    fisma_id = str(uuid.uuid4())

    HarborDBClient(test_dynamo_db_resource).create(
        Project(
            team_id=team_id,
            project_id=project_id,
            name=project_id,
            fisma=fisma_id,
        )
    )

    pet = EntityType.PROJECT.value
    project = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(pet, project_id),
        }
    )

    item = project["Item"]

    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(pet, project_id) == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert project_id == item["name"]


def test_create_codebase_only(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    codebase_id = str(uuid.uuid4())

    language = "JAVASCRIPT"
    build_tool = "YARN"

    HarborDBClient(test_dynamo_db_resource).create(
        CodeBase(
            team_id=team_id,
            project_id=project_id,
            codebase_id=codebase_id,
            name=codebase_id,
            language=language,
            build_tool=build_tool,
        )
    )

    cet = EntityType.CODEBASE.value
    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(cet, codebase_id),
        }
    )

    item = codebase["Item"]

    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(cet, codebase_id) == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert codebase_id == item["name"]


def test_create_member_only(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())

    HarborDBClient(test_dynamo_db_resource).create(
        Member(
            team_id=team_id,
            member_id=member_id,
            email=member_id,
            is_team_lead=True,
        )
    )

    met = EntityType.MEMBER.value
    member = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(met, member_id),
        }
    )

    item = member["Item"]

    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(met, member_id) == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert member_id == item["email"]
    assert item["isTeamLead"]


def test_create_token_only(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())
    token_val = str(uuid.uuid4())

    created = Decimal(507482179.234)
    expires = Decimal(507492179.234)

    HarborDBClient(test_dynamo_db_resource).create(
        Token(
            team_id=team_id,
            token_id=token_id,
            enabled=True,
            created=created,
            expires=expires,
            token=token_val,
        )
    )

    tet = EntityType.TOKEN.value
    token = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(tet, token_id),
        }
    )

    item = token["Item"]

    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(tet, token_id) == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert item["enabled"]
    assert created == item["created"]
    assert expires == item["expires"]
    assert token_val == item["token"]

def test_create_team_with_project(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    fisma_id = str(uuid.uuid4())

    HarborDBClient(test_dynamo_db_resource).create(
        Team(
            team_id=team_id,
            name=team_id,

            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name=project_id,
                    fisma=fisma_id,
                )
            ]
        ),
        recurse=True,
    )

    team = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    pet = EntityType.PROJECT.value
    project = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(pet, project_id),
        }
    )

    team_item = team["Item"]
    project_item = project["Item"]

    assert team_id == team_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert EntityType.TEAM.value == team_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert team_id == team_item["name"]

    assert team_id == project_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(pet, project_id) == project_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert project_id == project_item["name"]


def test_create_team_with_a_child_of_each_type(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    codebase_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())
    fisma_id = str(uuid.uuid4())

    language = "JAVASCRIPT"
    build_tool = "YARN"

    created = Decimal(507482179.234)
    expires = Decimal(507492179.234)
    token_val = str(uuid.uuid4())

    HarborDBClient(test_dynamo_db_resource).create(
        Team(
            team_id=team_id,
            name=team_id,
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name=project_id,
                    fisma=fisma_id,
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

    team = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
        }
    )

    pet = EntityType.PROJECT.value
    project = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(pet, project_id),
        }
    )

    cet = EntityType.CODEBASE.value
    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(cet, codebase_id),
        }
    )

    met = EntityType.MEMBER.value
    member = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(met, member_id),
        }
    )

    tet = EntityType.TOKEN.value
    token = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: "{}#{}".format(tet, token_id),
        }
    )

    team_item = team["Item"]
    assert team_id == team_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert EntityType.TEAM.value == team_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert team_id == team_item[Team.Fields.NAME]

    project_item = project["Item"]
    assert team_id == project_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(pet, project_id) == project_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert project_id == project_item[Project.Fields.NAME]

    codebase_item = codebase["Item"]
    assert team_id == codebase_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(cet, codebase_id) == codebase_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert codebase_id == codebase_item[CodeBase.Fields.NAME]
    assert language == codebase_item[CodeBase.Fields.LANGUAGE]
    assert build_tool == codebase_item[CodeBase.Fields.BUILD_TOOL]

    member_item = member["Item"]
    assert team_id == member_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(met, member_id) == member_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert member_id == member_item[Member.Fields.EMAIL]
    assert member_item[Member.Fields.IS_TEAM_LEAD]

    token_item = token["Item"]
    assert team_id == token_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(tet, token_id) == token_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert token_item[Token.Fields.ENABLED]
    assert created == token_item[Token.Fields.CREATED]
    assert expires == token_item[Token.Fields.EXPIRES]
    assert token_val == token_item[Token.Fields.TOKEN]
    
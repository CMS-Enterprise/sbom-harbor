
""" Database and Model tests for Creating objects in the HarborTeamsTable """

import uuid
from decimal import Decimal

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


def get_ek(et: str, model_id: str):
    return "{}#{}".format(et, model_id)


def test_update_team_only(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    entity_type = EntityType.TEAM.value
    team_name = "Dawn Patrol"
    new_team_name = "Yawn Petrol"

    try:

        HarborDBClient(test_dynamo_db_resource).update(
            Team(
                team_id=team_id,
                name=team_name,
            )
        )

        pytest.fail("HarborDBClient should have thrown exception")
    except Exception:
        ...

    # Put the Item
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_type,
            Team.Fields.NAME: team_name,
        }
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
    assert team_name == item[Team.Fields.NAME]

    HarborDBClient(test_dynamo_db_resource).update(
        Team(
            team_id=team_id,
            name=new_team_name,
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
    assert new_team_name == item[Team.Fields.NAME]


def test_update_project_only(test_dynamo_db_resource, test_harbor_teams_table):

    entity_type = EntityType.PROJECT.value

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    project_name = "RATM"
    new_project_name = "MTRA"

    try:

        HarborDBClient(test_dynamo_db_resource).update(
            Project(
                team_id=team_id,
                project_id=project_id,
                name=new_project_name,
            )
        )

        pytest.fail("HarborDBClient should have thrown exception")
    except Exception:
        ...

    entity_key = get_ek(entity_type, project_id)

    # Put the Item
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
            Project.Fields.NAME: project_name,
        }
    )

    project = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = project["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert project_name == item[Project.Fields.NAME]

    HarborDBClient(test_dynamo_db_resource).update(
        Project(
            team_id=team_id,
            project_id=project_id,
            name=new_project_name,
        )
    )

    project = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = project["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert new_project_name == item[Project.Fields.NAME]


def test_update_codebase_only(test_dynamo_db_resource, test_harbor_teams_table):

    entity_type = EntityType.CODEBASE.value

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())

    codebase_id = str(uuid.uuid4())

    codebase_name = "RATM"
    new_codebase_name = "MTRA"

    language = "JAVA"
    build_tool = "MAVEN"
    new_build_tool = "GRADLE"

    try:

        HarborDBClient(test_dynamo_db_resource).update(
            CodeBase(
                team_id=team_id,
                codebase_id=codebase_id,
                name=new_codebase_name,
                project_id=project_id,
                language=language,
                build_tool=new_build_tool,
            )
        )

        pytest.fail("HarborDBClient should have thrown exception")
    except Exception:
        ...

    entity_key = get_ek(entity_type, codebase_id)

    # Put the Item
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
            CodeBase.Fields.NAME: codebase_name,
            CodeBase.Fields.LANGUAGE: language,
            CodeBase.Fields.BUILD_TOOL: build_tool,
            CodeBase.Fields.PARENT_ID: project_id,
        }
    )

    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = codebase["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert codebase_name == item[CodeBase.Fields.NAME]
    assert project_id == item[CodeBase.Fields.PARENT_ID]
    assert build_tool == item[CodeBase.Fields.BUILD_TOOL]
    assert language == item[CodeBase.Fields.LANGUAGE]

    HarborDBClient(test_dynamo_db_resource).update(
        CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
            project_id=project_id,
            name=new_codebase_name,
            language=language,
            build_tool=new_build_tool,
        )
    )

    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = codebase["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert new_codebase_name == item[CodeBase.Fields.NAME]
    assert project_id == item[CodeBase.Fields.PARENT_ID]
    assert new_build_tool == item[CodeBase.Fields.BUILD_TOOL]
    assert language == item[CodeBase.Fields.LANGUAGE]


def test_update_member_only(test_dynamo_db_resource, test_harbor_teams_table):

    entity_type = EntityType.MEMBER.value

    team_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())

    email = "test.tester@testy.com"
    new_email = "test.tester@dunderclumpin.com"

    is_team_lead = True
    not_team_lead_anymore = False

    try:

        HarborDBClient(test_dynamo_db_resource).update(
            Member(
                team_id=team_id,
                member_id=member_id,
                email=email,
                is_team_lead=is_team_lead,
            )
        )

        pytest.fail("HarborDBClient should have thrown exception")
    except Exception:
        ...

    entity_key = get_ek(entity_type, member_id)

    # Put the Item
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
            Member.Fields.EMAIL: email,
            Member.Fields.IS_TEAM_LEAD: is_team_lead,
        }
    )

    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = codebase["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert email == item[Member.Fields.EMAIL]
    assert is_team_lead == item[Member.Fields.IS_TEAM_LEAD]

    HarborDBClient(test_dynamo_db_resource).update(
        Member(
            team_id=team_id,
            member_id=member_id,
            email=new_email,
            is_team_lead=not_team_lead_anymore,
        )
    )

    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = codebase["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert new_email == item[Member.Fields.EMAIL]
    assert not_team_lead_anymore == item[Member.Fields.IS_TEAM_LEAD]


def test_update_token_only(test_dynamo_db_resource, test_harbor_teams_table):

    entity_type = EntityType.TOKEN.value

    team_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())
    token_val = str(uuid.uuid4())

    created = Decimal(507482179.234)
    new_created = Decimal(507483179.234)

    expires = Decimal(507492179.234)
    new_expires = Decimal(507494179.234)

    try:

        HarborDBClient(test_dynamo_db_resource).update(
            Token(
                team_id=team_id,
                token_id=token_id,
                enabled=True,
                created=created,
                expires=expires,
                token=token_val,
            )
        )

        pytest.fail("HarborDBClient should have thrown exception")
    except Exception:
        ...

    entity_key = get_ek(entity_type, token_id)

    # Put the Item
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
            Token.Fields.CREATED: created,
            Token.Fields.EXPIRES: expires,
            Token.Fields.ENABLED: True,
            Token.Fields.TOKEN: token_val,
        }
    )

    token = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = token["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert item[Token.Fields.ENABLED]
    assert created == item[Token.Fields.CREATED]
    assert expires == item[Token.Fields.EXPIRES]
    assert token_val == item[Token.Fields.TOKEN]

    HarborDBClient(test_dynamo_db_resource).update(
        Token(
            team_id=team_id,
            token_id=token_id,
            enabled=True,
            created=new_created,
            expires=new_expires,
            token=token_val,
        )
    )

    codebase = test_harbor_teams_table.get_item(
        Key={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: entity_key,
        }
    )

    item = codebase["Item"]
    assert team_id == item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert entity_key == item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert item[Token.Fields.ENABLED]
    assert new_created == item[Token.Fields.CREATED]
    assert new_expires == item[Token.Fields.EXPIRES]
    assert token_val == item[Token.Fields.TOKEN]


def test_update_team_with_a_child_of_each_type(test_dynamo_db_resource, test_harbor_teams_table):

    team_id = str(uuid.uuid4())
    project_id = str(uuid.uuid4())
    codebase_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())

    team_name = "Dawn Patrol"
    new_team_name = "Dawn Patrol"

    project_name = "SBOM Harbor"
    new_project_name = "SBOM Harbor"

    codebase_name = "Backend"
    new_codebase_name = "Backend"

    email = "fancypants@hotmail.com"
    new_email = "testpants@hotmail.com"

    language = "JAVASCRIPT"
    new_language = "JAVA"

    build_tool = "YARN"

    created = Decimal(507482179.234)
    expires = Decimal(507492179.234)
    token_val = str(uuid.uuid4())
    new_token_val = str(uuid.uuid4())

    # Put the Team
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
            Team.Fields.NAME: team_name,
        }
    )

    # Put the Project
    pet = EntityType.PROJECT.value
    project_range_key = "{}#{}".format(pet, project_id)
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: project_range_key,
            Project.Fields.NAME: project_name,
        }
    )

    # Put the Codebase
    cet = EntityType.CODEBASE.value
    codebase_range_key = "{}#{}".format(cet, codebase_id)
    test_harbor_teams_table.put_item(
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
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: member_range_key,
            Member.Fields.EMAIL: email,
        }
    )

    # Put the Token
    tet = EntityType.TOKEN.value
    token_range_key = "{}#{}".format(tet, token_id)
    test_harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: token_range_key,
            Token.Fields.ENABLED: True,
            Token.Fields.CREATED: created,
            Token.Fields.EXPIRES: expires,
            Token.Fields.TOKEN: token_val,
        }
    )

    HarborDBClient(test_dynamo_db_resource).update(
        Team(
            team_id=team_id,
            name=new_team_name,
            projects=[
                Project(
                    team_id=team_id,
                    project_id=project_id,
                    name=new_project_name,
                    codebases=[
                        CodeBase(
                            team_id=team_id,
                            codebase_id=codebase_id,
                            project_id=project_id,
                            name=new_codebase_name,
                            language=new_language,
                            build_tool=build_tool,
                        )
                    ]
                )
            ],
            members=[
                Member(
                    team_id=team_id,
                    member_id=member_id,
                    email=new_email,
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
                    token=new_token_val,
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
    assert new_team_name == team_item[Team.Fields.NAME]

    project_item = project["Item"]
    assert team_id == project_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(pet, project_id) == project_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert new_project_name == project_item[Project.Fields.NAME]

    codebase_item = codebase["Item"]
    assert team_id == codebase_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(cet, codebase_id) == codebase_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert new_codebase_name == codebase_item[CodeBase.Fields.NAME]
    assert new_language == codebase_item[CodeBase.Fields.LANGUAGE]
    assert build_tool == codebase_item[CodeBase.Fields.BUILD_TOOL]

    member_item = member["Item"]
    assert team_id == member_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(met, member_id) == member_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert new_email == member_item[Member.Fields.EMAIL]
    assert member_item[Member.Fields.IS_TEAM_LEAD]

    token_item = token["Item"]
    assert team_id == token_item[HARBOR_TEAMS_TABLE_PARTITION_KEY]
    assert "{}#{}".format(tet, token_id) == token_item[HARBOR_TEAMS_TABLE_SORT_KEY]
    assert token_item[Token.Fields.ENABLED]
    assert created == token_item[Token.Fields.CREATED]
    assert expires == token_item[Token.Fields.EXPIRES]
    assert new_token_val == token_item[Token.Fields.TOKEN]

""" Database and Model tests for Getting objects in the HarborTeamsTable """

import uuid

from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_NAME, HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)
from cyclonedx.db.harbor_db_client import HarborDBClient
from cyclonedx.model import EntityType
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.member import Member
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from decimal import Decimal

from cyclonedx.model.token import Token
from tests import (
    dynamodb_test_resources,
    setup_database_tests,
    teardown_database_tests,
    database_smoke_test,
)

dynamodb_resources = dynamodb_test_resources["dynamodb"]
dynamodb_resource = dynamodb_resources["resource"]
dynamodb_client = dynamodb_resources["client"]
harbor_teams_table = dynamodb_resource.Table(HARBOR_TEAMS_TABLE_NAME)
setup_function = setup_database_tests
teardown_function = teardown_database_tests

# This is a smoke test designed to be sure we have
# database connectivity before launching into the
# actual tests
test_database = database_smoke_test

def test_get_team_only():

    team_id = 'dawn-patrol'

    # Put the Team
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
            Team.Fields.NAME: team_id,
        }
    )

    # Get the Team using the API
    team: Team = HarborDBClient(dynamodb_resource).get(
        Team(
            team_id=team_id
        )
    )

    assert team.team_id == team_id
    assert team.entity_key == EntityType.TEAM.value
    assert team.name == team_id

def test_get_project_only():

    team_id: str = 'dawn-patrol'
    project_id: str = str(uuid.uuid4())
    project_name: str = 'my-neat-project'

    pet = EntityType.PROJECT.value
    sort_key = "{}#{}".format(pet, project_id)

    # Put the Team
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: sort_key,
            Project.Fields.NAME: project_name,
        }
    )

    # Get the Team using the API
    filled_project: Project = HarborDBClient(dynamodb_resource).get(
        Project(
            team_id=team_id,
            project_id=project_id,
        )
    )

    assert filled_project.team_id == team_id
    assert filled_project.entity_key == sort_key
    assert filled_project._name == project_name


def test_get_codebase_only():

    team_id: str = 'dawn-patrol'
    codebase_id: str = str(uuid.uuid4())
    codebase_name: str = 'my-neat-project'
    project_id: str = str(uuid.uuid4())
    language = "PYTHON"
    build_tool  = "POETRY"

    cet = EntityType.CODEBASE.value
    sort_key = "{}#{}".format(cet, codebase_id)

    # Put the Codebase
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: sort_key,
            CodeBase.Fields.NAME: codebase_name,
            CodeBase.Fields.PARENT_ID: project_id,
            CodeBase.Fields.LANGUAGE: language,
            CodeBase.Fields.BUILD_TOOL: build_tool,
        }
    )

    # Get the Team using the API
    filled_codebase: CodeBase = HarborDBClient(dynamodb_resource).get(
        CodeBase(
            team_id=team_id,
            codebase_id=codebase_id,
        )
    )

    assert filled_codebase.team_id == team_id
    assert filled_codebase.entity_key == sort_key
    assert filled_codebase.name == codebase_name
    assert filled_codebase.parent_id == project_id
    assert filled_codebase.language == language
    assert filled_codebase.build_tool == build_tool

def test_get_member_only():

    team_id = str(uuid.uuid4())
    member_id = str(uuid.uuid4())
    email = "test.user@aquia.io"

    met = EntityType.MEMBER.value
    sort_key = "{}#{}".format(met, member_id)

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: sort_key,
            Member.Fields.EMAIL: email,
            Member.Fields.IS_TEAM_LEAD: True,
        }
    )

    member: Member = HarborDBClient(dynamodb_resource).get(
        Member(
            team_id=team_id,
            member_id=member_id,
        )
    )

    assert member.team_id == team_id
    assert member.entity_key == sort_key
    assert member.email == email
    assert member.is_team_lead == True


def test_get_token_only():

    team_id = str(uuid.uuid4())
    token_id = str(uuid.uuid4())
    token_val = str(uuid.uuid4())
    created = Decimal(507482179.234)
    expires = Decimal(507492179.234)

    tet = EntityType.TOKEN.value
    sort_key = "{}#{}".format(tet, token_id)

    # Put the Item
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: sort_key,
            Token.Fields.CREATED: created,
            Token.Fields.EXPIRES: expires,
            Token.Fields.ENABLED: True,
            Token.Fields.TOKEN: token_val,
        }
    )

    token: Token = HarborDBClient(dynamodb_resource).get(
        Token(
            team_id=team_id,
            token_id=token_id,
        )
    )

    assert token.team_id == team_id
    assert token.entity_key == sort_key
    assert token.enabled == True
    assert token.created == created
    assert token.expires == expires
    assert token.token == token_val

def test_get_team_recursively():

    team_id = 'dawn-patrol'

    # Put the Team
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: EntityType.TEAM.value,
            Team.Fields.NAME: team_id,
        }
    )

    project0_id = str(uuid.uuid4())
    project0_entity_key = "{}#{}".format(EntityType.PROJECT.value, project0_id)
    project0_name = "project-0"

    # Put the first Project
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: project0_entity_key,
            Project.Fields.NAME: project0_name,
            Project.Fields.PARENT_ID: team_id,
        }
    )

    project1_id = str(uuid.uuid4())
    project1_entity_key = "{}#{}".format(EntityType.PROJECT.value, project1_id)
    project1_name = "project-1"

    # Put the second Project
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: project1_entity_key,
            Project.Fields.NAME: project1_name,
            Project.Fields.PARENT_ID: team_id,
        }
    )

    codebase_id = str(uuid.uuid4())
    codebase_entity_key = "{}#{}".format(EntityType.CODEBASE.value, codebase_id)
    codebase_name = "project-1"
    language = "JAVA"
    build_tool = "MAVEN"

    # Put A Codebase in and associate it to project1_id
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: codebase_entity_key,
            CodeBase.Fields.NAME: codebase_name,
            CodeBase.Fields.LANGUAGE: language,
            CodeBase.Fields.PARENT_ID: project1_id,
            CodeBase.Fields.BUILD_TOOL: build_tool,
        }
    )

    project2_id = str(uuid.uuid4())
    project2_entity_key = "{}#{}".format(EntityType.PROJECT.value, project2_id)
    project2_name = "project-2"

    # Put the third, unrelated Project
    harbor_teams_table.put_item(
        Item={
            HARBOR_TEAMS_TABLE_PARTITION_KEY: "Different-Parent",
            HARBOR_TEAMS_TABLE_SORT_KEY: project2_entity_key,
            Project.Fields.NAME: project2_name,
            Project.Fields.PARENT_ID: "Different-Parent",
        }
    )

    # Get the Team using the API
    team: Team = HarborDBClient(dynamodb_resource).get(
        Team(
            team_id=team_id
        ),
        recurse=True,
    )

    assert team.team_id == team_id
    assert team.entity_key == EntityType.TEAM.value
    assert team.name == team_id
    assert team.has_children()

    children = team.get_children()
    child_projects = children[EntityType.PROJECT.value]
    assert len(child_projects) == 2

    child_project0 = child_projects[0]
    child_project1 = child_projects[1]

    # Does not come back in the same order all the time.
    if child_project0.get_item()['name'] == 'project-1':
        tmp_project = child_project0
        child_project0 = child_project1
        child_project1 = tmp_project

    child_project0_item = child_project0.get_item()
    child_project1_item = child_project1.get_item()

    assert child_project0.entity_id == project0_id
    assert child_project0_item[Project.Fields.NAME] == project0_name

    assert child_project1.entity_id == project1_id
    assert child_project1_item[Project.Fields.NAME] == project1_name
    assert child_project1.has_children()

    grandchild_codebase = child_project1.get_children()[EntityType.CODEBASE.value][0].get_item()
    assert grandchild_codebase[CodeBase.Fields.NAME] == codebase_name
    assert grandchild_codebase[CodeBase.Fields.PARENT_ID] == project1_id
    assert grandchild_codebase[CodeBase.Fields.LANGUAGE] == language
    assert grandchild_codebase[CodeBase.Fields.BUILD_TOOL] == build_tool



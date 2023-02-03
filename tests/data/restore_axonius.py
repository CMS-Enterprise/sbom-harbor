#!/bin/env python3
import boto3

from cyclonedx.clients import HarborDBClient
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.member import Member
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token


def do_restore():

    db_client: HarborDBClient = HarborDBClient(
        boto3.resource("dynamodb"),
        "prod-HarborTeams-use1"
    )

    axonius_fh = open("./axonius.entities.csv", "r")
    for line in axonius_fh.readlines():

        parts = line.split(",")
        parts = [p.lstrip("\"").rstrip("\"") for p in parts]

        team_id = parts[0]
        entity_key = parts[1]
        (entity_type, entity_id) = entity_key.split("#") if "#" in entity_key else (entity_key, "")
        build_tool = parts[2]
        clone_url = parts[3]
        created = parts[4]
        email = parts[5]
        enabled = parts[6]
        expires = parts[7]
        fisma = parts[8]
        is_team_lead = parts[9]
        language = parts[10]
        name = parts[11]
        parent_id = parts[12]
        token = parts[13]

        # if entity_type == "team":
        #
        #     team_axonius = Team(
        #         team_id=team_id,
        #         name=name,
        #     )
        #     db_client.create(team_axonius)
        #
        # elif entity_type == "project":
        #
        #     proj_axonius = Project(
        #         team_id=team_id,
        #         name=name,
        #         project_id=entity_id,
        #     )
        #     db_client.create(proj_axonius)
        #
        # elif entity_type == "codebase":
        #
        #     cb_axonius = CodeBase(
        #         team_id=team_id,
        #         name=name,
        #         codebase_id=entity_id,
        #         clone_url=clone_url,
        #         language=language,
        #         project_id=parent_id,
        #         build_tool=build_tool,
        #     )
        #     db_client.create(cb_axonius)
        #
        # elif entity_type == "token":
        #
        #     token_axonius = Token(
        #         team_id=team_id,
        #         token_id=entity_id,
        #         name=name,
        #         created=created,
        #         expires=expires,
        #         enabled=True,
        #         token=token,
        #     )
        #     db_client.create(token_axonius)
        #
        # elif entity_type == "member":
        #
        #     member_axonius = Member(
        #         team_id=team_id,
        #         member_id=entity_id,
        #         email=email,
        #         is_team_lead=True,
        #     )
        #     db_client.create(member_axonius)

        member_axonius = Member(
            team_id=team_id,
            member_id=entity_id,
            email="daniel.bowne@aquia.io",
            is_team_lead=True,
        )
        db_client.create(member_axonius)

        exit()

        print(line)


do_restore()

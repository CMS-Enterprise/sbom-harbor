"""
-> Module for Getting teams for ID
"""
from json import dumps
import boto3
from boto3.dynamodb.conditions import Attr

from cyclonedx.constants import TEAM_MEMBER_TABLE_NAME
from cyclonedx.handlers.cyclonedx_util import __get_team_by_team_id


def get_teams_for_id_handler(event: dict = None, context: dict = None):

    """Handler to get all the teams for a user given their email address"""

    user_id = event["queryStringParameters"]["user_id"]
    team_table = boto3.resource("dynamodb").Table(TEAM_MEMBER_TABLE_NAME)
    team_members_query_rsp = team_table.scan(
        Select="ALL_ATTRIBUTES",
        FilterExpression=Attr("email").eq(user_id),
    )

    teams = team_members_query_rsp["Items"]
    rsp_data = [__get_team_by_team_id(team["TeamId"]) for team in teams]

    return {
        "statusCode": 200,
        "isBase64Encoded": False,
        "body": dumps(rsp_data),
    }

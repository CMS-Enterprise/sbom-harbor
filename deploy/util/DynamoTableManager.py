
from aws_cdk import (
    aws_dynamodb as dynamodb,
    aws_lambda as lambda_,
)


class DynamoTableManager(object):

    def __init__(
        self,
        team_table: dynamodb.Table,
        team_member_table: dynamodb.Table,
        team_token_table: dynamodb.Table,
        harbor_teams_table: dynamodb.Table,
    ):

        self.team_table = team_table
        self.team_member_table = team_member_table
        self.team_token_table = team_token_table
        self.harbor_teams_table = harbor_teams_table

    def grant(self, func: lambda_.Function):
        self.team_table.grant_read_write_data(func)
        self.team_member_table.grant_read_write_data(func)
        self.team_token_table.grant_read_write_data(func)
        self.harbor_teams_table.grant_read_write_data(func)

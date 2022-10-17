"""
-> Module for Class to manage all the tables we have in DynamoDB.
-> We only have one table now, which might make this class unnecessary going forward
"""
from aws_cdk import (
    aws_dynamodb as dynamodb,
    aws_lambda as lambda_,
)


class DynamoTableManager:

    """
    -> Class to manage all the tables we have in DynamoDB.
    """

    def __init__(
        self,
        harbor_teams_table: dynamodb.Table,
    ):

        """
        -> Constructor
        """

        self.harbor_teams_table = harbor_teams_table

    def grant(self, func: lambda_.Function):

        """
        -> Function to grant permissions on all the tables this class manages
        """

        self.harbor_teams_table.grant_read_write_data(func)

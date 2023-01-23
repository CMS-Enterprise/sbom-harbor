"""
-> Module to house the SBOMUploadAPIKeyAuthorizerLambda
"""
from aws_cdk import Duration
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from constructs import Construct

from cyclonedx.constants import AWS_ACCOUNT_ID, ENVIRONMENT
from deploy.constants import API_KEY_AUTHORIZER_LN, SBOM_API_PYTHON_RUNTIME
from deploy.util import DynamoTableManager, create_asset


class SBOMUploadAPIKeyAuthorizerLambda(Construct):

    """Lambda to check DynamoDB for a token belonging to the team sending an SBOM"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
    ):

        super().__init__(scope, API_KEY_AUTHORIZER_LN)

        self.func = lambda_.Function(
            self,
            API_KEY_AUTHORIZER_LN,
            function_name=API_KEY_AUTHORIZER_LN,
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(
                subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
            ),
            handler="cyclonedx.handlers.api_key_authorizer_handler",
            code=create_asset(self),
            timeout=Duration.minutes(2),
            memory_size=512,
            environment={
                "CDK_DEFAULT_ACCOUNT": AWS_ACCOUNT_ID,
                "ENVIRONMENT": ENVIRONMENT,
            },
        )

        table_mgr.grant(self.func)

    def get_lambda_function(self):

        """
        -> Get the CDK Lambda Construct
        """

        return self.func

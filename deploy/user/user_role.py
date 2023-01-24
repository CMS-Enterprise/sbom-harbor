""" This module is where all the higher level CDK constructs are stored """
from aws_cdk import RemovalPolicy
from aws_cdk import aws_iam as iam
from constructs import Construct

from deploy.constants import USER_ROLE_ID, USER_ROLE_NAME
from deploy.user import SBOMUserPool


class SBOMUserRole(Construct):
    """
    This class is used to create the IAM role for the user pool .
    params:
        scope: Construct
        user_pool: SBOMUserPool
    """

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
    ):

        super().__init__(scope, USER_ROLE_ID)

        self.node.add_dependency(user_pool)

        self.user_role = iam.Role(
            self,
            USER_ROLE_ID,
            role_name=USER_ROLE_NAME,
            description="Default role for authenticated users",
            assumed_by=iam.FederatedPrincipal(
                "cognito-identity.amazonaws.com",
                {
                    "StringEquals": {
                        "cognito-identity.amazonaws.com:aud": user_pool.get_cognito_user_pool().user_pool_id,
                    },
                    "ForAnyValue:StringLike": {
                        "cognito-identity.amazonaws.com:amr": "authenticated",
                    },
                },
            ),
            managed_policies=[
                iam.ManagedPolicy.from_aws_managed_policy_name(
                    "service-role/AWSLambdaBasicExecutionRole",
                ),
                iam.ManagedPolicy.from_aws_managed_policy_name(
                    # TODO: remove this when we have a better solution
                    "AmazonS3FullAccess",
                ),
            ],
        )

        self.user_role.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_role(self) -> iam.Role:
        return self.user_role

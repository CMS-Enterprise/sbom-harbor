"""This Stack is used to set up shared resources
that the other stacks use when deploying the application"""

from aws_cdk import Stack
from aws_cdk import aws_cognito as cognito
from aws_cdk import aws_iam as iam
from constructs import Construct

from deploy.constants import USER_MANAGEMENT_STACK_ID
from deploy.user import (
    SBOMUserPool,
    SBOMUserPoolClient,
    SBOMUserPoolGroup,
    SBOMUserRole,
)


class SBOMUserManagement(Stack):
    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, USER_MANAGEMENT_STACK_ID, **kwargs)

        # Create the Cognito user pool
        user_pool = SBOMUserPool(self)
        self.cognito_user_pool = user_pool.get_cognito_user_pool()

        # Create the Cognito user pool app client
        user_pool_client = SBOMUserPoolClient(self, user_pool=user_pool)
        self.cognito_user_pool_client = user_pool_client.get_cognito_user_pool_client()

        # Create the role for the Cognito user pool group
        user_role = SBOMUserRole(self, user_pool=user_pool)
        self.cognito_user_role = user_role.get_cognito_user_role()

        # Create the Cognito "admin" user pool group
        user_pool_group = SBOMUserPoolGroup(
            self, user_pool=user_pool, user_role=user_role
        )
        self.cognito_user_pool_group = user_pool_group.get_cognito_user_pool_group()

    def get_user_role(self) -> iam.Role:
        """Gets the user pool"""

        return self.cognito_user_role

    def get_user_pool(self) -> cognito.UserPool:
        """Gets the user pool"""

        return self.cognito_user_pool

    def get_user_pool_client(self) -> cognito.UserPoolClient:
        """Gets the user pool app client"""

        return self.cognito_user_pool_client

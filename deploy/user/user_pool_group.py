from aws_cdk import RemovalPolicy
from aws_cdk import aws_cognito as cognito
from constructs import Construct

from deploy.constants import (
    USER_POOL_GROUP_DESCRIPTION,
    USER_POOL_GROUP_ID,
    USER_POOL_GROUP_NAME,
)
from deploy.user import SBOMUserPool, SBOMUserRole


class SBOMUserPoolGroup(Construct):

    """
    This class is used to create the user pool group.
    params:
        scope: Construct
        user_pool: SBOMUserPool
        user_role: SBOMUserRole
    """

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
        user_role: SBOMUserRole,
    ):

        super().__init__(scope, USER_POOL_GROUP_ID)

        for dep in [user_pool, user_role]:
            self.node.add_dependency(dep)

        self.user_pool_group = cognito.CfnUserPoolGroup(
            self,
            USER_POOL_GROUP_ID,
            description=USER_POOL_GROUP_DESCRIPTION,
            group_name=USER_POOL_GROUP_NAME,
            precedence=1,
            role_arn=user_role.get_cognito_user_role().role_arn,
            user_pool_id=user_pool.get_cognito_user_pool().user_pool_id,
        )

        self.user_pool_group.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_pool_group(self) -> cognito.CfnUserPoolGroup:
        return self.user_pool_group

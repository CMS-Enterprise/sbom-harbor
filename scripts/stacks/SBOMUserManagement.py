
"""This Stack is used to set up shared resources
that the other stacks use when deploying the application"""
from constructs import Construct
from aws_cdk import (
    aws_ec2 as ec2,
    aws_iam as iam,
    aws_cognito as cognito,
    RemovalPolicy,
    Stack,
)
from scripts.constants import (
    ADMIN_USER_ID,
    ADMIN_USER_USERNAME,
    USER_MANAGEMENT_STACK_ID,
    USER_POOL_GROUP_NAME,
    USER_POOL_NAME
)
from scripts.constructs import (
    ApplicationLoadBalancer,
    SBOMUserPool,
    SBOMUserPoolClient,
    SBOMUserPoolDomain,
    SBOMUserPoolGroup,
    SBOMUserRole,
)


class SBOMUserManagement(Stack):

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, USER_MANAGEMENT_STACK_ID, **kwargs)

        # Create the Cognito user pool
        user_pool = SBOMUserPool(self)
        self.cognito_user_pool = user_pool.get_cognito_user_pool()

        # Create the Cognito user pool domain
        user_pool_domain = SBOMUserPoolDomain(self, user_pool=user_pool)
        self.cognito_user_pool_domain = user_pool_domain.get_cognito_user_pool_domain()

        # Create the Cognito user pool app client
        user_pool_client = SBOMUserPoolClient(self, user_pool=user_pool)
        self.cognito_user_pool_client = user_pool_client.get_cognito_user_pool_client()

        # Create the Application Load Balancer
        lb = ApplicationLoadBalancer(
            self, vpc=vpc,
            user_pool=user_pool,
            user_pool_client=user_pool_client,
            user_pool_domain=user_pool_domain,
        )
        self.lb = lb.get_load_balancer()

        # Create the role for the Cognito user pool group
        user_role = SBOMUserRole(self, user_pool=user_pool)
        self.cognito_user_role = user_role.get_cognito_user_role()

        # Create the Cognito "admin" user pool group
        user_pool_group = SBOMUserPoolGroup(self, user_pool=user_pool, user_role=user_role)
        self.cognito_user_pool_group = user_pool_group.get_cognito_user_pool_group()

        for dependency in user_pool, user_pool_group, user_role:
            admin_user.node.add_dependency(dependency)

        # Add the admin user to the "admin" user pool group
        group_attachment = cognito.CfnUserPoolUserToGroupAttachment(
            self,
            f"{USER_POOL_GROUP_NAME}_{USER_POOL_NAME}_{ADMIN_USER_ID}",
            group_name=USER_POOL_GROUP_NAME,
            user_pool_id=self.cognito_user_pool.user_pool_id,
            username=ADMIN_USER_USERNAME,
        )
        group_attachment.apply_removal_policy(RemovalPolicy.DESTROY)
        group_attachment.add_depends_on(admin_user)
        for dependency in user_pool, user_pool_group, user_role:
            group_attachment.node.add_dependency(dependency)

    def get_user_role(self) -> iam.Role:

        """Gets the user pool"""

        return self.cognito_user_role

    def get_user_pool(self) -> cognito.UserPool:

        """Gets the user pool"""

        return self.cognito_user_pool

    def get_user_pool_client(self) -> cognito.UserPoolClient:
        """Gets the user pool app client"""

        return self.cognito_user_pool_client

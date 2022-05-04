"""This Stack is used to set up shared resources
that the other stacks use when deploying the application"""

from aws_cdk import (
    aws_cognito as cognito,
    aws_s3 as s3,
    RemovalPolicy,
    Stack,
)
from constructs import Construct
from scripts.constructs import (
    ApplicationLoadBalancer,
    SBOMApiVpc,
    SBOMUserPoolDomain,
    SBOMUserPoolGroup,
    SBOMUserRole,
    SBOMUserPool,
    SBOMUserPoolClient,
)
from scripts.constants import (
    ADMIN_USER_ID,
    ADMIN_USER_USERNAME,
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    SHARED_RESOURCE_STACK_ID,
    USER_POOL_GROUP_NAME,
    USER_POOL_NAME,
)


class SBOMSharedResourceStack(Stack):

    """This Stack is used to set up shared resources
    that the other stacks use when deploying the application"""

    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, SHARED_RESOURCE_STACK_ID, **kwargs)

        # Create the VPC
        vpc = SBOMApiVpc(self)
        self.vpc = vpc.get_vpc()

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
        lb = ApplicationLoadBalancer(self,
            vpc=self.vpc,
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

        # Create the admin user
        admin_user = cognito.CfnUserPoolUser(
            self,
            ADMIN_USER_ID,
            force_alias_creation=False,
            username=ADMIN_USER_USERNAME,
            user_pool_id=self.cognito_user_pool.user_pool_id,
            user_attributes=[
                cognito.CfnUserPoolUser.AttributeTypeProperty(
                    name="custom:role_name",
                    value=self.cognito_user_role.role_name,
                )
            ],
        )
        admin_user.apply_removal_policy(RemovalPolicy.DESTROY)
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

        # Create the S3 Bucket to put the BOMs in
        self.s3_bucket = s3.Bucket(
            self,
            S3_BUCKET_ID,
            bucket_name=S3_BUCKET_NAME,
            removal_policy=RemovalPolicy.DESTROY,
            auto_delete_objects=True,
        )

    def get_vpc(self):

        """Gets the VPC"""

        return self.vpc


    def get_lb(self):

        """Gets the Application Load Balancer"""

        return self.lb


    def get_user_role(self):

        """Gets the user pool"""

        return self.cognito_user_role


    def get_user_pool(self):

        """Gets the user pool"""

        return self.cognito_user_pool


    def get_user_pool_client(self):

        """Gets the user pool app client"""

        return self.cognito_user_pool_client


    def get_s3_bucket(self):

        """Gets the S3 Bucket"""

        return self.s3_bucket

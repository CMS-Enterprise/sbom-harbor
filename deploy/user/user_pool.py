""" This module is where all the higher level CDK constructs are stored """
from aws_cdk import (
    aws_cognito as cognito,
    RemovalPolicy,
)
from constructs import Construct

from deploy.constants import (
    USER_POOL_ID,
    USER_POOL_NAME,
)


class SBOMUserPool(Construct):

    """
    This class is used to create the user pool
    used throughout the application.
    """

    def __init__(
        self,
        scope: Construct,
    ):

        super().__init__(scope, USER_POOL_ID)

        self.id = USER_POOL_ID

        self.user_pool = cognito.UserPool(
            self,
            USER_POOL_ID,
            user_pool_name=USER_POOL_NAME,
            account_recovery=cognito.AccountRecovery.EMAIL_ONLY,
            auto_verify=cognito.AutoVerifiedAttrs(
                email=True,
            ),
            custom_attributes={
                "role_name": cognito.StringAttribute(min_len=5, max_len=15, mutable=False),
                "team_id": cognito.StringAttribute(min_len=5, max_len=15, mutable=False),
            },
            self_sign_up_enabled=True,
            sign_in_aliases=cognito.SignInAliases(
                email=True,
                phone=False,
                username=False,
                preferred_username=False,
            ),
            sign_in_case_sensitive=False,
            standard_attributes=cognito.StandardAttributes(
                email=cognito.StandardAttribute(
                    required=True,
                    mutable=False,
                ),
                fullname=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                given_name=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                family_name=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                locale=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
                timezone=cognito.StandardAttribute(
                    required=False,
                    mutable=True,
                ),
            ),
            password_policy=cognito.PasswordPolicy(
                min_length=8,
                require_symbols=True,
                require_digits=True,
                require_lowercase=True,
                require_uppercase=True,
            ),
            removal_policy=RemovalPolicy.DESTROY,
        )

    def get_cognito_user_pool(self) -> cognito.UserPool:
        return self.user_pool
    
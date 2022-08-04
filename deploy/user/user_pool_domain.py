from aws_cdk import (
    aws_cognito as cognito,
    RemovalPolicy,
)
from constructs import Construct

from deploy.constants import (
    COGNITO_DOMAIN_PREFIX,
    USER_POOL_DOMAIN_ID,
)
from deploy.user import SBOMUserPool


class SBOMUserPoolDomain(Construct):

    """This class is used to create the user pool app client domain."""

    def __init__(
        self,
        scope: Construct,
        *,
        user_pool: SBOMUserPool,
    ):

        super().__init__(scope, USER_POOL_DOMAIN_ID)

        self.node.add_dependency(user_pool)

        self.user_pool_domain = cognito.UserPoolDomain(
            self,
            USER_POOL_DOMAIN_ID,
            user_pool=user_pool.get_cognito_user_pool(),
            cognito_domain=cognito.CognitoDomainOptions(
                domain_prefix=COGNITO_DOMAIN_PREFIX,
            ),
        )

        self.user_pool_domain.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_pool_domain(self) -> cognito.UserPoolDomain:
        return self.user_pool_domain

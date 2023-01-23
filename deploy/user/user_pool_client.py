from aws_cdk import RemovalPolicy
from aws_cdk import aws_cognito as cognito
from constructs import Construct

from deploy.constants import USER_POOL_APP_CLIENT_ID
from deploy.user import SBOMUserPool


class SBOMUserPoolClient(Construct):

    """
    This class is used to create the user pool app client.
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

        super().__init__(scope, USER_POOL_APP_CLIENT_ID)

        self.node.add_dependency(user_pool)

        client_write_attributes = (cognito.ClientAttributes()).with_standard_attributes(
            email=True,
            phone_number=True,
            family_name=True,
            fullname=True,
            given_name=True,
            locale=True,
            preferred_username=True,
            timezone=True,
        )

        client_read_attributes = (
            (client_write_attributes)
            .with_standard_attributes(
                email_verified=True,
                phone_number_verified=True,
            )
            .with_custom_attributes(
                "custom:teams",
            )
        )

        self.client = cognito.UserPoolClient(
            self,
            USER_POOL_APP_CLIENT_ID,
            user_pool=user_pool.get_cognito_user_pool(),
            auth_flows=cognito.AuthFlow(
                custom=True,
                admin_user_password=True,
                user_password=True,
                # NOTE: USER_SRP is required for authenticating the
                #   client application using the Amplify JS library.
                user_srp=True,
            ),
            o_auth=cognito.OAuthSettings(
                flows=cognito.OAuthFlows(
                    authorization_code_grant=False,
                    client_credentials=False,
                    # TODO: The implicit grant flow exposes OAuth tokens in
                    #   the url. AWS recommends that only the team_management
                    #   code flow is used with PKCE for public clients.
                    implicit_code_grant=True,
                ),
            ),
            enable_token_revocation=True,
            prevent_user_existence_errors=True,
            read_attributes=client_read_attributes,
            write_attributes=client_write_attributes,
        )

        cfn_client = self.client.node.default_child
        cfn_client.add_property_override("RefreshTokenValidity", 1)
        cfn_client.add_property_override("SupportedIdentityProviders", ["COGNITO"])

        self.client.apply_removal_policy(RemovalPolicy.DESTROY)

    def get_cognito_user_pool_client(self) -> cognito.UserPoolClient:
        return self.client

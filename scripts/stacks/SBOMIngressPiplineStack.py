"""This stack deploys the Ingress Pipeline"""

from os import path
from aws_cdk import (
    CfnOutput,
    aws_logs as logs,
    aws_apigatewayv2_alpha as apigwv2a,
    aws_apigatewayv2 as apigwv2,
    aws_ec2 as ec2,
    aws_cognito as cognito,
    aws_s3 as s3,
    Stack,
)
from aws_cdk.aws_apigatewayv2_alpha import (
    CorsHttpMethod,
    CorsPreflightOptions,
    HttpNoneAuthorizer,
)
from aws_cdk.aws_apigatewayv2_integrations_alpha import HttpLambdaIntegration
from aws_cdk.aws_apigatewayv2_authorizers_alpha import HttpLambdaAuthorizer
from aws_cdk.aws_logs import RetentionDays
from constructs import Construct
from scripts.constants import (
    API_GW_ID_EXPORT_NAME,
    API_GW_URL_EXPORT_ID,
    AUTHORIZATION_HEADER,
    CREATE_TOKEN_LN,
    DELETE_TOKEN_LN,
    GET_TEAMS_FOR_ID_LN,
    GET_TEAM_LN,
    INGRESS_STACK_ID,
    REGISTER_TEAM_LN,
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    UPDATE_TEAM_LN,
    USER_SEARCH_LN,
)
from scripts.constructs import (
    AuthorizerLambdaFactory,
    PristineSbomIngressLambda,
    SBOMCreateTokenLambda,
    SBOMDeleteTokenLambda,
    SBOMGetTeamLambda,
    SBOMGetTeamsForUserIdLambda,
    SBOMLoginLambda,
    SBOMRegisterTeamLambda,
    SBOMUpdateTeamLambda,
    SBOMUploadAPIKeyAuthorizerLambda,
    SBOMUserSearchLambda,
)
from scripts.util import DynamoTableManager


class SBOMIngressPiplineStack(Stack):

    """This stack deploys the Ingress Pipeline"""

    __cwd = path.dirname(__file__)

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        user_pool: cognito.UserPool,
        user_pool_client: cognito.UserPoolClient,
        table_mgr: DynamoTableManager,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, INGRESS_STACK_ID, **kwargs)

        self.api = apigwv2a.HttpApi(
            self, id="SBOMApi",
            description="SBOM API (ACTUAL)",
            cors_preflight=CorsPreflightOptions(
                allow_origins=["*"],
                allow_headers=[
                    AUTHORIZATION_HEADER,
                    'X-Api-Key',
                    'Content-Type',
                    'X-Amz-Date',
                ],
                allow_methods=[
                    CorsHttpMethod.GET,
                    CorsHttpMethod.POST,
                    CorsHttpMethod.PUT,
                    CorsHttpMethod.HEAD,
                    CorsHttpMethod.DELETE,
                    CorsHttpMethod.OPTIONS,
                    CorsHttpMethod.PATCH,
                ]
            ),
        )

        authorizer_factory = AuthorizerLambdaFactory(self, vpc)

        self.__enable_logging(self.api, False)
        self.__generate_apigw_url_output()
        self.__add_login_route(user_pool_client, user_pool, vpc)
        self.__add_user_search_route(user_pool_client, user_pool, vpc, authorizer_factory)
        self.__add_team_routes(vpc, table_mgr, authorizer_factory)
        self.__add_token_routes(vpc, table_mgr, authorizer_factory)
        self.__add_sbom_upload_route(vpc, table_mgr)

    def __add_user_search_route(
            self, user_pool_client: cognito.UserPoolClient,
            user_pool: cognito.UserPool, vpc: ec2.Vpc,
            authorizer_factory: AuthorizerLambdaFactory,
    ):

        self.api.add_routes(
            path="/api/user/search",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "USER_SEARCH_HttpLambdaIntegration_ID",
                handler=SBOMUserSearchLambda(
                    self, vpc=vpc,
                    user_pool_client=user_pool_client,
                    user_pool=user_pool
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "UserSearch_HttpJwtAuthorizer_ID",
                authorizer_name="UserSearchHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    USER_SEARCH_LN
                ).get_lambda_function()
            )
        )

    def __add_login_route(
            self,
            user_pool_client: cognito.UserPoolClient,
            user_pool: cognito.UserPool,
            vpc: ec2.Vpc
    ):

        """ Adds the /api/login endpoint for getting a JWT and logging in """

        client_id = user_pool_client.user_pool_client_id
        user_pool_id = user_pool.user_pool_id

        self.api.add_routes(
            path="/api/login",
            authorizer=HttpNoneAuthorizer(),
            methods=[apigwv2a.HttpMethod.POST],
            integration=HttpLambdaIntegration(
                "LOGIN_HttpLambdaIntegration_ID",
                handler=SBOMLoginLambda(
                    self, vpc=vpc,
                    user_pool_client_id=client_id,
                    user_pool_id=user_pool_id,
                ).get_lambda_function(),
            )
        )

    def __add_team_routes(
            self, vpc: ec2.Vpc,
            dynamo_table_mgr: DynamoTableManager,
            authorizer_factory: AuthorizerLambdaFactory
    ):

        self.api.add_routes(
            path="/api/team/{team}",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "GET_TEAM_HttpLambdaIntegration_ID",
                handler=SBOMGetTeamLambda(
                    self, vpc=vpc,
                    table_mgr=dynamo_table_mgr,
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "GetTeam_HttpJwtAuthorizer_ID",
                authorizer_name="GetTeamHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    GET_TEAM_LN
                ).get_lambda_function()
            )
        )

        self.api.add_routes(
            path="/api/team",
            methods=[apigwv2a.HttpMethod.POST],
            integration=HttpLambdaIntegration(
                "REGISTER_TEAM_HttpLambdaIntegration_ID",
                handler=SBOMRegisterTeamLambda(
                    self, vpc=vpc,
                    table_mgr=dynamo_table_mgr,
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "RegisterTeam_HttpJwtAuthorizer_ID",
                authorizer_name="RegisterTeamHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    REGISTER_TEAM_LN
                ).get_lambda_function()
            )
        )

        self.api.add_routes(
            path="/api/team",
            methods=[apigwv2a.HttpMethod.PUT],
            integration=HttpLambdaIntegration(
                "UPDATE_TEAM_HttpLambdaIntegration_ID",
                handler=SBOMUpdateTeamLambda(
                    self, vpc=vpc,
                    table_mgr=dynamo_table_mgr,
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "UpdateTeam_HttpJwtAuthorizer_ID",
                authorizer_name="UpdateTeamHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    UPDATE_TEAM_LN
                ).get_lambda_function()
            )
        )

        self.api.add_routes(
            path="/api/user/teams",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "GET_TEAMS_FOR_ID_HttpLambdaIntegration_ID",
                handler=SBOMGetTeamsForUserIdLambda(
                    self, vpc=vpc,
                    table_mgr=dynamo_table_mgr,
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "GetTeamsForId_HttpJwtAuthorizer_ID",
                authorizer_name="GetTeamsForIdHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    GET_TEAMS_FOR_ID_LN
                ).get_lambda_function()
            )
        )

    def __add_token_routes(
        self, vpc: ec2.Vpc,
        dynamo_table_mgr: DynamoTableManager,
        authorizer_factory: AuthorizerLambdaFactory,
    ):

        """ Adds the create and delete token lambdas """

        self.api.add_routes(
            path="/api/{team}/token",
            methods=[apigwv2a.HttpMethod.POST],
            integration=HttpLambdaIntegration(
                "CREATE_TOKEN_HttpLambdaIntegration_ID",
                handler=SBOMCreateTokenLambda(
                    self, vpc=vpc, table_mgr=dynamo_table_mgr
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "CreateTokenHttpJwtAuthorizer_ID",
                authorizer_name="CreateTokenHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    CREATE_TOKEN_LN
                ).get_lambda_function()
            )
        )

        self.api.add_routes(
            path="/api/{team}/token/{token}",
            methods=[apigwv2a.HttpMethod.DELETE],
            integration=HttpLambdaIntegration(
                "DELETE_TOKEN_HttpLambdaIntegration_ID",
                handler=SBOMDeleteTokenLambda(
                    self, vpc=vpc, table_mgr=dynamo_table_mgr
                ).get_lambda_function(),
            ),
            authorizer=HttpLambdaAuthorizer(
                "DeleteTokenHttpJwtAuthorizer_ID",
                authorizer_name="DeleteTokenHttpJwtAuthorizer",
                handler=authorizer_factory.create(
                    DELETE_TOKEN_LN
                ).get_lambda_function()
            )
        )

    def __add_sbom_upload_route(
            self, vpc: ec2.Vpc,
            dynamo_table_mgr: DynamoTableManager,
    ):

        """ Adds the /api/sbom route """

        self.api.add_routes(
            path="/api/{team}/{project}/{codebase}/sbom",
            methods=[apigwv2a.HttpMethod.POST],
            authorizer=HttpLambdaAuthorizer(
                "UPLOAD_SBOM_HttpLambdaAuthorizer_ID",
                authorizer_name="UPLOAD_SBOM_HttpLambdaAuthorizer",
                handler=SBOMUploadAPIKeyAuthorizerLambda(
                    self, vpc=vpc, table_mgr=dynamo_table_mgr
                ).get_lambda_function()
            ),
            integration=HttpLambdaIntegration(
                "UPLOAD_SBOM_HttpLambdaIntegration_ID",
                handler=PristineSbomIngressLambda(
                    self, vpc=vpc,
                    s3_bucket=s3.Bucket.from_bucket_name(
                        self, id=S3_BUCKET_ID,
                        bucket_name=S3_BUCKET_NAME,
                    ),
                ).get_lambda_function(),
            )
        )

    def __generate_apigw_url_output(self):

        """ Create an output with the URL of this
        API so Cloudfront can forward the requests """

        CfnOutput(
            self,
            API_GW_URL_EXPORT_ID,
            # TODO: get rest api url dynamically for "value" below
            value=f"{self.api.http_api_id}.execute-api.us-east-1.amazonaws.com",
            export_name=API_GW_ID_EXPORT_NAME,
            description="URL Of the API Gateway",
        )

    @staticmethod
    def __enable_logging(api: apigwv2a.HttpApi, enabled):

        """ Enables logging if necessary """

        if enabled:

            stage: apigwv2.CfnStage = api.default_stage.node.default_child
            log_group = logs.LogGroup(
                api, 'AccessLogs',
                log_group_name="APIGWAccessLogs",
                retention=RetentionDays.ONE_DAY,
            )

            stage.access_log_settings = apigwv2.CfnStage.AccessLogSettingsProperty(
                destination_arn=log_group.log_group_arn,
                format="$context.requestId $context.error.messageString $context.integration.error",
            )

"""This stack deploys the Ingress Pipeline"""

from os import path

from aws_cdk import CfnOutput, Duration, RemovalPolicy, Stack
from aws_cdk import aws_apigatewayv2 as apigwv2
from aws_cdk import aws_apigatewayv2_alpha as apigwv2a
from aws_cdk import aws_cognito as cognito
from aws_cdk import aws_ec2 as ec2
from aws_cdk import aws_lambda as lambda_
from aws_cdk import aws_logs as logs
from aws_cdk import aws_s3 as s3
from aws_cdk.aws_apigatewayv2_alpha import (
    CorsHttpMethod,
    CorsPreflightOptions,
    HttpNoneAuthorizer,
)
from aws_cdk.aws_apigatewayv2_authorizers_alpha import HttpLambdaAuthorizer
from aws_cdk.aws_apigatewayv2_integrations_alpha import HttpLambdaIntegration
from aws_cdk.aws_iam import Effect, PolicyStatement
from aws_cdk.aws_logs import RetentionDays
from constructs import Construct

from deploy.authorizers import AuthorizerLambdaFactory, SBOMUploadAPIKeyAuthorizerLambda
from deploy.constants import (
    API_GW_ID_OUTPUT_ID,
    API_GW_LOG_GROUP_NAME,
    API_GW_URL_EXPORT_NAME,
    API_GW_URL_OUTPUT_ID,
    API_STACK_ID,
    AUTHORIZATION_HEADER,
    AUTHORIZER_LN,
    AWS_ACCOUNT_ID,
    AWS_REGION_SHORT,
    ENVIRONMENT,
    S3_BUCKET_ID,
    S3_BUCKET_NAME,
    SBOM_API_PYTHON_RUNTIME,
    USER_POOL_CLIENT_ID_KEY,
    USER_POOL_ID_KEY,
)
from deploy.user import SBOMLoginLambda, SBOMUserSearchLambda
from deploy.util import DynamoTableManager, SbomIngressLambda, create_asset


class LambdaFactory:

    """
    -> LambdaFactory creates Lambda configurations
    """

    class HarborLambda(Construct):

        """
        -> Lambda to check DynamoDB for a token
        -> belonging to the team sending an SBOM
        """

        # pylint: disable = R0913
        def __init__(
            self,
            scope: Construct,
            vpc: ec2.Vpc,
            table_mgr: DynamoTableManager,
            handler: str,
            name: str,
            user_pool_id: str,
            user_pool_client_id: str,
        ):

            super().__init__(scope, name)

            self.lambda_func = lambda_.Function(
                self,
                name,
                function_name=name,
                runtime=SBOM_API_PYTHON_RUNTIME,
                vpc=vpc,
                vpc_subnets=ec2.SubnetSelection(
                    subnet_type=ec2.SubnetType.PRIVATE_WITH_EGRESS,
                ),
                handler=handler,
                code=create_asset(self),
                timeout=Duration.minutes(2),
                memory_size=512,
                environment={
                    USER_POOL_ID_KEY: user_pool_id,
                    USER_POOL_CLIENT_ID_KEY: user_pool_client_id,
                    "CDK_DEFAULT_ACCOUNT": AWS_ACCOUNT_ID,
                    "ENVIRONMENT": ENVIRONMENT,
                },
            )

            self.lambda_func.add_to_role_policy(
                PolicyStatement(
                    effect=Effect.ALLOW,
                    actions=[
                        "cognito-idp:AdminDisableUser",
                        "cognito-idp:AdminEnableUser",
                        "cognito-idp:AdminGetUser",
                        "cognito-idp:ListUsers",
                        "cognito-idp:AdminUpdateUserAttributes",
                    ],
                    resources=["*"],
                )
            )

            table_mgr.grant(self.lambda_func)

        def get_lambda_function(self):

            """
            -> Returns the Lambda CDK Type
            """

            return self.lambda_func

    # pylint: disable = R0913
    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
        user_pool_id: str,
        user_pool_client_id: str,
    ):
        self.scope = scope
        self.vpc = vpc
        self.table_mgr = table_mgr
        self.user_pool_id = user_pool_id
        self.user_pool_client_id = user_pool_client_id

    def create(self, lambda_name: str, func: str):

        """
        -> SBOMLambda
        """

        return LambdaFactory.HarborLambda(
            self.scope,
            vpc=self.vpc,
            name=f"{ENVIRONMENT}_Harbor_{lambda_name}_Lambda_{AWS_REGION_SHORT}",
            table_mgr=self.table_mgr,
            handler=func,
            user_pool_id=self.user_pool_id,
            user_pool_client_id=self.user_pool_client_id,
        )


class SBOMIngressApiStack(Stack):

    """This stack deploys the Ingress Api"""

    __cwd = path.dirname(__file__)

    @staticmethod
    def __enable_logging(api: apigwv2a.HttpApi):

        """
        -> Enables logging if necessary
        """

        stage: apigwv2.CfnStage = api.default_stage.node.default_child
        log_group = logs.LogGroup(
            api,
            "AccessLogs",
            log_group_name=API_GW_LOG_GROUP_NAME,
            retention=RetentionDays.ONE_DAY,
            removal_policy=RemovalPolicy.DESTROY,
        )

        stage.access_log_settings = apigwv2.CfnStage.AccessLogSettingsProperty(
            destination_arn=log_group.log_group_arn,
            format="$context.requestId $context.error.messageString $context.integration.error",
        )

    # pylint: disable=R0913
    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
        user_pool: cognito.UserPool,
        user_pool_client: cognito.UserPoolClient,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, API_STACK_ID, **kwargs)

        authorizer_factory = AuthorizerLambdaFactory(
            self,
            vpc=vpc,
            user_pool_id=user_pool.user_pool_id,
            user_pool_client_id=user_pool_client.user_pool_client_id,
        )

        self.api = apigwv2a.HttpApi(
            self,
            id="SBOMManagementApi",
            default_authorizer=HttpLambdaAuthorizer(
                id=AUTHORIZER_LN,
                authorizer_name=AUTHORIZER_LN,
                handler=authorizer_factory.create(AUTHORIZER_LN).get_lambda_function(),
            ),
            description="SBOM Management API (Experimental)",
            cors_preflight=CorsPreflightOptions(
                allow_origins=["*"],
                allow_headers=[
                    AUTHORIZATION_HEADER,
                    "X-Api-Key",
                    "Content-Type",
                    "X-Amz-Date",
                ],
                allow_methods=[
                    CorsHttpMethod.GET,
                    CorsHttpMethod.POST,
                    CorsHttpMethod.PUT,
                    CorsHttpMethod.HEAD,
                    CorsHttpMethod.DELETE,
                    CorsHttpMethod.OPTIONS,
                    CorsHttpMethod.PATCH,
                ],
            ),
        )

        #SBOMIngressApiStack.__enable_logging(self.api)

        lambda_factory = LambdaFactory(
            self,
            vpc=vpc,
            table_mgr=table_mgr,
            user_pool_id=user_pool.user_pool_id,
            user_pool_client_id=user_pool_client.user_pool_client_id,
        )

        self.__add_team_routes(
            lambda_factory=lambda_factory,
        )

        self.__add_project_routes(
            lambda_factory=lambda_factory,
        )

        self.__add_codebase_routes(
            lambda_factory=lambda_factory,
        )

        self.__add_token_routes(
            lambda_factory=lambda_factory,
        )

        self.__add_member_routes(
            lambda_factory=lambda_factory,
        )

        self.__add_login_route(
            vpc=vpc,
            user_pool=user_pool,
            user_pool_client=user_pool_client,
        )

        self.__add_user_search_route(
            vpc=vpc,
            user_pool=user_pool,
            user_pool_client=user_pool_client,
        )

        self.__add_sbom_upload_route(
            vpc=vpc,
            dynamo_table_mgr=table_mgr,
        )

        self.__generate_apigw_url_output()

    def __generate_apigw_url_output(self):

        """
        -> Create an output with the URL of this
        -> API so Cloudfront can forward the requests
        """

        CfnOutput(
            self,
            API_GW_URL_OUTPUT_ID,
            value=self.api.url.replace("https://", "").replace("/", ""),
            export_name=API_GW_URL_EXPORT_NAME,
            description="URL Of the API Gateway",
        )

        CfnOutput(
            self,
            API_GW_ID_OUTPUT_ID,
            value=self.api.api_id,
            description="ID Of the API Gateway",
        )

    def __add_team_routes(
        self: "SBOMIngressApiStack",
        lambda_factory: LambdaFactory,
    ):

        self.api.add_routes(
            path="/api/v1/teams",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "Teams_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Teams",
                    func="cyclonedx.handlers.teams_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/team/{team}",
            methods=[
                apigwv2a.HttpMethod.GET,
                apigwv2a.HttpMethod.PUT,
                apigwv2a.HttpMethod.DELETE,
            ],
            integration=HttpLambdaIntegration(
                "Team_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Team",
                    func="cyclonedx.handlers.team_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/team",
            methods=[
                apigwv2a.HttpMethod.POST,
            ],
            integration=HttpLambdaIntegration(
                "Team_HttpLambdaIntegration_POST",
                handler=lambda_factory.create(
                    lambda_name="Team_POST",
                    func="cyclonedx.handlers.teams.team_handler",
                ).get_lambda_function(),
            ),
        )

    def __add_project_routes(
        self: "SBOMIngressApiStack",
        lambda_factory: LambdaFactory,
    ):

        self.api.add_routes(
            path="/api/v1/projects",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "Projects_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Projects",
                    func="cyclonedx.handlers.projects_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/project/{project}",
            methods=[
                apigwv2a.HttpMethod.GET,
                apigwv2a.HttpMethod.PUT,
                apigwv2a.HttpMethod.DELETE,
            ],
            integration=HttpLambdaIntegration(
                "Project_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Project",
                    func="cyclonedx.handlers.project_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/project",
            methods=[
                apigwv2a.HttpMethod.POST,
            ],
            integration=HttpLambdaIntegration(
                "Project_HttpLambdaIntegration_POST",
                handler=lambda_factory.create(
                    lambda_name="Project_POST",
                    func="cyclonedx.handlers.project_handler",
                ).get_lambda_function(),
            ),
        )

    def __add_codebase_routes(
        self: "SBOMIngressApiStack",
        lambda_factory: LambdaFactory,
    ):

        self.api.add_routes(
            path="/api/v1/codebases",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "Codebases_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Codebases",
                    func="cyclonedx.handlers.codebases_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/codebase/{codebase}",
            methods=[
                apigwv2a.HttpMethod.GET,
                apigwv2a.HttpMethod.PUT,
                apigwv2a.HttpMethod.DELETE,
            ],
            integration=HttpLambdaIntegration(
                "Codebase_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Codebase",
                    func="cyclonedx.handlers.codebase_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/codebase",
            methods=[
                apigwv2a.HttpMethod.POST,
            ],
            integration=HttpLambdaIntegration(
                "Codebase_HttpLambdaIntegration_POST",
                handler=lambda_factory.create(
                    lambda_name="Codebase_POST",
                    func="cyclonedx.handlers.codebase_handler",
                ).get_lambda_function(),
            ),
        )

    def __add_token_routes(
        self: "SBOMIngressApiStack",
        lambda_factory: LambdaFactory,
    ):

        self.api.add_routes(
            path="/api/v1/tokens",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "Tokens_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Tokens",
                    func="cyclonedx.handlers.tokens_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/token/{token}",
            methods=[
                apigwv2a.HttpMethod.GET,
                apigwv2a.HttpMethod.PUT,
                apigwv2a.HttpMethod.DELETE,
            ],
            integration=HttpLambdaIntegration(
                "Token_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Token",
                    func="cyclonedx.handlers.token_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/token",
            methods=[
                apigwv2a.HttpMethod.POST,
            ],
            integration=HttpLambdaIntegration(
                "Token_HttpLambdaIntegration_POST",
                handler=lambda_factory.create(
                    lambda_name="Token_POST",
                    func="cyclonedx.handlers.token_handler",
                ).get_lambda_function(),
            ),
        )

    def __add_member_routes(
        self: "SBOMIngressApiStack",
        lambda_factory: LambdaFactory,
    ):

        self.api.add_routes(
            path="/api/v1/members",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "Members_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Members",
                    func="cyclonedx.handlers.members_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/member/{member}",
            methods=[
                apigwv2a.HttpMethod.GET,
                apigwv2a.HttpMethod.PUT,
                apigwv2a.HttpMethod.DELETE,
            ],
            integration=HttpLambdaIntegration(
                "Member_HttpLambdaIntegration",
                handler=lambda_factory.create(
                    lambda_name="Member",
                    func="cyclonedx.handlers.member_handler",
                ).get_lambda_function(),
            ),
        )

        self.api.add_routes(
            path="/api/v1/member",
            methods=[
                apigwv2a.HttpMethod.POST,
            ],
            integration=HttpLambdaIntegration(
                "Member_HttpLambdaIntegration_POST",
                handler=lambda_factory.create(
                    lambda_name="Member_POST",
                    func="cyclonedx.handlers.member_handler",
                ).get_lambda_function(),
            ),
        )

    def __add_login_route(
        self,
        user_pool_client: cognito.UserPoolClient,
        user_pool: cognito.UserPool,
        vpc: ec2.Vpc,
    ):

        """Adds the /api/v1/login endpoint for getting a JWT and logging in"""

        client_id = user_pool_client.user_pool_client_id
        user_pool_id = user_pool.user_pool_id

        self.api.add_routes(
            path="/api/v1/login",
            authorizer=HttpNoneAuthorizer(),
            methods=[apigwv2a.HttpMethod.POST],
            integration=HttpLambdaIntegration(
                "LOGIN_HttpLambdaIntegration_ID",
                handler=SBOMLoginLambda(
                    self,
                    vpc=vpc,
                    user_pool_client_id=client_id,
                    user_pool_id=user_pool_id,
                ).get_lambda_function(),
            ),
        )

    def __add_user_search_route(
        self: "SBOMIngressApiStack",
        user_pool_client: cognito.UserPoolClient,
        user_pool: cognito.UserPool,
        vpc: ec2.Vpc,
    ):

        self.api.add_routes(
            path="/api/v1/user/search",
            methods=[apigwv2a.HttpMethod.GET],
            integration=HttpLambdaIntegration(
                "USER_SEARCH_HttpLambdaIntegration_ID",
                handler=SBOMUserSearchLambda(
                    self,
                    vpc=vpc,
                    user_pool_client=user_pool_client,
                    user_pool=user_pool,
                ).get_lambda_function(),
            ),
        )

    def __add_sbom_upload_route(
        self,
        vpc: ec2.Vpc,
        dynamo_table_mgr: DynamoTableManager,
    ):

        """
        -> Adds the /api/v1/sbom route
        """

        self.api.add_routes(
            path="/api/v1/{team}/{project}/{codebase}/sbom",
            methods=[apigwv2a.HttpMethod.POST],
            authorizer=HttpLambdaAuthorizer(
                "UPLOAD_SBOM_HttpLambdaAuthorizer_ID",
                authorizer_name="UPLOAD_SBOM_HttpLambdaAuthorizer",
                handler=SBOMUploadAPIKeyAuthorizerLambda(
                    self,
                    vpc=vpc,
                    table_mgr=dynamo_table_mgr,
                ).get_lambda_function(),
            ),
            integration=HttpLambdaIntegration(
                "UPLOAD_SBOM_HttpLambdaIntegration_ID",
                handler=SbomIngressLambda(
                    self,
                    vpc=vpc,
                    s3_bucket=s3.Bucket.from_bucket_name(
                        self,
                        id=S3_BUCKET_ID,
                        bucket_name=S3_BUCKET_NAME,
                    ),
                ).get_lambda_function(),
            ),
        )

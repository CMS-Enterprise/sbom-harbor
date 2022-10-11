"""This stack deploys the Ingress Pipeline"""

from os import path


from aws_cdk import (
    aws_apigatewayv2_alpha as apigwv2a,
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    aws_cognito as cognito,
    Duration,
    Stack,
)
from aws_cdk.aws_apigatewayv2_alpha import (
    CorsHttpMethod,
    CorsPreflightOptions,
    HttpNoneAuthorizer,
)
from aws_cdk.aws_apigatewayv2_authorizers_alpha import HttpLambdaAuthorizer
from aws_cdk.aws_apigatewayv2_integrations_alpha import HttpLambdaIntegration
from constructs import Construct

from deploy.user import SBOMLoginLambda
from deploy.util import create_asset
from deploy.constants import (
    AUTHORIZATION_HEADER,
    PRIVATE,
    SBOM_API_PYTHON_RUNTIME,
)
from deploy.teams import (
    AuthorizerLambdaFactory,
)
from deploy.util import (
    DynamoTableManager,
)

INGRESS_API_STACK_ID = "SBOM-Management-Api"


class LambdaFactory:

    """
    -> LambdaFactory creates Lambda configurations
    """

    class SBOMLambda(Construct):

        """
        -> Lambda to check DynamoDB for a token
        -> belonging to the team sending an SBOM
        """

        def __init__(
            self,
            scope: Construct,
            *,
            vpc: ec2.Vpc,
            table_mgr: DynamoTableManager,
            handler: str,
            name: str,
        ):

            super().__init__(scope, name)

            self.lambda_func = lambda_.Function(
                self,
                name,
                function_name=name,
                runtime=SBOM_API_PYTHON_RUNTIME,
                vpc=vpc,
                vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
                handler=handler,
                code=create_asset(self),
                timeout=Duration.seconds(10),
                memory_size=512,
            )

            table_mgr.grant(self.lambda_func)

        def get_lambda_function(self):

            """
            -> Returns the Lambda CDK Type
            """

            return self.lambda_func

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        table_mgr: DynamoTableManager,
    ):
        self.scope = scope
        self.vpc = vpc
        self.table_mgr = table_mgr

    def create(self, lambda_name: str, func: str):

        """
        -> SBOMLambda
        """

        return LambdaFactory.SBOMLambda(
            self.scope,
            vpc=self.vpc,
            name=f"SBOMHarbor_{lambda_name}_Lambda",
            table_mgr=self.table_mgr,
            handler=func,
        )


class SBOMIngressApiStack(Stack):

    """This stack deploys the Ingress Api"""

    __cwd = path.dirname(__file__)

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
        super().__init__(scope, INGRESS_API_STACK_ID, **kwargs)

        authorizer_factory = AuthorizerLambdaFactory(
            self,
            vpc=vpc,
        )

        self.api = apigwv2a.HttpApi(
            self,
            id="SBOMManagementApi",
            description="SBOM Management API (Experimental)",
            default_authorizer=HttpLambdaAuthorizer(
                "Teams_HttpLambdaAuthorizer",
                authorizer_name="Teams_HttpLambdaAuthorizer",
                handler=authorizer_factory.create(
                    "SBOMAPIAuthorizer"
                ).get_lambda_function(),
            ),
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

        lambda_factory = LambdaFactory(
            self,
            vpc=vpc,
            table_mgr=table_mgr,
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

        self.__add_login_route(user_pool_client, user_pool, vpc)

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
                    func="cyclonedx.handlers.teams.teams_handler",
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
                    func="cyclonedx.handlers.teams.team_handler",
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
                    func="cyclonedx.handlers.projects.projects_handler",
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
                    func="cyclonedx.handlers.projects.project_handler",
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
                    func="cyclonedx.handlers.projects.project_handler",
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
                    func="cyclonedx.handlers.codebases.codebases_handler",
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
                    func="cyclonedx.handlers.codebases.codebase_handler",
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
                    func="cyclonedx.handlers.codebases.codebase_handler",
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
                    func="cyclonedx.handlers.tokens.tokens_handler",
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
                    func="cyclonedx.handlers.tokens.token_handler",
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
                    func="cyclonedx.handlers.tokens.token_handler",
                ).get_lambda_function(),
            ),
        )

    def __add_login_route(
        self,
        user_pool_client: cognito.UserPoolClient,
        user_pool: cognito.UserPool,
        vpc: ec2.Vpc,
    ):

        """Adds the /api/login endpoint for getting a JWT and logging in"""

        client_id = user_pool_client.user_pool_client_id
        user_pool_id = user_pool.user_pool_id

        self.api.add_routes(
            path="/api/login",
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

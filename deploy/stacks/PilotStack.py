"""This Stack deploys the Harbor PilotStack"""
from aws_cdk import Stack
from aws_cdk import aws_apigatewayv2_alpha as apigwv2a
from aws_cdk.aws_apigatewayv2_alpha import HttpNoneAuthorizer
from aws_cdk.aws_apigatewayv2_integrations_alpha import HttpLambdaIntegration
from constructs import Construct

from deploy.constants import PILOT_STACK_ID
from deploy.util import PilotLambda


class PilotStack(Stack):
    """This deploys the Pilot Stack"""

    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:
        # Run the constructor of the Stack superclass.
        super().__init__(scope, PILOT_STACK_ID, **kwargs)

        self.lambda_func = PilotLambda(
            self,
        )

    def get_lambda_function(self) -> PilotLambda:
        """Gets the Harbor Pilot Lambda"""

        return self.lambda_func

    def add_pilot_route(
        self,
        ingress_api: apigwv2a.HttpApi,
    ):
        """
        -> Adds the /api/v1/pilot route
        """
        pilot_api = ingress_api.from_http_api_attributes(
            self, "PilotApi", http_api_id=ingress_api.http_api_id
        )

        http_route_key = apigwv2a.HttpRouteKey.with_(
            "/api/v1/pilot", apigwv2a.HttpMethod.POST
        )

        apigwv2a.HttpRoute(
            self,
            id="PilotRoute",
            http_api=pilot_api,
            route_key=http_route_key,
            authorizer=HttpNoneAuthorizer(),
            integration=HttpLambdaIntegration(
                "PILOT_HttpLambdaIntegration_POST",
                handler=self.lambda_func.get_lambda_function(),
            ),
        )

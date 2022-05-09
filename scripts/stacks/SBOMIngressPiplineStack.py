"""This stack deploys the Ingress Pipeline"""

from os import path
from aws_cdk import (
    CfnOutput, aws_apigateway as apigwv1,
    aws_ec2 as ec2,
    aws_cognito as cognito,
    aws_lambda as lambda_,
    aws_s3 as s3,
    Stack,
)
from aws_cdk.aws_apigateway import CognitoUserPoolsAuthorizer
from constructs import Construct
from scripts.constants import API_GW_ID_EXPORT_NAME, API_GW_URL_EXPORT_ID, AUTHORIZATION_HEADER, INGRESS_STACK_ID, \
    S3_BUCKET_NAME
from scripts.constructs import PristineSbomIngressLambda, SBOMAuthorizerLambda, SBOMCreateTokenLambda, \
    SBOMDeleteTokenLambda, SBOMLoginLambda


class SBOMIngressPiplineStack(Stack):

    """This stack deploys the Ingress Pipeline"""

    __cwd = path.dirname(__file__)

    def create_asset(self):
        from_asset = lambda_.AssetCode.from_asset
        return from_asset(f"{self.__cwd}/../../dist/lambda.zip")

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        user_pool: cognito.UserPool,
        user_pool_client: cognito.UserPoolClient,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, INGRESS_STACK_ID, **kwargs)

        s3_bucket = s3.Bucket.from_bucket_name(
            self, "THIS_CONSTRUCT_ID", S3_BUCKET_NAME
        )

        pristine_lambda = PristineSbomIngressLambda(
            self,
            vpc=vpc,
            s3_bucket=s3_bucket,
        )

        api = apigwv1.RestApi(
            self, id="SBOMApi",
        )

        CfnOutput(
            self,
            API_GW_URL_EXPORT_ID,
            # TODO: get rest api url dynamically for "value" below
            value=f"{api.rest_api_id}.execute-api.us-east-1.amazonaws.com",
            export_name=API_GW_ID_EXPORT_NAME,
            description="URL Of the API Gateway",
        )

        cup_authorizer = CognitoUserPoolsAuthorizer(
            self, "CognitoUserPoolsAuthorizer_ID",
            cognito_user_pools=[user_pool],
        )

        authorizer_lambda = SBOMAuthorizerLambda(
            self, vpc=vpc,
        ).get_lambda_function()

        api.root.add_resource("ANY")
        api = api.root.add_resource("api")

        token_ep = api.add_resource("token")
        token_ep.add_method(
            "POST",
            authorizer=cup_authorizer,
            integration=apigwv1.LambdaIntegration(
                SBOMCreateTokenLambda(
                    self, vpc=vpc,
                ).get_lambda_function(),
            ),
            authorization_type=apigwv1.AuthorizationType.COGNITO,
        )

        token_ep.add_method(
            "DELETE",
            authorizer=cup_authorizer,
            integration=apigwv1.LambdaIntegration(
                SBOMDeleteTokenLambda(
                    self, vpc=vpc,
                ).get_lambda_function(),
            ),
            authorization_type=apigwv1.AuthorizationType.COGNITO,
        )

        login_ep = api.add_resource("login")
        login_ep.add_method(
            "POST",
            integration=apigwv1.LambdaIntegration(
                SBOMLoginLambda(
                    self,
                    vpc=vpc,
                    user_pool_id=user_pool.user_pool_id,
                    user_pool_client_id=user_pool_client.user_pool_client_id
                ).get_lambda_function(),
            ),
            authorization_type=apigwv1.AuthorizationType.NONE,
        )

        token_ep = api.add_resource("sbom")
        token_ep.add_method(
            "POST",
            integration=apigwv1.LambdaIntegration(
                pristine_lambda.get_lambda_function(),
            ),
            authorizer=apigwv1.RequestAuthorizer(
                self, "Request_Authorizer_ID",
                handler=authorizer_lambda,
                identity_sources=[
                    apigwv1.IdentitySource.header(AUTHORIZATION_HEADER)
                ]
            ),
            authorization_type=apigwv1.AuthorizationType.CUSTOM,
        )

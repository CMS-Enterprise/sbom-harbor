import aws_cdk as cdk
from aws_cdk import Duration as duration
from os import path, system
from aws_cdk import aws_apigateway as apigw
from aws_cdk import aws_s3 as s3
from aws_cdk import aws_lambda as lambda_
from aws_cdk import Stack
from constructs import Construct

BUCKET_NAME = "SBOMBucket"
LAMBDA_NAME = "SBOMIngest"
REST_API_NAME = "SBOMApi"


class SBOMApiDeploy(Stack):

    """
    This class is where the infrastructure to run the application
    is built.  This class inherits from the Stack class, which is part of
    the AWS CDK.
    """

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, construct_id, **kwargs)

        # Get the Current Working Directory so we can construct a path to the
        # zip file for the Lambda
        cwd = path.dirname(__file__)
        code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)

        # Create the S3 Bucket to put the BOMs in
        bucket = s3.Bucket(self, BUCKET_NAME)

        # Create the Lambda Function to do the work
        sbom_ingest_func = lambda_.Function(
            self,
            LAMBDA_NAME,
            runtime=lambda_.Runtime.PYTHON_3_9,
            handler="cyclonedx.api.store_handler",
            code=code,
            environment={"SBOM_BUCKET_NAME": bucket.bucket_name},
            timeout=duration.minutes(2),
            memory_size=512,
        )

        # Grant permissions on the Bucket to be accesed by the Lambda
        bucket.grant_put(sbom_ingest_func)

        # Create an API Gateway with CORS applied so we can hit the
        # endpoint from the command line
        lambda_api = apigw.LambdaRestApi(
            self,
            REST_API_NAME,
            default_cors_preflight_options=apigw.CorsOptions(
                allow_origins=apigw.Cors.ALL_ORIGINS,
                allow_methods=apigw.Cors.ALL_METHODS,
            ),
            # default_method_options=apigw.MethodOptions(
            #    authorization_type=apigw.AuthorizationType.NONE
            # ),
            handler=sbom_ingest_func,
            proxy=False,
        )

        # Create a POST Endpoint called 'store' to hit.
        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")


def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    app = cdk.App()
    SBOMApiDeploy(app, "SBOMApiDeploy")
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    system("cdk deploy")

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


    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        cwd = path.dirname(__file__)

        code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)

        bucket = s3.Bucket(self, BUCKET_NAME)
        sbom_ingest_func = lambda_.Function(
            self, LAMBDA_NAME,
            runtime=lambda_.Runtime.PYTHON_3_9,
            handler="cyclonedx.api.lambda_handler",
            code=code,
            environment={
                'SBOM_BUCKET_NAME': bucket.bucket_name
            },
            timeout=duration.minutes(2),
            memory_size=512)

        bucket.grant_put(sbom_ingest_func)

        lambda_api = apigw.LambdaRestApi(
            self, REST_API_NAME,
            handler=sbom_ingest_func, 
            proxy=False)

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")


def dodep() -> None:
    app = cdk.App()
    SBOMApiDeploy(app, "SBOMApiDeploy")
    app.synth()

def run() -> None:
    system("cdk deploy")
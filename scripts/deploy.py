import aws_cdk as cdk
from aws_cdk import Duration as duration
from os import path
from aws_cdk import aws_apigateway as apigw
from aws_cdk import aws_s3 as s3
from aws_cdk import aws_lambda as lambda_
from aws_cdk import Stack
from constructs import Construct


class AquiaSbomApiDeploy(Stack):

    def __init__(self, scope: Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        cwd = path.dirname(__file__)

        code = lambda_.AssetCode.from_asset("%s/../dist/lambda.zip" % cwd)

        bucket = s3.Bucket(self, "sbomBucket")
        sbom_ingest_func = lambda_.Function(
            self, "sbomIngestFunction",
            runtime=lambda_.Runtime.PYTHON_3_9,
            handler="cyclonedx.api.lambda_handler",
            code=code,
            environment={
                'SBOM_BUCKET_NAME': bucket.bucket_name
            },
            timeout=duration.minutes(2),
            memory_size=512)

        bucket.grant_put(sbom_ingest_func)

        apigw.LambdaRestApi(self, "SbomApi", handler=sbom_ingest_func, proxy=True)


def run() -> None:
    app = cdk.App()
    AquiaSbomApiDeploy(app, "AquiaSbomApiDeploy")
    app.synth()

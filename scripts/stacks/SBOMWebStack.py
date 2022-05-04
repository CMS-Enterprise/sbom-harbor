import os
import json
from aws_cdk import (
    aws_cloudfront as cf,
    aws_cognito as cognito,
    aws_iam as iam,
    aws_s3 as s3,
    aws_s3_deployment as s3d,
    Duration,
    Fn,
    RemovalPolicy,
    Stack,
)
from constructs import Construct
from scripts.constants import (
    API_GW_ID_EXPORT_NAME,
    API_GW_URL_KEY,
    CLOUDFRONT_DIST_NAME,
    S3_WS_BUCKET_ID,
    S3_WS_BUCKET_NAME,
    UI_CONFIG_FILE_NAME,
    UI_DEPLOYMENT_ID,
    WEB_STACK_ID,
)


class SBOMWebStack(Stack):

    __cwd = os.path.dirname(__file__)
    __ui_loc = f"{__cwd}/../../ui/sbom/build"

    def __create_ui_config_file(self, apigw_url):

        config_file = f"{self.__ui_loc}/{UI_CONFIG_FILE_NAME}"
        config = {
            API_GW_URL_KEY: apigw_url
        }

        if os.path.exists(config_file):
            os.remove(config_file)

        fh = open(config_file, "w")
        fh.write(json.dumps(config))
        fh.close()

    def __init__(
        self,
        scope: Construct,
        user_pool: cognito.UserPool,
        **kwargs,
    ) -> None:

        super().__init__(scope, WEB_STACK_ID, **kwargs)

        website_bucket = s3.Bucket(
            self, S3_WS_BUCKET_ID,
            bucket_name=S3_WS_BUCKET_NAME,
            public_read_access=True,
            removal_policy=RemovalPolicy.DESTROY,
            website_index_document="index.html",
            auto_delete_objects=False,
        )

        oai = cf.OriginAccessIdentity(
            self, "SBOMCFOAI",
            comment=""
        )

        website_bucket.add_to_resource_policy(iam.PolicyStatement(
            actions=["s3:GetObject"],
            resources=[f"{website_bucket.bucket_arn}/*"],
            principals=[oai.grant_principal],
        ))

        apigw_url = Fn.import_value(API_GW_ID_EXPORT_NAME)

        # self.__create_ui_config_file(apigw_url)

        # This line specifies where the UI is as an asset.
        # We need to have written whatever we needed already to the
        # UI build folder before this line runs.
        sources = s3d.Source.asset(self.__ui_loc)

        s3d.BucketDeployment(
            self, UI_DEPLOYMENT_ID,
            sources=[sources],
            destination_bucket=website_bucket,
        )

        cf.CloudFrontWebDistribution(
            self, CLOUDFRONT_DIST_NAME,
            origin_configs=[
                cf.SourceConfiguration(
                    custom_origin_source=cf.CustomOriginConfig(
                        # TODO: get rest api url dynamically for "value" below
                        # see: scripts/constructs/__init__.py/PristineSbomIngressLambda
                        domain_name=apigw_url,
                    ),
                    behaviors=[
                        cf.Behavior(
                            path_pattern="/api/*",
                            allowed_methods=cf.CloudFrontAllowedMethods.ALL,
                            default_ttl=Duration.seconds(5),
                            forwarded_values=cf.CfnDistribution.ForwardedValuesProperty(
                                headers=["Authorization"],
                                query_string=True,
                            ),
                        ),
                    ],
                ),
                cf.SourceConfiguration(
                    s3_origin_source=cf.S3OriginConfig(
                        origin_access_identity=oai,
                        s3_bucket_source=website_bucket,
                    ),
                    behaviors=[
                        cf.Behavior(
                            path_pattern="/*",
                            is_default_behavior=True,
                            allowed_methods=cf.CloudFrontAllowedMethods.ALL,
                            compress=False,
                            default_ttl=Duration.minutes(30),
                            forwarded_values=cf.CfnDistribution.ForwardedValuesProperty(
                                cookies=cf.CfnDistribution.CookiesProperty(
                                    forward="all",
                                ),
                                headers=["Authorization"],
                                query_string=True,
                            ),
                        ),
                    ],
                ),
            ]
        )

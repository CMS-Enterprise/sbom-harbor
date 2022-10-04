import os

import constructs
from aws_cdk import (
    aws_cloudfront as cf,
    aws_iam as iam,
    aws_s3 as s3,
    aws_s3_deployment as s3d,
    Duration,
    Fn,
    RemovalPolicy,
    Stack,
)
from constructs import Construct
from deploy.constants import (
    API_GW_ID_EXPORT_NAME,
    AUTHORIZATION_HEADER,
    CLOUDFRONT_DIST_NAME,
    S3_WS_BUCKET_ID,
    S3_WS_BUCKET_NAME,
    UI_DEPLOYMENT_ID,
    VPC_NAME,
    WEB_STACK_ID,
)
from deploy.util.SBOMHarborCertificate import Cert as SBOMHarborCert

class SBOMWebStack(Stack):

    __cwd = os.path.dirname(__file__)
    # TODO: make the UI build output path configurable
    __ui_loc = f"{__cwd}/../../ui/packages/sbom/build"

    def __get_cloudfront_distribution(
            self: constructs.Construct,
            apigw_url: str,
            harbor_cert: SBOMHarborCert,
            oai: cf.OriginAccessIdentity,
            website_bucket: s3.Bucket
    ):

        kw_args = {}
        if harbor_cert.enabled:
            kw_args["viewer_certificate"] = harbor_cert.get_viewer_cert()

        return cf.CloudFrontWebDistribution(
            self, CLOUDFRONT_DIST_NAME,
            **kw_args,
            origin_configs=[
                cf.SourceConfiguration(
                    custom_origin_source=cf.CustomOriginConfig(
                        domain_name=apigw_url,
                        origin_path=""
                    ),
                    behaviors=[
                        cf.Behavior(
                            path_pattern="/api/*",
                            allowed_methods=cf.CloudFrontAllowedMethods.ALL,
                            default_ttl=Duration.seconds(5),
                            forwarded_values=cf.CfnDistribution.ForwardedValuesProperty(
                                headers=[AUTHORIZATION_HEADER],
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
                                headers=[AUTHORIZATION_HEADER],
                                query_string=True,
                            ),
                        ),
                    ],
                ),
            ]
        )

    def __get_logging_configuration(self):
        return cf.LoggingConfiguration(
            bucket=s3.Bucket(
                self, "cloudfront.logging.bucket",
                bucket_name="sbom.harbor.cf",
                public_read_access=False,
                auto_delete_objects=True,
                removal_policy=RemovalPolicy.DESTROY,
            ),
            include_cookies=False,
            prefix="prefix"
        )

    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:

        super().__init__(scope, WEB_STACK_ID, **kwargs)

        website_bucket = s3.Bucket(
            self, S3_WS_BUCKET_ID,
            bucket_name=S3_WS_BUCKET_NAME,
            public_read_access=False,
            auto_delete_objects=True,
            removal_policy=RemovalPolicy.DESTROY,
            website_index_document="index.html",
        )

        oai = cf.OriginAccessIdentity(
            self, "SBOMHarborOriginAccessIdentity",
            comment="SBOM Origin Access Identity"
        )

        website_bucket.add_to_resource_policy(
            iam.PolicyStatement(
                actions=["s3:GetObject"],
                resources=[f"{website_bucket.bucket_arn}/*"],
                principals=[oai.grant_principal],
            )
        )

        # This line specifies where the UI is as an asset.
        # We need to have written whatever we needed already to the
        # UI build folder before this line runs.
        sources = s3d.Source.asset(self.__ui_loc)

        s3d.BucketDeployment(
            self, UI_DEPLOYMENT_ID,
            sources=[sources],
            destination_bucket=website_bucket,
        )

        # Create a Certificate to read the environment
        # and decide if we should be deploying the harbor with
        # a public domain.
        harbor_cert = SBOMHarborCert(self)

        distribution = self.__get_cloudfront_distribution(
            website_bucket=website_bucket, oai=oai,
            apigw_url=Fn.import_value(API_GW_ID_EXPORT_NAME),
            harbor_cert=harbor_cert,
        )

        harbor_cert.create_host_record(distribution)

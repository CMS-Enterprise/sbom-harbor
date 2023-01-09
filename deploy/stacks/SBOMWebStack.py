"""This Stack deploys the Web Stack Pipeline"""

import os

from aws_cdk import CfnOutput, Duration, Fn, RemovalPolicy, Stack
from aws_cdk import aws_cloudfront as cf
from aws_cdk import aws_iam as iam
from aws_cdk import aws_s3 as s3
from constructs import Construct

from deploy.constants import (
    API_GW_URL_EXPORT_NAME,
    AUTHORIZATION_HEADER,
    CLOUDFRONT_DIST_ID,
    S3_WS_BUCKET_ID,
    S3_WS_BUCKET_NAME,
    WEB_STACK_ID,
)
from deploy.util.SBOMHarborCertificate import Cert as SBOMHarborCert


class SBOMWebStack(Stack):
    """This Stack deploys the Web Stack Pipeline"""

    __cwd = os.path.dirname(__file__)
    # TODO: make the UI build output path configurable
    __ui_loc = f"{__cwd}/../../ui/packages/sbom/build"

    def __get_logging_configuration(self):
        return cf.LoggingConfiguration(
            bucket=s3.Bucket(
                self,
                "cloudfront.logging.bucket",
                bucket_name="sbom.harbor.cf",
                public_read_access=False,
                removal_policy=RemovalPolicy.DESTROY,
                block_public_access=s3.BlockPublicAccess.BLOCK_ALL,
            ),
            include_cookies=False,
            prefix="prefix",
        )

    def __init__(
        self,
        scope: Construct,
        **kwargs,
    ) -> None:

        super().__init__(scope, WEB_STACK_ID, **kwargs)

        website_bucket = s3.Bucket(
            self,
            S3_WS_BUCKET_ID,
            bucket_name=S3_WS_BUCKET_NAME,
            public_read_access=False,
            block_public_access=s3.BlockPublicAccess.BLOCK_ALL,
            removal_policy=RemovalPolicy.DESTROY,
            website_index_document="index.html",
        )

        oai = cf.OriginAccessIdentity(
            self,
            "WebsiteOriginAccessIdentity",
            comment="Website Origin Access Identity",
        )

        website_bucket.add_to_resource_policy(
            iam.PolicyStatement(
                actions=["s3:GetObject"],
                resources=[f"{website_bucket.bucket_arn}/*"],
                principals=[oai.grant_principal],
            )
        )

        # Create a Certificate to read the environment
        # and decide if we should be deploying the harbor with
        # a public domain.
        harbor_cert = SBOMHarborCert(self)

        distribution = cf.CloudFrontWebDistribution(
            self,
            CLOUDFRONT_DIST_ID,
            viewer_certificate=harbor_cert.get_viewer_cert()
            if harbor_cert.enabled
            else None,
            origin_configs=[
                cf.SourceConfiguration(
                    custom_origin_source=cf.CustomOriginConfig(
                        domain_name=Fn.import_value(API_GW_URL_EXPORT_NAME),
                        origin_path="",
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
            ],
        )

        harbor_cert.create_host_record(distribution)

        # referenced by UI deployment script
        CfnOutput(
            self,
            "WebAssetsBucketName",
            description="Source for CloudFront to serve frontend UI assets",
            value=website_bucket.bucket_name,
        )

        # referenced by UI deployment script
        CfnOutput(
            self,
            "CloudFrontDomain",
            description="CloudFront Distribution domain name",
            value=distribution.distribution_domain_name,
        )

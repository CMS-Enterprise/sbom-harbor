"""This stack deploys the Ingress Pipeline"""
import os
import aws_cdk as cdk
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
import aws_cdk.aws_cloudfront as cf
from aws_cdk import Stack
from constructs import Construct
import aws_cdk.aws_cognito as cognito
from scripts.constants import (
    API_GW_URL_EXPORT_ID,
    INGRESS_STACK_ID,
    API_GW_ID_EXPORT_NAME,
)

from scripts.constructs import PristineSbomIngressLambda


class SBOMIngressPiplineStack(Stack):

    """This stack deploys the Ingress Pipeline"""

    __cwd = os.path.dirname(__file__)

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        user_pool: cognito.UserPool,
        s3_bucket: s3.Bucket,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, INGRESS_STACK_ID, **kwargs)

        PristineSbomIngressLambda(
            self,
            vpc=vpc,
            code=lambda_.AssetCode.from_asset("%s/../../dist/lambda.zip" % self.__cwd),
            s3_bucket=s3_bucket,
            user_pool=user_pool,
        )


"""This stack deploys the Ingress Pipeline"""

from os import path
from aws_cdk import (
    aws_cognito as cognito,
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    aws_s3 as s3,
    Stack,
)
from constructs import Construct
from scripts.constants import INGRESS_STACK_ID
from scripts.constructs import PristineSbomIngressLambda


class SBOMIngressPiplineStack(Stack):

    """This stack deploys the Ingress Pipeline"""

    __cwd = path.dirname(__file__)

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

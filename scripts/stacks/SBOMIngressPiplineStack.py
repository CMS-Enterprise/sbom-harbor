"""This stack deploys the Ingress Pipeline"""

from os import path

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
import aws_cdk.aws_s3 as s3
from aws_cdk import Stack
from constructs import Construct

from scripts.constants import INGRESS_STACK_ID

from scripts.constructs import PristineSbomIngressLambda

cwd = path.dirname(__file__)
ingress_code = lambda_.AssetCode.from_asset("%s/../../dist/lambda.zip" % cwd)


class SBOMIngressPiplineStack(Stack):

    """This stack deploys the Ingress Pipeline"""

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        s3_bucket: s3.Bucket,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, INGRESS_STACK_ID, **kwargs)

        PristineSbomIngressLambda(
            self,
            vpc=vpc,
            code=ingress_code,
            s3_bucket=s3_bucket,
        )

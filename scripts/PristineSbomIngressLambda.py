import aws_cdk.aws_apigateway as apigwv1
import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_
from aws_cdk import Duration
from aws_cdk.aws_lambda import AssetCode
from aws_cdk.aws_s3 import Bucket
from constructs import Construct
from cyclonedx.constants import SBOM_BUCKET_NAME_EV
from scripts.constants import (
    PRISTINE_SBOM_INGRESS_API_ID,
    PRISTINE_SBOM_INGRESS_LN,
    PRIVATE,
)


class PristineSbomIngressLambda(Construct):

    def __init__(self, scope: Construct, *, vpc: ec2.Vpc,
                 code: AssetCode, s3_bucket: Bucket,):

        super().__init__(scope, PRISTINE_SBOM_INGRESS_LN)

        sbom_ingest_func = lambda_.Function(
            self,
            PRISTINE_SBOM_INGRESS_LN,
            runtime=lambda_.Runtime.PYTHON_3_9,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.api.pristine_sbom_ingress_handler",
            code=code,
            environment={
                SBOM_BUCKET_NAME_EV: s3_bucket.bucket_name
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        s3_bucket.grant_put(sbom_ingest_func)

        lambda_api = apigwv1.LambdaRestApi(
            self,
            id=PRISTINE_SBOM_INGRESS_API_ID,
            handler=sbom_ingest_func,
        )

        store_ep = lambda_api.root.add_resource("store")
        store_ep.add_method("POST")

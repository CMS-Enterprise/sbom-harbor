from aws_cdk import (
    aws_ec2 as ec2,
    aws_lambda as lambda_,
    aws_s3 as i_bucket,
    aws_iam as iam,
    Duration,
)
from constructs import Construct

from cyclonedx.constants import (
    SBOM_BUCKET_NAME_KEY
)
from deploy.constants import (
    SBOM_INGRESS_LN,
    SBOM_API_PYTHON_RUNTIME,
    PRIVATE
)
from deploy.util import create_asset


class SbomIngressLambda(Construct):
    
    """Constructs a Lambda that can take
    SBOMS and puts them in the S3 Bucket"""

    def __init__(
        self,
        scope: Construct,
        *,
        vpc: ec2.Vpc,
        s3_bucket: i_bucket,
    ):

        super().__init__(scope, SBOM_INGRESS_LN)

        self.sbom_ingress_func = lambda_.Function(
            self,
            SBOM_INGRESS_LN,
            function_name="SbomIngressLambda",
            runtime=SBOM_API_PYTHON_RUNTIME,
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=PRIVATE),
            handler="cyclonedx.handlers.ingress.sbom_ingress_handler",
            code=create_asset(self),
            environment={
                SBOM_BUCKET_NAME_KEY: s3_bucket.bucket_name,
            },
            timeout=Duration.seconds(10),
            memory_size=512,
        )

        s3_bucket.grant_put(self.sbom_ingress_func)

        self.sbom_ingress_func.grant_invoke(
            iam.ServicePrincipal('apigateway.amazonaws.com'),
        )

    def get_lambda_function(self):
        return self.sbom_ingress_func

"""This Stack deploys the SBOMGeneratorPipelineStack"""
from aws_cdk import Stack
from aws_cdk import aws_ec2 as ec2
from constructs import Construct

from deploy.constants import SBOM_GENERATOR_STACK_ID
from deploy.util import SBOMGeneratorLambda


class SBOMGeneratorPipelineStack(Stack):

    """This Stack deploys the SBOM Generator Pipeline"""

    def __init__(
        self,
        scope: Construct,
        vpc: ec2.Vpc,
        **kwargs,
    ) -> None:

        # Run the constructor of the Stack superclass.
        super().__init__(scope, SBOM_GENERATOR_STACK_ID, **kwargs)

        self.sbom_generator_lambda = SBOMGeneratorLambda(
            self,
            vpc=vpc,
        )

    def get_sbom_generator(self) -> SBOMGeneratorLambda:

        """Gets the SBOM Generator Lambda"""

        return self.sbom_generator_lambda

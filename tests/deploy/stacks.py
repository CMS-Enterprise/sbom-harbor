"""
-> Module for the code that tests cdk stacks.
"""
import aws_cdk as cdk

from deploy.constants import AWS_ACCOUNT_ID, AWS_REGION
from deploy.stacks import SBOMGeneratorPipelineStack, SBOMSharedResourceStack


def test_sbom_generator_pipeline():

    """
    -> Test the SBOMGeneratorPipelineStack will work.
    """

    env = cdk.Environment(
        account=AWS_ACCOUNT_ID,
        region=AWS_REGION,
    )

    app = cdk.App()

    shared_resources = SBOMSharedResourceStack(app, env=env)
    vpc = shared_resources.get_vpc()

    SBOMGeneratorPipelineStack(
        app,
        vpc,
        env=env,
    )

    app.synth()

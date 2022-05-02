""" This module is the start of the deployment for SBOM-API """

from os import system, getenv
import aws_cdk as cdk

from scripts.stacks import (
    SBOMEnrichmentPiplineStack,
    SBOMIngressPiplineStack,
    SBOMSharedResourceStack,
)
from scripts.stacks.SBOMWebStack import SBOMWebStack


def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    env = cdk.Environment(
        region=getenv("AWS_REGION"),
        account=getenv("AWS_ACCOUNT_NUM"),
    )

    app = cdk.App()
    shared_resources = SBOMSharedResourceStack(app, env=env)
    vpc = shared_resources.get_vpc()
    user_pool = shared_resources.get_user_pool()
    s3_bucket = shared_resources.get_s3_bucket()

    SBOMIngressPiplineStack(app, vpc, user_pool, s3_bucket, env=env)
    SBOMEnrichmentPiplineStack(app, vpc, env=env)
    SBOMWebStack(app, user_pool)
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    no_req_approval = "--require-approval never"

    system(f"cdk deploy --all {no_req_approval}")

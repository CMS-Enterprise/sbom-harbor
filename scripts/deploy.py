""" This module is the start of the deployment for SBOM-API """

import aws_cdk as cdk
from os import system, getenv
from scripts.stacks import (
    SBOMEnrichmentPiplineStack,
    SBOMIngressPiplineStack,
    SBOMSharedResourceStack,
    SBOMWebStack,
)

def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    default_region = getenv("AWS_DEFAULT_REGION", "us-east-1")

    env = cdk.Environment(
        account=getenv("AWS_ACCOUNT_NUM"),
        region=getenv("AWS_REGION", default_region),
    )

    app = cdk.App()
    shared_resources = SBOMSharedResourceStack(app, env=env)
    vpc = shared_resources.get_vpc()
    user_pool = shared_resources.get_user_pool()
    s3_bucket = shared_resources.get_s3_bucket()

    ingress_stack = SBOMIngressPiplineStack(app, vpc, user_pool, s3_bucket, env=env)

    SBOMEnrichmentPiplineStack(app, vpc, env=env)

    web_stack = SBOMWebStack(app, user_pool)
    web_stack.add_dependency(ingress_stack)

    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    no_req_approval = "--require-approval never"

    system(f"cdk deploy --all {no_req_approval}")

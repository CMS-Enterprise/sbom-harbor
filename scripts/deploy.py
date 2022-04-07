""" This module is the start of the deployment for SBOM-API """

from os import system, getenv

import aws_cdk as cdk

from scripts.stacks import (
    SBOMEnrichmentPiplineStack,
    SBOMIngressPiplineStack,
    SBOMSharedResourceStack,
)

from scripts.constants import (
    ENRICHMENT_STACK_ID,
    INGRESS_STACK_ID,
    SHARED_RESOURCE_STACK_ID,
)


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
    SBOMSharedResourceStack(
        app,
        SHARED_RESOURCE_STACK_ID,
        env=env,
    )
    SBOMIngressPiplineStack(
        app,
        INGRESS_STACK_ID,
        env=env,
    )
    SBOMEnrichmentPiplineStack(
        app,
        ENRICHMENT_STACK_ID,
        env=env,
    )
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    system("cdk deploy Shared-Resource-SBOMApiStack")
    system("cdk deploy Ingress-SBOMApiStack")
    system("cdk deploy Enrichment-SBOMApiStack")

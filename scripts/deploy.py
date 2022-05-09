""" This module is the start of the deployment for SBOM-API """

import aws_cdk as cdk
from aws_cdk import (
    aws_iam as iam,
    aws_cognito as cognito,
)

from os import system, getenv
from scripts.stacks import (
    SBOMEnrichmentPiplineStack,
    SBOMIngressPiplineStack,
    SBOMSharedResourceStack,
    SBOMUserManagement,
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

    # Create the CDK app to pass into all the Stacks
    app = cdk.App()

    # Create Shared Resources Stack to create the services used
    # by all the other stacks.
    shared_resources = SBOMSharedResourceStack(app, env=env)
    vpc = shared_resources.get_vpc()

    user_management = SBOMUserManagement(app, vpc=vpc, env=env)
    user_pool: cognito.UserPool = user_management.get_user_pool()
    user_pool_client: cognito.UserPoolClient = user_management.get_user_pool_client()

    # The Ingress stack set up the infrastructure to handle incoming SBOMs
    ingress_stack = SBOMIngressPiplineStack(
        app, vpc, env=env,
        user_pool=user_pool,
        user_pool_client=user_pool_client
    )

    # The Enrichment Stack sets up the infrastructure to enrich SBOMs
    SBOMEnrichmentPiplineStack(app, vpc, env=env)

    # The Web Stack has all the web oriented entities to manage the website
    web_stack = SBOMWebStack(app, user_pool)
    web_stack.add_dependency(ingress_stack)

    # Synth the CDK app
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    no_req_approval = "--require-approval never"

    system(f"cdk deploy --all {no_req_approval}")

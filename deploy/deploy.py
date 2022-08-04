""" This module is the start of the deployment for SBOM-API """
import os
from os import system

import aws_cdk as cdk
from aws_cdk import (
    aws_cognito as cognito,
    aws_events as eventbridge,
)

from deploy.constants import CONFIG
from deploy.stacks import (
    SBOMEnrichmentPiplineStack,
    SBOMIngressPiplineStack,
    SBOMSharedResourceStack,
    SBOMUserManagement,
    SBOMWebStack,
)
from deploy.util import DynamoTableManager


def dodep() -> None:

    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    default_region = CONFIG["AWS_DEFAULT_REGION"]
    region = CONFIG["AWS_REGION"]
    account = os.environ.get("AWS_ACCOUNT_NUM")

    env = cdk.Environment(
        account=account,
        region = region if region is not None else default_region,
    )

    # Create the CDK app to pass into all the Stacks
    app = cdk.App()

    # Create Shared Resources Stack to create the services used
    # by all the other stacks.
    shared_resources = SBOMSharedResourceStack(app, env=env)
    vpc = shared_resources.get_vpc()
    table_manager: DynamoTableManager = shared_resources.get_dynamo_table_manager()
    event_bus: eventbridge.EventBus = shared_resources.get_event_bus()

    user_management = SBOMUserManagement(app, vpc=vpc, env=env)
    user_pool: cognito.UserPool = user_management.get_user_pool()
    user_pool_client: cognito.UserPoolClient = user_management.get_user_pool_client()

    # The Ingress stack set up the infrastructure to handle incoming SBOMs
    ingress_stack = SBOMIngressPiplineStack(
        app, vpc, env=env,
        user_pool=user_pool,
        user_pool_client=user_pool_client,
        table_mgr=table_manager,
    )

    # The Enrichment Stack sets up the infrastructure to enrich SBOMs
    SBOMEnrichmentPiplineStack(
        app, vpc,
        env=env,
        event_bus=event_bus,
    )

    # The Web Stack has all the web oriented entities to manage the website
    web_stack = SBOMWebStack(app, env=env)
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


def setup_admin_user() -> None:

    """
    This method creates the users table in the database.
    """

    system("./tests/scripts/create.cognito.user.sh")
    system("./tests/scripts/change.cognito.user.password.sh")

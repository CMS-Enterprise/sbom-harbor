""" This module is the start of the deployment for SBOM-API """
from os import system

import aws_cdk as cdk
from aws_cdk import aws_cognito as cognito
from aws_cdk import aws_events as eventbridge

from deploy.constants import AWS_ACCOUNT_ID, AWS_REGION, ENVIRONMENT
from deploy.stacks import (
    SBOMEnrichmentPiplineStack,
    SBOMGeneratorPipelineStack,
    SBOMSharedResourceStack,
    SBOMUserManagement,
    SBOMWebStack,
    HarborDevOpsStack,
)
from deploy.stacks.SBOMIngressApiStack import SBOMIngressApiStack
from deploy.util import DynamoTableManager
from tests.data.create_cognito_users import test_create_cognito_users


def dodep() -> None:
    """
    This is a build handler used by Poetry to
    construct the resources necessary to run the app.
    """

    env = cdk.Environment(
        account=AWS_ACCOUNT_ID,
        region=AWS_REGION,
    )

    # Create the CDK app to pass into all the Stacks
    app = cdk.App()

    if ENVIRONMENT == "cicd":
        HarborDevOpsStack(app, env=env)
    else:
        # Create Shared Resources Stack to create the services used
        # by all the other stacks.
        shared_resources = SBOMSharedResourceStack(app, env=env)
        vpc = shared_resources.get_vpc()
        table_manager: DynamoTableManager = shared_resources.get_dynamo_table_manager()
        event_bus: eventbridge.EventBus = shared_resources.get_event_bus()

        user_management = SBOMUserManagement(app, env=env)
        user_pool: cognito.UserPool = user_management.get_user_pool()
        user_pool_client: cognito.UserPoolClient = (
            user_management.get_user_pool_client()
        )

        # The Ingress stack set up the infrastructure to handle incoming SBOMs
        ingress_api_stack = SBOMIngressApiStack(
            app,
            vpc,
            env=env,
            table_mgr=table_manager,
            user_pool=user_pool,
            user_pool_client=user_pool_client,
        )

        # The Enrichment Stack sets up the infrastructure to enrich SBOMs
        SBOMEnrichmentPiplineStack(
            app,
            vpc,
            env=env,
            event_bus=event_bus,
        )

        # The Web Stack has all the web oriented entities to manage the website
        web_stack = SBOMWebStack(app, env=env)
        web_stack.add_dependency(ingress_api_stack)

        # The SBOM Generator Pipeline stack has the lambda to
        # generate SBOMs by crawling GitHub repositories.
        SBOMGeneratorPipelineStack(
            app,
            vpc,
            env=env,
        )

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

    test_create_cognito_users()

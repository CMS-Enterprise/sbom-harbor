""" This module is the start of the deployment for SBOM-API """
import logging
from logging import config
from os import system

import aws_cdk as cdk
from aws_cdk import aws_cognito as cognito
from aws_cdk import aws_events as eventbridge

from deploy.constants import (
    AWS_ACCOUNT_ID,
    AWS_REGION,
    CMS_PERMISSION_BOUNDARY_ARN,
    CMS_ROLE_PATH,
    ENVIRONMENT,
    PYTHON_LOGGING_CONFIG,
)
from deploy.role_aspect import RoleAspect
from deploy.stacks import (
    HarborDevOpsStack,
    PilotStack,
    SBOMEnrichmentPiplineStack,
    SBOMSharedResourceStack,
    SBOMUserManagement,
    SBOMWebStack,
)
from deploy.stacks.SBOMIngressApiStack import SBOMIngressApiStack
from deploy.util import DynamoTableManager

config.fileConfig(PYTHON_LOGGING_CONFIG)
logger = logging.getLogger(__name__)


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
        stacks = []
        # Create Shared Resources Stack to create the services used
        # by all the other stacks.
        shared_resources = SBOMSharedResourceStack(app, env=env)
        stacks.append(shared_resources)
        vpc = shared_resources.get_vpc()
        table_manager: DynamoTableManager = shared_resources.get_dynamo_table_manager()
        event_bus: eventbridge.EventBus = shared_resources.get_event_bus()

        user_management = SBOMUserManagement(app, env=env)
        stacks.append(user_management)
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
        stacks.append(ingress_api_stack)

        # The Enrichment Stack sets up the infrastructure to enrich SBOMs
        sbom_enrichment_pipeline_stack = SBOMEnrichmentPiplineStack(
            app,
            vpc,
            env=env,
            table_mgr=table_manager,
            event_bus=event_bus,
        )
        stacks.append(sbom_enrichment_pipeline_stack)

        # The Web Stack has all the web oriented entities to manage the website
        web_stack = SBOMWebStack(app, env=env)
        stacks.append(web_stack)
        web_stack.add_dependency(ingress_api_stack)

        # Pilot stack includes Rust-based ci endpoint
        pilot_stack = PilotStack(
            app,
            env=env,
        )
        stacks.append(pilot_stack)
        pilot_stack.add_dependency(ingress_api_stack)
        pilot_stack.add_pilot_route(ingress_api_stack.api)

        for stack in stacks:
            cdk.Aspects.of(stack).add(
                RoleAspect(
                    permission_boundary_arn=CMS_PERMISSION_BOUNDARY_ARN,
                    path=CMS_ROLE_PATH,
                )
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

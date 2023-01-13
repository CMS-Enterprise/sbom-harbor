"""
-> Module for the code that tests cdk stacks.
"""
import aws_cdk as cdk
from aws_cdk import aws_cognito as cognito

from deploy.constants import AWS_ACCOUNT_ID, DEPLOYMENT_AWS_REGION
from deploy.stacks import (
    PilotStack,
    SBOMGeneratorPipelineStack,
    SBOMSharedResourceStack,
    SBOMUserManagement,
)
from deploy.stacks.SBOMIngressApiStack import SBOMIngressApiStack
from deploy.util import DynamoTableManager


def test_pilot_stack():

    """
    -> Test the PilotStack will build.
    """

    env = cdk.Environment(
        account=AWS_ACCOUNT_ID,
        region=DEPLOYMENT_AWS_REGION,
    )

    app = cdk.App()

    # Create Shared Resources Stack to create the services used
    # by all the other stacks.
    shared_resources = SBOMSharedResourceStack(app, env=env)

    vpc = shared_resources.get_vpc()
    table_manager: DynamoTableManager = shared_resources.get_dynamo_table_manager()

    user_management = SBOMUserManagement(app, env=env)
    user_pool: cognito.UserPool = user_management.get_user_pool()
    user_pool_client: cognito.UserPoolClient = user_management.get_user_pool_client()

    # The Ingress stack set up the infrastructure to handle incoming SBOMs
    ingress_api_stack = SBOMIngressApiStack(
        app,
        vpc,
        env=env,
        table_mgr=table_manager,
        user_pool=user_pool,
        user_pool_client=user_pool_client,
    )

    pilot_stack = PilotStack(
        app,
    )

    pilot_stack.add_dependency(ingress_api_stack)
    pilot_stack.add_pilot_route(ingress_api_stack.api)

    app.synth()


def test_sbom_generator_pipeline():

    """
    -> Test the SBOMGeneratorPipelineStack will build.
    """

    env = cdk.Environment(
        account=AWS_ACCOUNT_ID,
        region=DEPLOYMENT_AWS_REGION,
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

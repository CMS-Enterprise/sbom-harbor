""" This module is the start of the deployment for SBOM-API """

from os import system, getenv

import aws_cdk as cdk

from scripts.SBOMApiStack import SBOMApiStack
from scripts.constants import STACK_ID


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
    SBOMApiStack(
        app,
        STACK_ID,
        env=env,
    )
    app.synth()


def run() -> None:

    """
    Starts the process of deploying.
    To Run: poetry run deploy
    """

    system("cdk deploy")

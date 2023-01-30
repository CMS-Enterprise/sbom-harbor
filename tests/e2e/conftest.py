import subprocess
from os import getenv

import boto3
import pytest


@pytest.fixture(name="session", autouse=True)
def get_boto_session():

    """
    -> Get a Boto Session to the CMS AWS Dev Account
    """
    return boto3.Session(profile_name=getenv("AWS_PROFILE", "default"))


@pytest.fixture(name="environment", autouse=True)
def get_current_environment():

    """
    -> Get Environment of current branch
    """
    process = subprocess.run(
        ["git", "rev-parse", "--abbrev-ref", "HEAD"],
        capture_output=True,
        check=False,
    )
    results = process.stdout.decode().strip().split("/")[0].split("-")[1]

    return f"e{results}"

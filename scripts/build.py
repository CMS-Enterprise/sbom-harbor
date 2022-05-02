""" This Module has functions that can be run using Poetry to perform build tasks"""

from asyncio.log import logger
from shutil import rmtree
from inspect import stack
from optparse import OptionParser
from os import system
from sys import path

parser = OptionParser("usage: %prog [options]")
parser.add_option('--ui', dest="ui", help='ui flag', action="store_true")

def install():

    """
    Use Poetry to install dependencies.
    """

    system("poetry install")
    run_ui_if_enabled()


def install_ui():

    """
    Installs dependencies for the UI
    """

    system("yarn prepare")


def run():

    """
    Uses Poetry to build the code.
    """

    system("poetry build")
    run_ui_if_enabled()


def run_ui():

    """
    Uses Webpack to build the UI code.
    """

    system("yarn build")


def lint():

    """
    Lint the code with Pylint
    """

    system("pylint cyclonedx/")
    run_ui_if_enabled()


def lint_ui():

    """
    Lint the UI code with ESLint
    """

    system("yarn lint:js")


def test():

    """
    Run Pytest and get coverage
    """

    path.insert(0, "cyclonedx/")
    system("poetry run python -m pytest -v --cov=cyclonedx/ tests/")
    run_ui_if_enabled()


def test_ui():

    """
    Run Jest tests
    """

    system("yarn test")


def package():

    """
    This method uses poetry to package the application and
    all of its dependencies into a zip file that can be used by
    AWS Lambda to urn the application.
    """

    system("poetry run pip install --upgrade -t tmp dist/*.whl")
    system("cd tmp; zip -r ../dist/lambda.zip . -x '*.pyc'")


def clean():

    """
    Removes unwanted junk from the project.  Maybe we should read
    .gitignore...?
    """

    for unwanted_dir in ["dist", "tmp", "cdk.out"]:
        try:
            rmtree(unwanted_dir)
        except OSError as os_error:
            print("Error: %s : %s" % (unwanted_dir, os_error.strerror))
    run_ui_if_enabled()


def clean_ui():

    """
    Removes build artifacts and temporary files from the UI

    """

    system("yarn clean")


def run_ui_if_enabled():

    """
    This function is used to run the same command for the
    UI if the --ui flag is passed when calling the script.
    Usage:
        poetry run <install,run,lint,test> --ui
    """

    (options, args) = parser.parse_args()
    caller = stack()[1][3]

    if options.ui:
        method = caller + "_ui"
        print("\nRunning " + caller + " for UI...\n")
        globals()[method]()

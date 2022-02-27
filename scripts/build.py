from shutil import rmtree
from os import system


def package():
    system("poetry run pip install --upgrade -t tmp dist/*.whl")
    system("cd tmp; zip -r ../dist/lambda.zip . -x '*.pyc'")


def clean():

    try:
        rmtree("dist")
    except OSError as e:
        print("Error: %s : %s" % ("dist", e.strerror))

    try:
        rmtree("tmp")
    except OSError as e:
        print("Error: %s : %s" % ("tmp", e.strerror))



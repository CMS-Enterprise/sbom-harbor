from shutil import rmtree
from os import system

def run():
    system("poetry build")


def package():
    system("poetry run pip install --upgrade -t tmp dist/*.whl")
    system("cd tmp; zip -r ../dist/lambda.zip . -x '*.pyc'")


def clean():
    for dir in ["dist", "tmp", "cdk.out"]:    
        try:
            rmtree(dir)
        except OSError as e:
            print("Error: %s : %s" % (dir, e.strerror))

    


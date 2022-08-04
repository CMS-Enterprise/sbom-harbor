from os import path

from aws_cdk import (
    aws_lambda as lambda_
)


def create_asset(self):
    __cwd = path.dirname(__file__)

    from_asset = lambda_.AssetCode.from_asset
    return from_asset(f"{__cwd}/../../dist/lambda.zip")
    
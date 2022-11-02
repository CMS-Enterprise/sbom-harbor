"""
-> Module to house functions to create cognito users
-> for test.
"""
import boto3
from botocore.exceptions import ClientError

from cyclonedx.ciam import CognitoUserData
from tests.conftest import FINAL_TEST_PASSWORD

client = boto3.client("cognito-idp")


def test_create_cognito_users():

    """
    -> Test to generate cognito users in the AWS account
    """

    usernames = [
        "sbomadmin",
        "quinn",
        "quinton",
        "quison",
        "heather",
        "bill",
        "martha",
        "fred",
        "maria",
        "sam",
        "linda",
    ]

    response = client.list_user_pools(MaxResults=5)

    user_pool = response["UserPools"][0]
    user_pool_id: str = user_pool["Id"]

    print(user_pool)

    for username in usernames:

        email_username = f"{username}@aquia.io"

        teams = "dawn-patrol,dusk-patrol" if username == "sbomadmin" else "dawn-patrol"

        try:
            client.admin_delete_user(
                UserPoolId=user_pool_id,
                Username=email_username,
            )
        except ClientError:
            ...

        client.admin_create_user(
            UserPoolId=user_pool_id,
            Username=email_username,
            UserAttributes=[
                {
                    "Name": CognitoUserData.Attrib.EMAIL,
                    "Value": email_username,
                },
                {
                    "Name": CognitoUserData.Attrib.TEAMS,
                    "Value": teams,
                },
            ],
            TemporaryPassword="AbC123P@55!",
            ForceAliasCreation=True,
            MessageAction="SUPPRESS",
            DesiredDeliveryMediums=["EMAIL"],
        )

        client.admin_set_user_password(
            UserPoolId=user_pool_id,
            Username=email_username,
            Password=FINAL_TEST_PASSWORD,
            Permanent=True,
        )

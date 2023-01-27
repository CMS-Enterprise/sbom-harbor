"""
-> Module to house functions to create cognito users
-> for test.
"""
from botocore.exceptions import ClientError

from cyclonedx.clients.ciam import CognitoUserData
from tests.conftest import FINAL_TEST_PASSWORD


def test_create_cognito_users(session, environment):

    """
    -> Test to generate cognito users in the AWS account
    """
    cognito_client = session.client("cognito-idp")
    cfn_client = session.client("cloudformation")

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

    stack_name: str = f"{environment}-harbor-user-management-use1"

    response = cfn_client.describe_stacks(StackName=stack_name)

    stack_outputs = response["Stacks"][0]["Outputs"]

    user_pool_id = next(
        output["OutputValue"]
        for output in stack_outputs
        if output["OutputKey"].startswith("ExportsOutputRefCognitoUserPool")
    )

    for username in usernames:

        email_username = f"{username}@aquia.io"

        teams = "dawn-patrol,dusk-patrol" if username == "sbomadmin" else "dawn-patrol"

        try:
            cognito_client.admin_delete_user(
                UserPoolId=user_pool_id,
                Username=email_username,
            )
        except ClientError:
            ...

        cognito_client.admin_create_user(
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

        cognito_client.admin_set_user_password(
            UserPoolId=user_pool_id,
            Username=email_username,
            Password=FINAL_TEST_PASSWORD,
            Permanent=True,
        )

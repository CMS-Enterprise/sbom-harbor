"""
-> Module to house the HarborCognitoClient class
"""
from os import environ

import boto3
import botocore.exceptions

from cyclonedx import harbor_logger
from cyclonedx.clients.ciam.jwt_data import JwtData
from cyclonedx.clients.ciam.user_data import CognitoUserData
from cyclonedx.constants import (
    COGNITO_TEAM_DELIMITER,
    USER_POOL_CLIENT_ID_KEY,
    USER_POOL_ID_KEY,
)
from cyclonedx.exceptions.ciam_exception import HarborCiamError
from cyclonedx.model.member import Member

# logger = logging.getLogger('')
# logger.setLevel(logging.DEBUG)
# sh = logging.StreamHandler(sys.stdout)
# formatter = logging.Formatter('[%(asctime)s] %(levelname)s [%(filename)s.%(funcName)s:%(lineno)d] %(message)s', datefmt='%a, %d %b %Y %H:%M:%S')
# sh.setFormatter(formatter)
# logger.addHandler(sh)
logger = harbor_logger.getChild(__name__)


class HarborCognitoClient:

    """
    -> Class to present an API for CIAM actions
    """

    @staticmethod
    def get_jwt_data(token: str) -> JwtData:
        # logger2 = logging.getLogger(__name__)
        """
        -> returns a JwtData object useful for extracting values from a JWT
        """
        # print("getting shit-------------")

        logger.info("Component INFO TEST----------- ")
        logger.error("this is an error oh noes %s", "fuck")
        logger.debug("DEBUG ----------- ")
        logger.warning("WARNING ----------- ")
        logger.critical("This is a critical error: %s", "my stack trace")
        return JwtData(token)

    def __find_username_by_email(self: "HarborCognitoClient", user_email: str) -> str:

        """
        -> Use the member's email, which is a standard attribute of the Cognito
        -> user, to locate the username so we can add a team to the user's
        -> "teams:custom" custom attribute.
        """

        response = self.cognito_client.list_users(
            UserPoolId=self.user_pool_id,
            Limit=1,
            Filter=f'email ^= "{user_email}"',
        )

        users: list = response["Users"]

        if not users:
            raise HarborCiamError(f"No users found with email {user_email}")

        user: dict = users.pop()
        username: str = user["Username"]

        return username

    def __init__(self):

        """
        -> Constructor
        """

        self.user_pool_id: str = environ.get(USER_POOL_ID_KEY)
        if not self.user_pool_id:
            raise ValueError(
                "Error: The value of USER_POOL_ID_KEY is empty in this environment."
            )

        self.cognito_client = boto3.client("cognito-idp")

    def get_user_data(
        self: "HarborCognitoClient",
        cognito_username: str,
    ) -> CognitoUserData:

        """
        -> Creates and returns a CognitoUserData object
        -> to extract values from the Cognito User Data
        """

        return CognitoUserData(
            self.cognito_client.admin_get_user(
                UserPoolId=self.user_pool_id,
                Username=cognito_username,
            )
        )

    def get_jwt(
        self: "HarborCognitoClient",
        username: str,
        password: str,
    ) -> str:

        """
        -> Get the JWT from a username and password
        """

        try:
            resp = self.cognito_client.admin_initiate_auth(
                UserPoolId=environ.get(USER_POOL_ID_KEY),
                ClientId=environ.get(USER_POOL_CLIENT_ID_KEY),
                AuthFlow="ADMIN_NO_SRP_AUTH",
                AuthParameters={
                    "USERNAME": username,
                    "PASSWORD": password,
                },
            )
        except self.cognito_client.exceptions.NotAuthorizedException as err:
            logger.info("Caught NotAuthorizedException: %s", err)
            raise HarborCiamError("Not Authorized") from err

        return resp["AuthenticationResult"]["AccessToken"]

    def add_team_to_member(
        self: "HarborCognitoClient",
        team_id: str,
        cognito_username: str = "",
        member: Member = None,
    ):

        """
        -> Method to add team to the member's custom attribute
        """

        if not team_id:
            raise HarborCiamError(f"Team id must be specified. It was: ({team_id})")

        try:

            if not cognito_username and not member:
                raise HarborCiamError("Must specify either cognito_username or Member.")

            if not cognito_username:
                cognito_username: str = self.__find_username_by_email(member.email)

            teams: str = self.get_user_data(cognito_username).teams
            teams_set: set = set(teams.split(COGNITO_TEAM_DELIMITER))
            teams_set.add(team_id)

            try:
                teams_set.remove("")
            except KeyError:
                # Remove the blank string if it's there
                # otherwise, just ignore the exception
                ...

            self.cognito_client.admin_update_user_attributes(
                UserPoolId=self.user_pool_id,
                Username=cognito_username,
                UserAttributes=[
                    {
                        "Name": CognitoUserData.Attrib.TEAMS,
                        "Value": COGNITO_TEAM_DELIMITER.join(teams_set),
                    },
                ],
            )

        except botocore.exceptions.ClientError as ce:
            raise HarborCiamError(
                f"Unable to update user with team({team_id}): {str(ce)}"
            ) from ce

    def remove_team_from_member(
        self: "HarborCognitoClient",
        team_id: str,
        cognito_username: str = "",
        member: Member = None,
    ):

        """
        -> Method to remove a team from the member's custom attribute
        """

        if not cognito_username and member:
            cognito_username: str = self.__find_username_by_email(member.email)

        teams: str = self.get_user_data(cognito_username).teams
        teams_set: set = set(teams.split(COGNITO_TEAM_DELIMITER))

        try:
            teams_set.remove(team_id)
            self.cognito_client.admin_update_user_attributes(
                UserPoolId=self.user_pool_id,
                Username=cognito_username,
                UserAttributes=[
                    {
                        "Name": CognitoUserData.Attrib.TEAMS,
                        "Value": COGNITO_TEAM_DELIMITER.join(teams_set),
                    },
                ],
            )
        except KeyError as ke:
            # If the user didn't have that team, then here we are.
            err: str = (
                f"KeyError attempting to remove {team_id} from {cognito_username}"
            )
            raise HarborCiamError(err) from ke

    def get_matching_users(self: "HarborCognitoClient", filter_str: str) -> list[str]:

        """
        -> Get the matching users for a filter string
        """

        user_filter = f'email ^= "{filter_str}"'

        response = self.cognito_client.list_users(
            UserPoolId=environ.get(USER_POOL_ID_KEY),
            AttributesToGet=[
                "email",
            ],
            Limit=60,  # Max is 60
            Filter=user_filter,
        )

        # fmt: off
        return [
            user["Attributes"][0]["Value"]
            for user in response["Users"]
        ]
        # fmt: on

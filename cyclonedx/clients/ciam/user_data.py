"""
-> Module to house the CognitoUserData class
"""
from typing import Callable


class CognitoUserData:

    """
    -> Class to store and manipulate Cognito User Data
    """

    class Attrib:

        """
        -> Cognito attributes we are interested in
        """

        TEAMS = "custom:teams"
        EMAIL = "email"

    def __init__(self, cognito_user_data: dict):

        """
        -> Constructor
        """

        self.cognito_user_data: dict = cognito_user_data

    @property
    def email(self) -> str:

        """
        -> Getter property for a user's email
        """

        return self.__get_attrib_value(
            CognitoUserData.Attrib.EMAIL,
        )

    @property
    def teams(self) -> str:

        """
        -> Getter property for a user's teams
        """

        return self.__get_attrib_value(CognitoUserData.Attrib.TEAMS)

    def __get_attrib_value(self, attrib: str):

        try:
            user_attrib = self.cognito_user_data["UserAttributes"]

            filter_lambda: Callable = lambda o: o["Name"] == attrib
            teams_filter: filter = filter(filter_lambda, user_attrib)
            teams_attr: list = list(teams_filter)

            teams: str = ""
            if len(teams_attr) > 0:
                teams_attr_value: dict = teams_attr[0]
                teams: str = teams_attr_value["Value"]

            return "" if teams == "," else teams
        except KeyError as ke:
            print(f"KeyError while getting teams: {ke}")
            return ""

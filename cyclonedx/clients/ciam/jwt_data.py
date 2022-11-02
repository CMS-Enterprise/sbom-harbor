"""
-> Module to house the JwtData class
"""
from jose.jwt import get_unverified_claims


class JwtData:

    """
    -> Class to extract JWT data
    """

    def __init__(self, token: str):

        """
        -> Constructor store the token and get the claims
        """

        self._token = token
        self._claims = get_unverified_claims(token)

    @property
    def token(self):

        """
        -> Property to return the JWT token
        """

        return self._token

    @property
    def username(self):

        """
        -> Property to return the extracted username
        """

        return self._claims["username"]

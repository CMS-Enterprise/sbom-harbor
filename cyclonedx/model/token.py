""" Token Model Object. Represents a Token within the SBOM Harbor System. """
from datetime import datetime
from uuid import uuid4

import pytz
from dateutil.parser import parse

from cyclonedx.model import EntityKey, EntityType, HarborModel


def generate_token() -> str:
    """
    -> Function to generate an API token consistently
    """

    return f"sbom-api-{uuid4()}"


def parse_datetime(expires: str):
    """
    Parses date/datetime strings and converts to UTC.
    """
    if isinstance(expires, datetime):
        return expires.astimezone(pytz.utc).isoformat()

    if expires is None or len(expires) == 0:
        return expires

    try:
        dt = parse(expires)

        # All expiration logic uses UTC time, so we have to store a timezone aware date.
        if dt.tzinfo is None:
            # if no timezone is set by the caller we have to pick one, so we pick the server time.
            dt = dt.astimezone()

        return dt.astimezone(pytz.utc).isoformat()
    # pylint: disable = W0703
    except Exception:
        # We don't want to error because we had a problem parsing a date
        # so just set it to a blank string and the token will always be
        # invalid and unusable.
        return ""


class Token(HarborModel):
    """
    A Token, is an entity that represents a string used to authorize sending
    SBOMs into the system.
    """

    class Fields(HarborModel.Fields):
        """Inner Class to hold the fields of a Token"""

        NAME = "name"
        CREATED = "created"
        EXPIRES = "expires"
        ENABLED = "enabled"
        TOKEN = "token"

    @classmethod
    def to_instance(
        cls,
        entity_key: EntityKey,
        item: dict,
        children: dict[str, list[HarborModel]] = None,
    ) -> "Token":
        """to_instance() Creates a Token from its data"""

        return Token(
            team_id=entity_key.team_id,
            token_id=entity_key.entity_id,
            name=item[Token.Fields.NAME],
            created=item[Token.Fields.CREATED],
            expires=item[Token.Fields.EXPIRES],
            enabled=item[Token.Fields.ENABLED],
            token=item[Token.Fields.TOKEN],
        )

    # pylint: disable=R0913
    def __init__(
        self: "HarborModel",
        team_id: str,
        token_id: str,
        name: str = "None",
        created: str = None,
        expires: str = None,
        enabled: bool = True,
        token: str = "",
    ):
        """Constructor"""

        super().__init__(
            EntityKey(
                team_id=team_id,
                entity_id=token_id,
                entity_type=EntityType.TOKEN,
            ),
        )

        self._name: str = name
        self._created: str = created
        self._expires: str = parse_datetime(expires)
        self._enabled: bool = enabled
        self._token: str = token

    @property
    def name(self) -> str:
        """Define the property that holds when the token was created"""

        return self._name

    @name.setter
    def name(self, name: str):
        """Define the property that holds when the token was created"""

        self._name = name

    @property
    def created(self) -> str:
        """Define the property that holds when the token was created"""

        return self._created

    @property
    def expires(self) -> str:
        """Define the property that holds when the token expires"""

        return self._expires

    @expires.setter
    def expires(self, expires: str):
        """
        -> Setter for the str 'expires' property
        """

        self._expires = parse_datetime(expires)

    @property
    def enabled(self) -> bool:
        """Define the property that tell us if the token is enabled"""

        return self._enabled

    @enabled.setter
    def enabled(self, enabled: bool):
        """
        -> Setter for the boolean 'enabled' property
        """

        self._enabled = enabled

    @property
    def token(self) -> str:
        """Define the property that holds the token itself"""

        return self._token

    def get_item(self) -> dict:
        """Get the dictionary representation of the Token"""

        return {
            **super().get_item(),
            Token.Fields.NAME: self._name,
            Token.Fields.CREATED: self.created,
            Token.Fields.EXPIRES: self.expires,
            Token.Fields.ENABLED: self.enabled,
            Token.Fields.TOKEN: self.token,
        }

    def to_json(self):
        """
        Return a dictionary that can be sent as the
        json representation of a given model object
        """

        return {
            HarborModel.Fields.ID: self.entity_id,
            Token.Fields.NAME: self._name,
            Token.Fields.CREATED: self.created,
            Token.Fields.EXPIRES: self.expires,
            Token.Fields.ENABLED: self.enabled,
            Token.Fields.TOKEN: self.token,
        }

    def is_expired(self):
        """
        Determines whether a token is expired.
        """

        # If the expires field is not set, it is an invalid token.
        if self.expires is None:
            return True

        dt = parse(self.expires)
        # Defensive coding for existing records.
        if dt.tzinfo is None:
            dt = dt.astimezone(pytz.utc)

        return dt <= datetime.now(pytz.utc)

    def is_valid(self):
        """
        Determines whether a token is valid for use.
        """
        return self.enabled and not self.is_expired()

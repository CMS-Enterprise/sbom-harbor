""" Token Model Object. Represents a Token within the SBOM Harbor System. """

from decimal import Decimal
from cyclonedx.model import (
    EntityKey,
    HarborModel,
    EntityType,
)


class Token(HarborModel):

    """
    A Token, is an entity that represents a string used to authorize sending
    SBOMs into the system.
    """

    class Fields(HarborModel.Fields):

        """Inner Class to hold the fields of a Token"""

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
        created: Decimal = None,
        expires: Decimal = None,
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

        self._created: Decimal = created
        self._expires: Decimal = expires
        self._enabled: bool = enabled
        self._token: str = token

    @property
    def created(self) -> Decimal:

        """Define the property that holds when the token was created"""

        return self._created

    @property
    def expires(self) -> Decimal:

        """Define the property that holds when the token expires"""

        return self._expires

    @property
    def enabled(self) -> bool:

        """Define the property that tell us if the token is enabled"""

        return self._enabled

    @property
    def token(self) -> str:

        """Define the property that holds the token itself"""

        return self._token

    def get_item(self) -> dict:

        """Get the dictionary representation of the Token"""

        return {
            **super().get_item(),
            **self.to_json(),
        }

    def to_json(self):

        """
        Return a dictionary that can be sent as the
        json representation of a given model object
        """

        return {
            Token.Fields.CREATED: float(self.created),
            Token.Fields.EXPIRES: float(self.expires),
            Token.Fields.ENABLED: self.enabled,
            Token.Fields.TOKEN: self.token,
        }

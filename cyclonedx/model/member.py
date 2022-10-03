""" Member Model Object. Represents a Team Member within the SBOM Harbor System. """

from cyclonedx.model import (
    EntityKey,
    HarborModel,
    EntityType,
)


class Member(HarborModel):

    """
    A Member, is an entity representing an
    SBOM Harbor User who can submit SBOMs
    """

    class Fields(HarborModel.Fields):

        """Inner Class to hold the fields of a Member"""

        # The Email of the Member
        EMAIL = "email"

        # Whether the user is a team lead
        IS_TEAM_LEAD = "isTeamLead"

    @classmethod
    def to_instance(
        cls,
        entity_key: EntityKey,
        item: dict,
        children: dict[str, list[HarborModel]] = None,
    ) -> "Member":

        """to_instance() Creates a Member from its data"""

        return Member(
            team_id=entity_key.team_id,
            member_id=entity_key.entity_id,
            is_team_lead=item[Member.Fields.IS_TEAM_LEAD],
            email=item[Member.Fields.EMAIL],
        )

    def __init__(
        self: "Member",
        team_id: str,
        member_id: str,
        email: str = "",
        is_team_lead: bool = False,
    ):

        """Constructor"""

        super().__init__(
            EntityKey(
                team_id=team_id, entity_id=member_id, entity_type=EntityType.MEMBER
            ),
        )

        self._email = email
        self._is_team_lead = is_team_lead

    @property
    def email(self) -> str:

        """Define the email property"""

        return self._email

    @property
    def is_team_lead(self) -> bool:

        """Define the is_team_lead property"""

        return self._is_team_lead

    def get_item(self) -> dict:

        """Get the dictionary representation of the Member"""

        return {
            **super().get_item(),
            Member.Fields.EMAIL: self.email,
            Member.Fields.IS_TEAM_LEAD: self.is_team_lead,
        }

    def to_json(self):

        return {
            Member.Fields.EMAIL: self.email,
            Member.Fields.IS_TEAM_LEAD: self.is_team_lead,
        }

""" Team Model Object. Represents a Team within the SBOM Harbor System. """

from cyclonedx.model import (
    HarborModel,
    EntityKey,
    EntityType,
)
from cyclonedx.model.project import Project


class Team(HarborModel):

    """
    A Team, is a named entity that can Contain 3 child types:
    - A Project
    - A Member of the Team
    - A Token
    """

    class Fields(HarborModel.Fields):

        """Inner Class to hold the fields of a Team"""

        # The Name of the Team
        NAME = "name"

    @classmethod
    def to_instance(
        cls,
        entity_key: EntityKey,
        item: dict,
        children: dict[str, list[HarborModel]] = None,
    ) -> "Team":

        """to_instance() Creates a Team from its data"""

        children = {} if not children else children
        return Team(
            team_id=entity_key.team_id,
            name=item[Team.Fields.NAME],
            projects=children.get(EntityType.PROJECT.value, []),
            members=children.get(EntityType.MEMBER.value, []),
            tokens=children.get(EntityType.TOKEN.value, []),
        )

    # pylint: disable = R0913
    def __init__(
        self: "Team",
        team_id: str,
        name: str = "",
        projects: list[HarborModel] = None,
        members: list[HarborModel] = None,
        tokens: list[HarborModel] = None,
    ):

        """Constructor"""

        super().__init__(
            EntityKey(
                team_id=team_id,
                entity_type=EntityType.TEAM,
            ),
            child_types=[
                EntityType.PROJECT,
                EntityType.MEMBER,
                EntityType.TOKEN,
            ],
        )

        # The name is the only Team Field for now
        self._name: str = name

        # Initialize the children
        self._children: dict[str, list[HarborModel]] = {
            EntityType.PROJECT.value: projects or [],
            EntityType.MEMBER.value: members or [],
            EntityType.TOKEN.value: tokens or [],
        }

    @property
    def name(self) -> str:

        """Define the name property"""

        return self._name

    @name.setter
    def name(self, name) -> None:

        """Set the name property"""

        self._name = name

    @property
    @HarborModel.entity_id.getter
    def entity_id(self):

        """The Entity id of a Team is just the id of the Team itself"""

        return self.team_id

    @property
    def projects(self) -> list[Project]:

        """
        -> Return a list of Projects that are the children of this Team
        """

        children: dict[str, list[HarborModel]] = self.get_children()
        projects: list[HarborModel] = children["project"]

        return [
            Project(
                team_id=self.team_id,
                project_id=project.entity_id,
                name=project.get_item().get("name"),
                codebases=project.codebases,
            )
            for project in projects
        ]

    def clear_child_type(self, entity_type: EntityType):

        """
        -> Lets us remove the children before adding more
        """

        self._children[entity_type.value].clear()

    def get_item(self) -> dict:

        """
        -> Get the dictionary representation of the Team
        """

        return {
            **super().get_item(),
            Team.Fields.NAME: self.name,
        }

    def to_json(self):

        """
        -> Return a dictionary that can be sent as the
        -> json representation of a given model object
        """

        projects: [HarborModel] = self._children[EntityType.PROJECT.value]
        ret_projects = {}
        for project in projects:
            ret_projects[project.entity_id] = project.to_json()

        members: [HarborModel] = self._children[EntityType.MEMBER.value]
        ret_members = {}
        for member in members:
            ret_members[member.entity_id] = member.to_json()

        tokens: [HarborModel] = self._children[EntityType.TOKEN.value]
        ret_tokens = {}
        for token in tokens:
            ret_tokens[token.entity_id] = token.to_json()

        return {
            Team.Fields.NAME: self.name,
            "projects": ret_projects,
            "members": ret_members,
            "tokens": ret_tokens,
        }


""" Project Model Object. Represents a Project within the SBOM Harbor System. """

from cyclonedx.model import (
    EntityKey,
    EntityType,
    HarborModel,
)

class Project(HarborModel):

    """
        A Project, is a named entity that can Contain 1 child type:
        - A Codebase
    """

    class Fields(HarborModel.Fields):

        """ Inner Class to hold the fields of a Project """

        # The Name of the Project
        NAME="name"

    @classmethod
    def to_instance(
        cls, entity_key: EntityKey, item: dict,
        children: dict[str, list[HarborModel]] = None
    ) -> 'Project':

        """ to_instance() Creates a Project from its data """

        children = {} if not children else children
        return Project(
            team_id=entity_key.team_id,
            project_id=entity_key.entity_id,
            name=item[Project.Fields.NAME],
            codebases=children.get(EntityType.CODEBASE.value, [])
        )

    def __init__(
        self: 'Project',
        team_id: str,
        project_id: str,
        name: str="",
        codebases: list[HarborModel]=None,
    ):

        """ Constructor """

        super().__init__(
            EntityKey(
                team_id=team_id,
                entity_id=project_id,
                entity_type=EntityType.PROJECT,
            ),
            child_types=[
                EntityType.CODEBASE
            ]
        )

        # The name is the only Project Field for now
        self._name: str = name

        # Initialize the children
        self._children: dict[str, list[HarborModel]] = {
            EntityType.CODEBASE.value: codebases or []
        }

    @property
    def name(self) -> str:

        """ Define the name property """

        return self._name

    def get_item(self) -> dict:

        """ Get the dictionary representation of the Project """

        return {
            **super().get_item(),
            Project.Fields.NAME: self._name,
        }

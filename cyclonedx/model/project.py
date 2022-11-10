""" Project Model Object. Represents a Project within the SBOM Harbor System. """

from cyclonedx.model import EntityKey, EntityType, HarborModel
from cyclonedx.model.codebase import CodeBase


class Project(HarborModel):

    """
    A Project, is a named entity that can Contain 1 child type:
    - A Codebase
    """

    class Fields(HarborModel.Fields):

        """Inner Class to hold the fields of a Project"""

        # The Name of the Project
        NAME = "name"

        # The FISMA ID for the Project
        FISMA = "fisma"

    @classmethod
    def to_instance(
        cls,
        entity_key: EntityKey,
        item: dict,
        children: dict[str, list[HarborModel]] = None,
    ) -> "Project":

        """to_instance() Creates a Project from its data"""

        children = {} if not children else children
        return Project(
            team_id=entity_key.team_id,
            project_id=entity_key.entity_id,
            name=item[Project.Fields.NAME],
            fisma=item[Project.Fields.FISMA],
            codebases=children.get(EntityType.CODEBASE.value, []),
        )

    def __init__(
        self: "Project",
        team_id: str,
        project_id: str,
        name: str = "",
        fisma: str = "unknown",
        codebases: list[HarborModel] = None,
    ):

        """Constructor"""

        super().__init__(
            EntityKey(
                team_id=team_id,
                entity_id=project_id,
                entity_type=EntityType.PROJECT,
            ),
            child_types=[EntityType.CODEBASE],
        )

        self._name: str = name
        self._fisma: str = fisma

        # Initialize the children
        self._children: dict[str, list[HarborModel]] = {
            EntityType.CODEBASE.value: codebases or []
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
    def fisma(self) -> str:

        """Define the fisma property"""

        return self._fisma

    @fisma.setter
    def fisma(self, fisma) -> None:

        """Set the fisma property"""

        self._fisma = fisma

    @property
    def codebases(self) -> list[CodeBase]:

        """
        -> Return a list of Codebases that are children of this project
        """

        children: dict[str, list[HarborModel]] = self.get_children()
        codebases: list[HarborModel] = children.get("codebase", [])
        return [
            CodeBase(
                team_id=self.team_id,
                project_id=codebase.parent_id,
                codebase_id=codebase.entity_id,
                name=codebase.get_item().get(CodeBase.Fields.NAME),
                language=codebase.get_item().get(CodeBase.Fields.LANGUAGE),
                build_tool=codebase.get_item().get(CodeBase.Fields.BUILD_TOOL),
            )
            for codebase in codebases
        ]

    def clear_codebases(self):

        """
        -> Lets us remove the codebases before adding more
        """

        self._children[EntityType.CODEBASE.value].clear()

    def get_item(self) -> dict:

        """Get the dictionary representation of the Project"""

        return {
            **super().get_item(),
            Project.Fields.NAME: self._name,
            Project.Fields.FISMA: self._fisma,
        }

    def to_json(self):

        codebases: list[HarborModel] = self._children[EntityType.CODEBASE.value]
        ret_codebases = [codebase.to_json() for codebase in codebases]

        return {
            HarborModel.Fields.ID: self.entity_id,
            Project.Fields.NAME: self._name,
            Project.Fields.FISMA: self._fisma,
            "codebases": ret_codebases,
        }

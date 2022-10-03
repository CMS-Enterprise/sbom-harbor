"""
-> Codebase Model Object. Represents a Team within the SBOM Harbor System.
"""

from cyclonedx.model import (
    EntityKey,
    HarborModel,
    EntityType,
)


class CodeBase(HarborModel):

    """
    -> A Codebase, is a named entity that contains information about
    -> a code base such as the language the software is developed in
    -> and the tooling use to build the code.
    """

    class Fields(HarborModel.Fields):

        """Inner Class to hold the fields of a Codebase"""

        # The Name of the Codebase
        NAME = "name"

        # The Language the code is written in.
        LANGUAGE = "language"

        # The build tool used to build the code
        BUILD_TOOL = "buildTool"

    @classmethod
    def to_instance(
        cls,
        entity_key: EntityKey,
        item: dict,
        children: dict[str, list[HarborModel]] = None,
    ) -> "CodeBase":

        """to_instance() Creates a Codebase from its data"""

        return CodeBase(
            team_id=entity_key.team_id,
            codebase_id=entity_key.entity_id,
            name=item[CodeBase.Fields.NAME],
            language=item[CodeBase.Fields.LANGUAGE],
            build_tool=item[CodeBase.Fields.BUILD_TOOL],
            project_id=item[CodeBase.Fields.PARENT_ID],
        )

    # pylint: disable = R0913
    def __init__(
        self: "HarborModel",
        team_id: str,
        codebase_id: str,
        name: str = "",
        language: str = "",
        build_tool: str = "",
        project_id: str = "",
    ):

        """Constructor"""

        super().__init__(
            EntityKey(
                team_id=team_id,
                entity_id=codebase_id,
                entity_type=EntityType.CODEBASE,
            ),
            parent_id=project_id,
        )

        self._name = name
        self._language = language
        self._build_tool = build_tool

    @property
    def name(self):

        """Define the name property"""

        return self._name

    @property
    def language(self):

        """Define the language property"""

        return self._language

    @property
    def build_tool(self):

        """Define the build_tool property"""

        return self._build_tool

    def get_item(self) -> dict:

        """Get the dictionary representation of the Codebase"""

        return {
            **super().get_item(),
            CodeBase.Fields.NAME: self._name,
            CodeBase.Fields.LANGUAGE: self._language,
            CodeBase.Fields.BUILD_TOOL: self._build_tool,
        }

    def to_json(self):
        return {
            CodeBase.Fields.NAME: self._name,
            CodeBase.Fields.LANGUAGE: self._language,
            CodeBase.Fields.BUILD_TOOL: self._build_tool,
        }

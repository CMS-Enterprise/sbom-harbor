""" Module contains Abstract Types to use with DynamoDB as the SBOM Harbor Model Structure """

# This must be imported to allow the enclosing class to specify
# itself as a return type: -> list[HarborModel]
from __future__ import annotations

import abc
import copy
from enum import Enum
from uuid import uuid4

from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)


def generate_model_id():

    """
    -> This is the single location that model object IDs are created
    """

    return str(uuid4())


class EntityType(Enum):

    """
    -> EntityType Defines the Model Types
    """

    TEAM = "team"
    PROJECT = "project"
    CODEBASE = "codebase"
    MEMBER = "member"
    TOKEN = "token"


class EntityKey:

    """EntityKey defines the fields that identify a given object in DynamoDB"""

    def __init__(
        self: "EntityKey",
        team_id: str,
        entity_type: EntityType,
        entity_id: str = "",
    ):
        self._entity_type = entity_type
        self._team_id = team_id
        self._entity_id = entity_id

    @staticmethod
    def split_entity_key(entity_key: str) -> (str, str):

        """
        The EntityKey is string form has a hash(#) which separates
        the EntityType(project, codebase, etc.) from the id of the object
        itself, Ex:

        project#3afe6c86-bf1e-43ef-8335-e0ccf670dc50.

        This was necessary because DynamoDB requires unique needs PrimaryKeys
        and we use unique Sort Keys to implement that.
        """

        if "#" in entity_key:
            (entity_type, entity_id) = entity_key.split("#")
        else:
            entity_type = entity_key
            entity_id = ""

        return entity_type, entity_id

    @property
    def entity_type(self) -> EntityType:

        """Return the EntityType"""

        return self._entity_type

    @property
    def entity_id(self) -> str:

        """
        Return the entity id:
            the unique string that identifies a given model object.
        """

        return self._entity_id

    @property
    def team_id(self) -> str:

        """The team id is the Partition Key. Unique only to a Team"""

        return self._team_id

    @property
    def value(self) -> str:

        """The Value of this Object: The string version of the EntityKey"""

        return (
            f"{self.entity_type.value}#{self.entity_id}"
            if self.entity_id
            else self.entity_type.value
        )


class HarborModel:

    """
    -> Parent class for all model objects.
    """

    class Fields:

        """Inner Class to hold the fields of the HarborModel"""

        PARENT_ID = "parentId"

        ID = "id"

    def __init__(
        self: object,
        entity_key: EntityKey,
        parent_id: str = "",
        child_types: list[EntityType] = None,
    ):
        """Constructor"""

        self._entity_key = entity_key

        # Parent id does not need to be specified if no parent exists
        # it will default to the team id
        self._parent_id = parent_id or entity_key.team_id
        self._children: dict[str, list[HarborModel]] = {}
        self._child_types = child_types or []

    @property
    def entity_id(self):

        """Return the id of this model object"""

        return self._entity_key.entity_id

    @property
    def entity_type(self):

        """Return the id of this model object"""

        return self._entity_key.entity_type

    @property
    def team_id(self) -> str:

        """return the team id or PartitionKey"""

        return self._entity_key.team_id

    @property
    def parent_id(self) -> str:

        """return the parent id if this model object has one."""

        return self._parent_id

    @property
    def entity_key(self) -> str:

        """Return the EntityKey (RangeKey)"""

        return self._entity_key.value

    def get_item(self) -> dict:

        """
        Returns the fields defined in this abstract class:

        - TeamId: The PartitionKey.  Used to scoop a section of DynamoDB's table
        - EntityKey: The RangeKey. Unique composite key used to identify children
            of the Team and specific objects in the system.
        """

        return {
            HARBOR_TEAMS_TABLE_PARTITION_KEY: self.team_id,
            HARBOR_TEAMS_TABLE_SORT_KEY: self.entity_key,
            HarborModel.Fields.PARENT_ID: self.parent_id,
        }

    def has_child_types(self) -> bool:

        """Returns True if a model object has child types"""

        return len(self.get_child_types()) > 0

    def get_child_types(self) -> list[str]:

        """Gets the child types of this model object"""

        return [ct.value for ct in self._child_types]

    def has_children(self):

        """Gets any actual children this model object has loaded"""

        all_children = []

        for _, arr in self._children.items():
            all_children.extend(arr)

        return len(all_children) > 0

    def get_children(self) -> dict[str, list[HarborModel]]:

        """
        Get the children of this HarborModel Object:

            dict["EntityType String Value", list["HarborModel Subtypes"]]
        """

        return copy.deepcopy(self._children)

    def add_child(self, instance: HarborModel):

        """
        Add a child Model Object to this Object.
        If the child type is not defined in the constructor,
        a KeyError will be thrown when attempting to add a child.
        """

        try:
            (entity_type, _) = EntityKey.split_entity_key(instance.entity_key)
            self._children[entity_type].append(instance)
        except KeyError:
            print(f"This class has no children of {entity_type} type, moving on...")

    @classmethod
    @abc.abstractmethod
    def to_instance(
        cls,
        entity_key: EntityKey,
        item: dict,
        children: dict[str, list[HarborModel]] = None,
    ) -> HarborModel:

        """
        This method must be defined for reach Child, Model type
        It takes the EntityKey, the dictionary version of the model object
        and the children and creates a Model Object from them.
        """

        # Not Defined for the parent class.
        ...

    @abc.abstractmethod
    def to_json(self) -> dict:

        """
        Return a dictionary that can be sent as the
        json representation of a given model object
        """

        # Not Defined for the parent class.
        ...

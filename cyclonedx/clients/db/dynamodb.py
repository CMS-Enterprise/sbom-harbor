"""
    This is the module containing the SBOM Harbor Database Client.
"""

from typing import Any, Callable, Type, TypeVar

from boto3.dynamodb.conditions import Attr, Equals, Key
from botocore.exceptions import ClientError

from cyclonedx.constants import (
    HARBOR_TEAMS_TABLE_NAME,
    HARBOR_TEAMS_TABLE_PARTITION_KEY,
    HARBOR_TEAMS_TABLE_SORT_KEY,
)
from cyclonedx.exceptions.database_exception import DatabaseError
from cyclonedx.model import EntityKey, EntityType, HarborModel
from cyclonedx.model.codebase import CodeBase
from cyclonedx.model.member import Member
from cyclonedx.model.project import Project
from cyclonedx.model.team import Team
from cyclonedx.model.token import Token

# Generic type bound to the HarborModel
T = TypeVar("T", bound=HarborModel)


def get_error_message(op: str, model: HarborModel, error: Exception):

    """
    -> Returns a pretty useful error message
    """

    et: str = model.entity_type.value
    ei: str = model.entity_id
    return f"Error executing operation({op}) for {et} with id: {ei} - {str(error)}"


class HarborDBClient:

    """
    SBOM Harbor Database Client.

    This client can Create, Update or Delete the data in DynamoDB that
    corresponds to a given HarborModel Object.

    Each of the methods in the client take a 'recurse' flag so it can
    perform the same actions requested on the top level Model Object
    to all the child objects it has.  The only caveat is that the objects
    need to be loaded before they can be deleted by the client.  Any
    Model object that has children can be loaded with said children by
    specifying that the Object be loaded with the 'recurse' flag.

    For Example, here is the code necessary to load an entire Team with
    all it's children (Projects, Members and Tokens) that correspond to
    the team id 'abc123':

    client = HarborDBClient()
    team: Team = client.get(Team(team_id='abc123'), recurse=True)

    """

    @staticmethod
    def __map_to_type(entity_type: str) -> Type[HarborModel]:

        """
        Function that Maps the EntityType to the Class type
        :param entity_type: is the string version of EntityType
        :return: the corresponding Class type to be instantiated
        """

        type_map = {
            EntityType.TEAM.value: Team,
            EntityType.PROJECT.value: Project,
            EntityType.CODEBASE.value: CodeBase,
            EntityType.MEMBER.value: Member,
            EntityType.TOKEN.value: Token,
        }

        return type_map[entity_type]

    def __init__(self, dynamodb_resource: Any):

        """
        Constructor: Sets up the DynamoDB Table for our use.
        :param dynamodb_resource: is the Boto3 DynamoDB Resource
        https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/dynamodb.html#service-resource
        """

        self.table = dynamodb_resource.Table(HARBOR_TEAMS_TABLE_NAME)

    @staticmethod
    def _recurse(model: T, func: Callable) -> None:

        """
        Generic recursion to run the input function on each model object
        """

        items = model.get_children().items()
        for _, child_model_objs in items:
            child_model_obj: HarborModel
            for child_model_obj in child_model_objs:
                func(child_model_obj, recurse=True)

    @staticmethod
    def __get_instance(
        item: dict,
        team_id: str,
        children: dict[str, list[HarborModel]] = None,
    ) -> T:

        """
        Gets an Instance for the data from a DynamoDB Request

        :param item: is the return value of a DynamoDB Query
        :param team_id: Is the PartitionKey for the database
        :param children: Are child types associated to the Model Object
        :return: A Filled Model Object with filled children
        """

        entity_key: str = item[HARBOR_TEAMS_TABLE_SORT_KEY]
        (entity_type, entity_id) = EntityKey.split_entity_key(entity_key)
        model_type = HarborDBClient.__map_to_type(entity_type)
        return model_type.to_instance(
            entity_key=EntityKey(
                team_id=team_id,
                entity_type=entity_type,
                entity_id=entity_id,
            ),
            item=item,
            children=children,
        )

    def create(
        self: "HarborDBClient",
        model: T,
        recurse: bool = False,
    ) -> T:

        """
        Creates an entry or series of entries in DynamoDB representing
        a single or series of Model Objects

        :param model: Model Object to use for the DynamoDB Entry
        :param recurse: Create any children the Model Object has
        :return: The Filled, new Model Object
        """

        try:
            self.table.put_item(Item=model.get_item())
        except ClientError as err:
            err_msg: str = get_error_message("create", model, err)
            raise DatabaseError(err_msg) from err

        if recurse and model.has_children():
            HarborDBClient._recurse(model=model, func=self.create)

        return model

    def update(self: "HarborDBClient", model: T, recurse: bool = False) -> T:

        """
        Updates an entry or series of entries in DynamoDB representing
        a single or series of Model Objects

        :param model: Model Object to use to update the DynamoDB Entry
        :param recurse: Update any children the Model Object has
        :return: The Filled Model Object
        """

        try:

            sort_key_attr: Attr = Attr(HARBOR_TEAMS_TABLE_SORT_KEY)
            entity_key_already_exists: Equals = sort_key_attr.eq(model.entity_key)

            self.table.put_item(
                Item=model.get_item(),
                ConditionExpression=entity_key_already_exists,
            )
        except ClientError as err:
            err_msg: str = get_error_message("update", model, err)
            raise DatabaseError(err_msg) from err

        if recurse and model.has_children():
            HarborDBClient._recurse(model=model, func=self.update)

        return model

    def delete(self: "HarborDBClient", model: T, recurse: bool = False) -> None:

        """
        Deletes an entry or series of entries in DynamoDB representing
        a single or series of Model Objects

        :param model: Model Object to use to delete the DynamoDB Entry
        :param recurse: Delete any children the Model Object has
        """

        try:
            self.table.delete_item(
                Key={
                    HARBOR_TEAMS_TABLE_PARTITION_KEY: model.team_id,
                    HARBOR_TEAMS_TABLE_SORT_KEY: model.entity_key,
                }
            )
        except ClientError as err:
            err_msg: str = get_error_message("delete", model, err)
            raise DatabaseError(err_msg) from err

        if recurse and model.has_children():
            HarborDBClient._recurse(model=model, func=self.delete)

    def __load_children(
        self: "HarborDBClient",
        parent: T,
        equals_clause: Equals,
        sort_key: Key,
    ) -> None:

        """
        Helper Function for get().  Loads the children of a given Model Object
        if recursion is specified when getting data from the Database

        :param parent: Is the parent model Object
        :param equals_clause: Specifies that the TeamId is used for the PartitionKey
        :param sort_key: Specifies the begins_with() clause for a given type
        """

        if parent.has_child_types():
            for child_type in parent.get_child_types():
                try:
                    results: dict = self.table.query(
                        KeyConditionExpression=equals_clause
                        & sort_key.begins_with(child_type),
                    )

                    try:
                        items = results["Items"]
                    except KeyError:
                        items = []

                    for item in items:

                        # Create a new Instance from the model and item
                        instance = HarborDBClient.__get_instance(
                            team_id=parent.team_id,
                            item=item,
                        )

                        # Verify that any children actually belong to the parent
                        # This is a bit inefficient because we will be getting all the children
                        # back from every query and we should only need 1 query of any given type,
                        # but we'll worry about that later.
                        if parent.entity_id == instance.parent_id:
                            self.__load_children(instance, equals_clause, sort_key)
                            parent.add_child(instance)

                except Exception as err:
                    err_msg: str = get_error_message("loading children", parent, err)
                    raise DatabaseError(err_msg) from err

    def get(
        self: "HarborDBClient",
        model: T,
        recurse: bool = False,
    ) -> T:

        """
        Retrieves an entry or series of entries in DynamoDB representing
        a single or series of Model Objects

        :param model: Model Object to use to retrieve the DynamoDB Entry
        :param recurse: Retrieve any children the Model Object has
        :return: The Filled Model Object
        """

        partition_key: Key = Key(HARBOR_TEAMS_TABLE_PARTITION_KEY)
        sort_key: Key = Key(HARBOR_TEAMS_TABLE_SORT_KEY)
        partition_key_equals: Equals = partition_key.eq(model.team_id)

        if recurse and model.has_child_types():
            self.__load_children(model, partition_key_equals, sort_key)

        try:
            sort_key_equals: Equals = sort_key.eq(model.entity_key)
            results: dict = self.table.query(
                KeyConditionExpression=partition_key_equals & sort_key_equals,
            )

            item = results["Items"].pop()
            return HarborDBClient.__get_instance(
                item, team_id=model.team_id, children=model.get_children()
            )
        except Exception as err:
            err_msg: str = get_error_message("getting", model, err)
            raise DatabaseError(err_msg) from err

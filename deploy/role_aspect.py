from typing import Union

import jsii
import aws_cdk as cdk
import constructs
from aws_cdk import aws_iam
from jsii._reference_map import _refs
from jsii._utils import Singleton


@jsii.implements(cdk.IAspect)
class RoleAspect:
    """
    This aspect finds all aws_iam.Role objects in a node (ie. CDK stack) and sets permission boundary to the given ARN.
    """

    def __init__(self, permission_boundary_arn: str, path: str) -> None:
        """
        :param permission_boundary_arn: Either aws_iam.ManagedPolicy object or managed policy's ARN string
        """
        self.permission_boundary_arn = permission_boundary_arn
        self.path = path

    def visit(self, construct_ref: constructs.IConstruct) -> None:
        """
        construct_ref only contains a string reference to an object. To get the actual object, we need to resolve it using JSII mapping.
        :param construct_ref: ObjRef object with string reference to the actual object.
        :return: None
        """
        if isinstance(construct_ref, jsii._kernel.ObjRef) and hasattr(
            construct_ref, "ref"
        ):
            kernel = Singleton._instances[
                jsii._kernel.Kernel
            ]  # The same object is available as: jsii.kernel
            resolve = _refs.resolve(kernel, construct_ref)
        else:
            resolve = construct_ref

        def _walk(obj):
            # print(obj)
            if isinstance(obj, aws_iam.Role):
                cfn_role = obj.node.find_child("Resource")
                cfn_role.add_property_override(
                    "PermissionsBoundary", self.permission_boundary_arn
                )
                cfn_role.add_property_override("Path", self.path)
            elif isinstance(obj, aws_iam.CfnRole):
                # print(obj.)
                obj.permissions_boundary = self.permission_boundary_arn
                obj.path = self.path
            else:
                if hasattr(obj, "permissions_node"):
                    for c in obj.permissions_node.children:
                        _walk(c)
                if hasattr(obj, "node") and obj.node.children:
                    for c in obj.node.children:
                        _walk(c)

        _walk(resolve)

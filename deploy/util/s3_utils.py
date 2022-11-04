""" Utility class for adding extended features to S3 buckets"""
from aws_cdk import aws_s3 as s3
from aws_cdk.aws_iam import PolicyStatement, Role, ServicePrincipal
from aws_cdk.aws_s3 import CfnBucket

from deploy.constants import EXTERNAL_BUCKET_NAME


def set_source_bucket_replication(self):
    """Sets a bucket to be a source of replication and point it to its destination"""
    destination_bucket = s3.Bucket.from_bucket_arn(
        self, "DestinationBucket", bucket_arn=f"arn:aws:s3:::{EXTERNAL_BUCKET_NAME}"
    )

    replication_role = Role(
        self,
        "ReplicationRole",
        assumed_by=ServicePrincipal("s3.amazonaws.com"),
        path="/service-role/",
    )

    replication_role.add_to_policy(
        PolicyStatement(
            resources=[self.s3_bucket.bucket_arn],
            actions=["s3:GetReplicationConfiguration", "s3:ListBucket"],
        )
    )

    replication_role.add_to_policy(
        PolicyStatement(
            resources=[self.s3_bucket.arn_for_objects("*")],
            actions=[
                "s3:GetObjectVersion",
                "s3:GetObjectVersionAcl",
                "s3:GetObjectVersionForReplication",
                "s3:GetObjectLegalHold",
                "s3:GetObjectVersionTagging",
                "s3:GetObjectRetention",
            ],
        )
    )

    replication_role.add_to_policy(
        PolicyStatement(
            resources=[destination_bucket.arn_for_objects("*")],
            actions=[
                "s3:ReplicateObject",
                "s3:ReplicateDelete",
                "s3:ReplicateTags",
                "s3:GetObjectVersionTagging",
                "s3:ObjectOwnerOverrideToBucketOwner",
            ],
        )
    )

    self.s3_bucket.node.default_child.replication_configuration = (
        CfnBucket.ReplicationConfigurationProperty(
            role=replication_role.role_arn,
            rules=[
                CfnBucket.ReplicationRuleProperty(
                    destination=CfnBucket.ReplicationDestinationProperty(
                        bucket=destination_bucket.bucket_arn,
                    ),
                    status="Enabled",
                    prefix="harbor-data-summary/",
                ),
            ],
        )
    )

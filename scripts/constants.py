""" This module has constants throughout the build code """

import aws_cdk.aws_ec2 as ec2

BUCKET_NAME = "SBOMBucket"
CIDR = "10.0.0.0/16"
EC2_INSTANCE_NAME = "DependencyTrack"
EC2_SSH_KEY_NAME = "aquia"
EC2_INSTANCE_AMI = "ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20220131"
EC2_INSTANCE_TYPE = "t2.medium"
STACK_ID = "SBOMApiDeploy"
INGEST_LAMBDA_NAME = "SBOMIngest"
ENRICHMENT_LAMBDA_NAME = "SBOMEnrichmentEntry"
DT_LAMBDA_NAME = "DependencyTrackFunction"
PRIVATE = ec2.SubnetType.PRIVATE_WITH_NAT
PUBLIC = ec2.SubnetType.PUBLIC
REST_API_NAME = "SBOMApi"
PRIVATE_SUBNET_NAME = "SBOMPrivateSubnet"
PUBLIC_SUBNET_NAME = "SBOMPublicSubnet"
VPC_NAME = "SBOMVpc"

EFS_MOUNT_ID = "dtApiStorage"
DT_CONTAINER_ID = "dtContainer"
FARGATE_CLUSTER_ID = "FargateCluster"
DT_TASK_DEF_ID = "dtTaskDefinition"

DT_SBOM_QUEUE_NAME = "DT_SBOMQueue"

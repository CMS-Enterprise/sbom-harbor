""" This module has constants throughout the build code """

import aws_cdk.aws_ec2 as ec2
import aws_cdk.aws_lambda as lambda_

SBOM_API_PYTHON_RUNTIME = lambda_.Runtime.PYTHON_3_9

PRISTINE_SBOM_INGRESS_LN = "PristineSBOMIngressLambda"
SBOM_ENRICHMENT_LN = "SBOMEnrichmentIngressLambda"
DT_INTERFACE_LN = "DependencyTrackInterfaceLambda"
ENRICHMENT_EGRESS_LN = "EnrichmentEgressLambda"

DT_LB_ID = "DEPENDENCY-TRACK-LOAD-BALANCER"
DT_LB_SG_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-SECURITY-GROUP"
DT_LB_LOGGING_ID = "DEPENDENCY-TRACK-LOAD-BALANCER-LOGGING"

PRISTINE_SBOM_INGRESS_API_ID = "pristine-ingress-api"

DT_DOCKER_ID = "dependencytrack/apiserver"
DT_INSTALL_LOC = "/apiserver"

S3_BUCKET_ID = "sbom.bucket.id"
S3_BUCKET_NAME = "sbom.bucket.name"
INGRESS_BUCKET_NAME = f"ingress.{S3_BUCKET_NAME}"
ENRICHMENT_BUCKET_NAME = f"enrichment.{S3_BUCKET_NAME}"

VPC_ID = "sbom.vpc.id"
VPC_NAME = "sbom.vpc.name"

STACK_ID = "SBOMApiStack"
SHARED_RESOURCE_STACK_ID = f"Shared-Resource-{STACK_ID}"
ENRICHMENT_STACK_ID = f"Enrichment-{STACK_ID}"
INGRESS_STACK_ID = f"Ingress-{STACK_ID}"

VPC_TAG_NAME = f"{SHARED_RESOURCE_STACK_ID}/{VPC_NAME}/{VPC_NAME}"

CIDR = "10.0.0.0/16"
EC2_INSTANCE_NAME = "DependencyTrack"
EC2_SSH_KEY_NAME = "aquia"
EC2_INSTANCE_AMI = "ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20220131"
EC2_INSTANCE_TYPE = "t2.medium"
PRIVATE = ec2.SubnetType.PRIVATE_WITH_NAT
PUBLIC = ec2.SubnetType.PUBLIC
REST_API_NAME = "SBOMApi"
PRIVATE_SUBNET_NAME = "SBOMPrivateSubnet"
PUBLIC_SUBNET_NAME = "SBOMPublicSubnet"

EFS_MOUNT_ID = "dtApiStorage"
DT_CONTAINER_ID = "dtContainer"
FARGATE_CLUSTER_ID = "DTFargateCluster"
DT_FARGATE_SVC_NAME = "DTFargateService"
DT_TASK_DEF_ID = "dtTaskDefinition"

DT_SBOM_QUEUE_NAME = "DT_SBOMQueue"
DT_REST_API_GATEWAY = "DT_REST_API_GW"
DT_API_INTEGRATION = "DT_API_INT"

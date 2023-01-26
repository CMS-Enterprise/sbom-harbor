#!/usr/bin/env bash
# set -euxo pipefail

source ./deploy-preamble.sh

echo "
This will DESTROY the $ENVIRONMENT environment!
This includes deleting all objects in S3 buckets, and deleting all cloudformation stacks!

ARE YOU SURE? (Type \"yes\" to acknowledge and continue):"

read answer
if [[ $answer != "yes" ]]; then
  exit
fi

sbomBucket="$ENVIRONMENT-harbor-sbom-uploads-enrichment-$AWS_ACCOUNT_ID-$AWS_REGION_SHORT"
echo "Deleting objects in $sbomBucket"
aws s3 rm s3://$sbomBucket --recursive

assetsBucket="$ENVIRONMENT-harbor-web-assets-$AWS_ACCOUNT_ID-$AWS_REGION_SHORT"
echo "Deleting objects in $assetsBucket"
aws s3 rm s3://$assetsBucket --recursive

dtLogsBucket="$ENVIRONMENT-dt-alb-logs-$AWS_ACCOUNT_ID-$AWS_REGION_SHORT"
echo "Deleting objects in $dtLogsBucket"
aws s3 rm s3://$dtLogsBucket --recursive

ION_CHANNEL_TOKEN="none" cdk destroy --role-arn $CDK_ROLE_ARN --concurrency 6 $ENVIRONMENT-harbor-shared-resources-$AWS_REGION_SHORT $ENVIRONMENT-harbor-user-management-$AWS_REGION_SHORT $ENVIRONMENT-harbor-backend-$AWS_REGION_SHORT $ENVIRONMENT-harbor-frontend-$AWS_REGION_SHORT $ENVIRONMENT-harbor-enrichment-$AWS_REGION_SHORT

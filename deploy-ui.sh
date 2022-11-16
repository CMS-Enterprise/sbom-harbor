#!/usr/bin/env bash
# set -euxo pipefail
source ./deploy-preamble.sh

# get cloudformation values
export USER_POOL=$(aws cloudformation describe-stacks --stack-name "${ENVIRONMENT}-harbor-user-management-${AWS_REGION_SHORT}" --query 'Stacks[0].Outputs')
# extract specific values with jq
export USER_POOL_ID=$(echo $USER_POOL | jq -r '.[] | select(.OutputKey|test("ExportsOutputRefCognitoUserPool"))| .OutputValue')
export USER_POOL_CLIENT_ID=$(echo $USER_POOL | jq -r '.[] | select(.OutputKey|test("ExportsOutputRefUserPoolAppClient"))| .OutputValue')

FRONTEND=$(aws cloudformation describe-stacks --stack-name "${ENVIRONMENT}-harbor-frontend-${AWS_REGION_SHORT}" --query 'Stacks[0].Outputs')
ASSETS_BUCKET=$(echo $FRONTEND | jq -r '.[] | select(.OutputKey=="WebAssetsBucketName") | .OutputValue')
export CF_DOMAIN=$(echo $FRONTEND | jq -r '.[] | select(.OutputKey=="CloudFrontDomain") | .OutputValue')

cd ui
rm -rf ./packages/sbom/build
yarn install
yarn build

aws s3 sync ./packages/sbom/build s3://${ASSETS_BUCKET}

cd ..

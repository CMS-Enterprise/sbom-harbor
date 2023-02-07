#!/usr/bin/env bash

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1

source ./deploy-preamble.sh

# get cloudformation values for the cognito user pool
export USER_POOL="$(aws cloudformation describe-stacks --stack-name ${ENVIRONMENT}-harbor-user-management-${AWS_REGION_SHORT} --query 'Stacks[0].Outputs')"

# extract specific values with jq
export USER_POOL_ID="$(echo ${USER_POOL} | jq -r '.[] | select(.OutputKey|test("ExportsOutputRefCognitoUserPool"))| .OutputValue')"
export USER_POOL_CLIENT_ID="$(echo ${USER_POOL} | jq -r '.[] | select(.OutputKey|test("ExportsOutputRefUserPoolAppClient"))| .OutputValue')"

# get cloudformation values for the frontend
FRONTEND="$(aws cloudformation describe-stacks --stack-name ${ENVIRONMENT}-harbor-frontend-${AWS_REGION_SHORT} --query 'Stacks[0].Outputs')"
export ASSETS_BUCKET="$(echo ${FRONTEND} | jq -r '.[] | select(.OutputKey=="WebAssetsBucketName") | .OutputValue')"
export CF_DOMAIN="$(echo ${FRONTEND} | jq -r '.[] | select(.OutputKey=="CloudFrontDomain") | .OutputValue')"

# print out the settings
echo "Configured SBOM Harbor UI with the following settings:

    UI Configuration:
    ------------------------------------------
    ASSETS_BUCKET: ${ASSETS_BUCKET}
    CF_DOMAIN: ${CF_DOMAIN}
    USER_POOL_ID: ${USER_POOL_ID}
    USER_POOL_CLIENT_ID: ${USER_POOL_CLIENT_ID}
"

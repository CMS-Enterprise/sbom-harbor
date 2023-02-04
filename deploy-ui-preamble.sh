#!/usr/bin/env bash
source ./deploy-preamble.sh

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

# get cloudformation values
export USER_POOL="$(aws cloudformation describe-stacks --stack-name ${ENVIRONMENT}-harbor-user-management-${AWS_REGION_SHORT} --query 'Stacks[0].Outputs')"

# extract specific values with jq
export USER_POOL_ID="$(echo ${USER_POOL} | jq -r '.[] | select(.OutputKey|test("ExportsOutputRefCognitoUserPool"))| .OutputValue')"
export USER_POOL_CLIENT_ID="$(echo ${USER_POOL} | jq -r '.[] | select(.OutputKey|test("ExportsOutputRefUserPoolAppClient"))| .OutputValue')"

FRONTEND="$(aws cloudformation describe-stacks --stack-name ${ENVIRONMENT}-harbor-frontend-${AWS_REGION_SHORT} --query 'Stacks[0].Outputs')"
ASSETS_BUCKET="$(echo ${FRONTEND} | jq -r '.[] | select(.OutputKey=="WebAssetsBucketName") | .OutputValue')"
export CF_DOMAIN="$(echo ${FRONTEND} | jq -r '.[] | select(.OutputKey=="CloudFrontDomain") | .OutputValue')"

echo "\nReady to deploy SBOM Harbor UI with the following settings:

    AWS Configuration:
    ------------------------------------------
    PROFILE: ${AWS_PROFILE}
    USER: ${AWS_USER_ID}
    BRANCH: ${BRANCH}
    ENVIRONMENT: ${ENVIRONMENT}
    REGION: ${AWS_REGION}
    ACCOUNT: ${AWS_ACCOUNT_ID}
    CDK ROLE: ${CDK_ROLE_ARN}

    UI Configuration:
    ------------------------------------------
    CF_DOMAIN: ${CF_DOMAIN}
    USER_POOL_ID: ${USER_POOL_ID}
    USER_POOL_CLIENT_ID: ${USER_POOL_CLIENT_ID}
"

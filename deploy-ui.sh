#!/usr/bin/env bash

source ./deploy-preamble.sh

USER_POOL_ID=$(jq -r '."'"${AWS_VAULT}-harbor-user-management-${AWS_REGION_SHORT}"'".UserPoolID' < ${OUTPUTS})
USER_POOL_CLIENT_ID=$(jq -r '."'"${AWS_VAULT}-harbor-user-management-${AWS_REGION_SHORT}"'".UserPoolClientID' < ${OUTPUTS})
CF_DOMAIN=$(jq -r '."'"${AWS_VAULT}-harbor-frontend-${AWS_REGION_SHORT}"'".CloudFrontDomain' < ${OUTPUTS})
WEB_ASSETS_BUCKET=$(jq -r '."'"${AWS_VAULT}-harbor-frontend-${AWS_REGION_SHORT}"'".WebAssetsBucketName' < ${OUTPUTS})

cd ui
rm -rf ./packages/sbom/build
yarn install
yarn build
aws s3 sync ./packages/sbom/build s3://${WEB_ASSETS_BUCKET}

cd ..

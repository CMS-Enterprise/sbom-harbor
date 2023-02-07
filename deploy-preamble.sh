#!/usr/bin/env bash

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd

if [[ -z $AWS_REGION ]]; then
  export AWS_REGION=$(aws configure get region --output text)
fi

if [[ -z $AWS_DEFAULT_REGION ]]; then
  export AWS_DEFAULT_REGION="${AWS_REGION}"
fi

if [[ -z $AWS_PROFILE ]]; then
  export AWS_PROFILE="default"
fi

if [[ -z $ENVIRONMENT ]]; then
  # derive environment name from branch
  export BRANCH=$(git rev-parse --abbrev-ref HEAD)
  env=$(echo ${BRANCH} | awk '{split($0, a, "/"); print tolower(a[1])}')
  # ispgcasp- ends up making some aws resource names too long, replace with "e" (for "ephemeral")
  export ENVIRONMENT="${env/ispgcasp-/e}"
fi

# ION_CHANNEL vars are a temporary shim until the client pulls from secrets manager
if [[ -z ION_CHANNEL_TOKEN ]]; then
  export ION_CHANNEL_TOKEN="IonChannelDummyToken"
fi

# if [[ -z ION_CHANNEL_TEAM_ID ]]; then
#   export ION_CHANNEL_TEAM_ID="none"
# fi

# unset any existing environment variable values
unset \
  AWS_REGION_SHORT \
  AWS_ACCOUNT_ID \
  AWS_USER_ID \
  CDK_ROLE_ARN \
  USER_POOL \
  USER_POOL_ID \
  USER_POOL_CLIENT_ID \
  ASSETS_BUCKET \
  CF_DOMAIN

# get the short name for the AWS region
case "${AWS_REGION}" in
  "us-east-2") export AWS_REGION_SHORT="use2";;
  "us-east-1") export AWS_REGION_SHORT="use1";;
  "us-west-1") export AWS_REGION_SHORT="usw1";;
  "us-west-2") export AWS_REGION_SHORT="usw2";;
esac

# get AWS account ID, user ID, and CDK role ARN
CALLER_IDENTITY=$(aws sts get-caller-identity)
export AWS_ACCOUNT_ID=$(echo ${CALLER_IDENTITY} | jq -r '.Account')
export AWS_USER_ID=$(echo ${CALLER_IDENTITY} | jq -r '.UserId')
export CDK_ROLE_ARN="arn:aws:iam::${AWS_ACCOUNT_ID}:role/delegatedadmin/developer/cdk-hnb659fds-cfn-exec-role-${AWS_ACCOUNT_ID}-us-east-1"

echo "Ready to deploy SBOM Harbor application with the following settings:

    AWS Configuration:
    ------------------------------------------
    PROFILE: ${AWS_PROFILE}
    USER: ${AWS_USER_ID}
    BRANCH: ${BRANCH}
    ENVIRONMENT: ${ENVIRONMENT}
    REGION: ${AWS_REGION}
    ACCOUNT: ${AWS_ACCOUNT_ID}
    CDK ROLE: ${CDK_ROLE_ARN}
"

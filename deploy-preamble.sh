#!/usr/bin/env bash

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd

echo "HERE 0"

declare -A regionShortCodes
regionShortCodes[us-east-1]="use1"
regionShortCodes[us-east-2]="use2"
regionShortCodes[us-west-1]="usw1"
regionShortCodes[us-west-2]="usw2"

if [[ -z $AWS_REGION ]]; then
  export AWS_REGION=$(aws configure get region --output text)
fi

if [[ -z $AWS_DEFAULT_REGION ]]; then
  export AWS_DEFAULT_REGION=$AWS_REGION
fi

if [[ -z $AWS_PROFILE ]]; then 
  export AWS_PROFILE="default"
fi

if [[ -z $ENVIRONMENT ]]; then
  # derive environment name from branch
  export BRANCH=$(git rev-parse --abbrev-ref HEAD)
  env=$(echo $BRANCH | awk '{split($0, a, "/"); print tolower(a[1])}')
  # ispgcasp- ends up making some aws resource names too long, replace with "e" (for "ephemeral")
  export ENVIRONMENT="${env/ispgcasp-/e}"
fi

# ION_CHANNEL vars are a temporary shim until the client pulls from secrets manager
if [[ -z ION_CHANNEL_TOKEN ]]; then
  export ION_CHANNEL_TOKEN="IonChannelDummyToken"
fi

echo "HERE 3"
# if [[ -z ION_CHANNEL_TEAM_ID ]]; then
#   export ION_CHANNEL_TEAM_ID="none"
# fi

export AWS_REGION_SHORT=${regionShortCodes[$AWS_REGION]}
CALLER_IDENTITY=$(aws sts get-caller-identity)
export AWS_ACCOUNT_ID=$(echo "$CALLER_IDENTITY" | jq -r '.Account')
export AWS_USER_ID=$(echo "$CALLER_IDENTITY" | jq -r '.UserId')
export CDK_ROLE_ARN="arn:aws:iam::$AWS_ACCOUNT_ID:role/delegatedadmin/developer/cdk-hnb659fds-cfn-exec-role-$AWS_ACCOUNT_ID-us-east-1"


echo "HERE 4"
echo "Deploying SBOM Harbor application with the following settings:
    PROFILE: $AWS_PROFILE
    USER: $AWS_USER_ID
    BRANCH: $BRANCH
    ENVIRONMENT: $ENVIRONMENT
    REGION: $AWS_REGION
    ACCOUNT: $AWS_ACCOUNT_ID
    CDK ROLE: $CDK_ROLE_ARN"

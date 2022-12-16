#!/usr/bin/env bash

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd

declare -A regionShortCodes
regionShortCodes[us-east-1]="use1"
regionShortCodes[us-east-2]="use2"
regionShortCodes[us-west-1]="usw1"
regionShortCodes[us-west-2]="usw2"


if [[ -z $AWS_REGION ]]; then
  REGION=$(aws configure get region --output text)
  export AWS_REGION=$REGION
fi

if [[ -z $AWS_DEFAULT_REGION ]]; then
  export AWS_DEFAULT_REGION=$REGION
fi

if [[ -z $ENVIRONMENT ]]; then
  export ENVIRONMENT="sandbox"
fi

if [[ -z $AWS_PROFILE ]]; then 
  export AWS_PROFILE="default"
fi

export AWS_REGION_SHORT=${regionShortCodes[$AWS_REGION]}
export AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query 'Account' --output text)

if [[ $AWS_PROFILE == "cms-dev" || $AWS_PROFILE == "cms-prod" ]]; then
  export CDK_ROLE_ARN="--role-arn arn:aws:iam::${AWS_ACCOUNT_ID}:role/delegatedadmin/developer/cdk-hnb659fds-cfn-exec-role-${AWS_ACCOUNT_ID}-us-east-1"
else
  export CDK_ROLE_ARN=""
fi

echo "Deploying SBOM Harbor application with the following settings:
    PROFILE: $AWS_PROFILE
    ENVIRONMENT: $ENVIRONMENT
    REGION: $AWS_REGION
    ACCOUNT: $AWS_ACCOUNT_ID
    CDK ROLE: $CDK_ROLE_ARN"

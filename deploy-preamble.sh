#!/usr/bin/env bash

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd 

declare -A regionShortCodes
regionShortCodes[us-east-1]="use1"
regionShortCodes[us-east-2]="use2"
regionShortCodes[us-west-1]="usw1"
regionShortCodes[us-west-2]="usw2"


if [[ -z "${AWS_REGION}" ]]
then
  REGION=$(aws configure get region --output text)
  export AWS_REGION=$REGION
fi

if [[ -z "${ENVIRONMENT}" ]]
then
  ENV="sandbox"
  export ENVIRONMENT=ENV
fi

if [ $AWS_PROFILE = "default" ]; then PROFILE="sandbox"; else PROFILE=$AWS_PROFILE; fi;

AWS_REGION_SHORT=${regionShortCodes[$AWS_REGION]}
AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query 'Account' --output text)

echo "Using profile $PROFILE to deploy $ENVIRONMENT environment into $AWS_REGION for account $AWS_ACCOUNT_ID"

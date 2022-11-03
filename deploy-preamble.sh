#!/usr/bin/env bash

# DO NOT EXECUTE THIS DIRECTLY!
# It is intended to be sourced by other deploy scripts

cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd 

declare -A regionShortCodes
regionShortCodes[us-east-1]="use1"
regionShortCodes[us-east-2]="use2"
regionShortCodes[us-west-1]="usw1"
regionShortCodes[us-west-2]="usw2"

AWS_REGION_SHORT=${regionShortCodes[$AWS_REGION]}

OUTPUTS="cdk-outputs-${AWS_VAULT}-${AWS_REGION_SHORT}.json"

echo "Deploying into $AWS_VAULT $AWS_REGION and saving results to ${OUTPUTS}"

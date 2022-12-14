#!/usr/bin/env bash
source ./deploy-preamble.sh

API_GW_ID=$(aws cloudformation describe-stacks --stack-name "$ENVIRONMENT-harbor-backend-$AWS_REGION_SHORT" --query 'Stacks[0].Outputs[?OutputKey==`apigwid`].OutputValue' --output text)
CF_DOMAIN=$(aws cloudformation describe-stacks --stack-name "$ENVIRONMENT-harbor-frontend-$AWS_REGION_SHORT" --query 'Stacks[0].Outputs[?OutputKey==`CloudFrontDomain`].OutputValue' --output text)

aws apigatewayv2 export-api --api-id $API_GW_ID --stage-name '$default' --output-type JSON --specification OAS30 ./harbor-documentation/tmp.json

cat ./harbor-documentation/tmp.json | jq ".servers[0].url=\"https://$CF_DOMAIN/{basePath}\"" > ./harbor-documentation/openapi.json
rm ./harbor-documentation/tmp.json

aws s3 sync ./harbor-documentation s3://$ENVIRONMENT-harbor-web-assets-$AWS_ACCOUNT_ID-$AWS_REGION_SHORT/docs/ --delete

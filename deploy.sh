#!/usr/bin/env bash

source ./deploy-preamble.sh

ENRICHMENTS=""
BACKEND=""
UI=0

while getopts "eu" arg; do
  case "${arg}" in
    e) ENRICHMENTS="${AWS_VAULT}-harbor-enrichment-${AWS_REGION_SHORT}"
       ;;
    u) UI=1
       ;;
    ?)
      echo "Invalid option: -${OPTARG}."
      echo
      usage
      ;;
  esac
done

AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query 'Account' --output text)

poetry run clean
poetry -q install
poetry build
pip install --upgrade -t tmp dist/*.whl
cd ./tmp
zip -q -r ../dist/lambda.zip . -x '*.pyc'
cd ..

cdk deploy --require-approval never --concurrency 5 --outputs-file ${OUTPUTS} ${AWS_VAULT}-harbor-shared-resources-${AWS_REGION_SHORT} ${AWS_VAULT}-harbor-user-management-${AWS_REGION_SHORT} ${AWS_VAULT}-harbor-backend-${AWS_REGION_SHORT} ${AWS_VAULT}-harbor-frontend-${AWS_REGION_SHORT} {$ENRICHMENTS}

jq -r < ${OUTPUTS}

if [ $UI -eq 1 ]; then
  ./deploy-ui.sh
fi

#!/usr/bin/env bash
set -eo pipefail

source ./deploy-preamble.sh

ENRICHMENTS=""
UI=0
DEPLOY_ONLY=0
PILOT=0

while getopts "eudp" arg; do
  case "${arg}" in
    e) ENRICHMENTS="$ENVIRONMENT-harbor-enrichment-$AWS_REGION_SHORT"
       ;;
    u) UI=1
       ;;
    d) DEPLOY_ONLY=1
       ;;
    p) PILOT=1
       ;;
    ?)
      echo "Invalid option: -$OPTARG"
      ;;
  esac
done

echo "    ENRICHMENTS: $ENRICHMENTS
    UI: $UI
    PILOT: $PILOT
    DEPLOY ONLY: $DEPLOY_ONLY
    "

if [[ $DEPLOY_ONLY == 0 ]]; then
  poetry run clean > /dev/null 2>&1
  poetry -q install
  poetry build
  pip install --upgrade -t tmp dist/*.whl
  cd ./tmp
  zip -q -r ../dist/lambda.zip . -x '*.pyc'
  cd ..
fi

cdk deploy --role-arn $CDK_ROLE_ARN --require-approval never --concurrency 6 $ENVIRONMENT-harbor-shared-resources-$AWS_REGION_SHORT 

# ./upload-swagger-docs.sh

if [[ $UI == 1 ]]; then
  ./deploy-ui.sh
fi

if [[ $PILOT == 1 ]]; then
  ./deploy-pilot.sh
fi

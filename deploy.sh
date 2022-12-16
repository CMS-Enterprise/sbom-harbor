#!/usr/bin/env bash
set -eo pipefail

source ./deploy-preamble.sh

ENRICHMENTS=""
UI=0
DEPLOY_ONLY=0

while getopts "eud" arg; do
  case "${arg}" in
    e) ENRICHMENTS="$ENVIRONMENT-harbor-enrichment-$AWS_REGION_SHORT"
       ;;
    u) UI=1
       ;;
    d) DEPLOY_ONLY=1
       ;;
    ?)
      echo "Invalid option: -$OPTARG"
      ;;
  esac
done

echo "    ENRICHMENTS: $ENRICHMENTS
    UI: $UI
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

cdk deploy $CDK_ROLE_ARN --require-approval never --concurrency 5 $ENVIRONMENT-harbor-shared-resources-$AWS_REGION_SHORT $ENVIRONMENT-harbor-user-management-$AWS_REGION_SHORT $ENVIRONMENT-harbor-backend-$AWS_REGION_SHORT $ENVIRONMENT-harbor-frontend-$AWS_REGION_SHORT $ENRICHMENTS

./upload-swagger-docs.sh

if [ $UI -eq 1 ]; then
  ./deploy-ui.sh
fi


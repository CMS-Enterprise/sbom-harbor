#!/usr/bin/env bash
# set -euxo pipefail
source ./deploy-preamble.sh

ENRICHMENTS=""
BACKEND=""
UI=0

while getopts "eu" arg; do
  case "${arg}" in
    e) ENRICHMENTS="${ENVIRONMENT}-harbor-enrichment-${AWS_REGION_SHORT}"
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

poetry run clean > /dev/null 2>&1
poetry -q install
poetry build
pip install --upgrade -t tmp dist/*.whl
cd ./tmp
zip -q -r ../dist/lambda.zip . -x '*.pyc'
cd ..

cdk deploy --require-approval never --concurrency 5 ${ENVIRONMENT}-harbor-shared-resources-${AWS_REGION_SHORT} ${ENVIRONMENT}-harbor-user-management-${AWS_REGION_SHORT} ${ENVIRONMENT}-harbor-backend-${AWS_REGION_SHORT} ${ENVIRONMENT}-harbor-frontend-${AWS_REGION_SHORT} {$ENRICHMENTS}

if [ $UI -eq 1 ]; then
  ./deploy-ui.sh
fi

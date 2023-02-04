#!/usr/bin/env bash
# set -euxo pipefail

source ./deploy-preamble.sh

echo "\nDeploying Harbor Pilot...\n"

cd harbor-rs

./pilot-build.sh

docker build -t harbor/pilot:latest -f Dockerfile.pilot .

cd ..

cdk deploy --require-approval never "${ENVIRONMENT}-harbor-pilot-${AWS_REGION_SHORT}"

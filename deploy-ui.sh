#!/usr/bin/env bash
# set -euxo pipefail
source ./deploy-ui-preamble.sh

echo "\Deploying Harbor UI...\n"

cd ui
rm -rf ./packages/sbom/build
yarn install
yarn build

aws s3 sync ./packages/sbom/build s3://${ASSETS_BUCKET}

cd ..

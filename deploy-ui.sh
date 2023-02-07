#!/usr/bin/env bash
source ./deploy-ui-preamble.sh

echo "Building Harbor UI..."

yarn --cwd ui clean
yarn --cwd ui install
yarn --cwd ui build

echo "Deploying Harbor UI to \"s3://${ASSETS_BUCKET}\"..."

aws s3 sync ui/packages/sbom/build "s3://${ASSETS_BUCKET}"

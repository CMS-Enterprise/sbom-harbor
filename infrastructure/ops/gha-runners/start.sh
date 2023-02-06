#!/bin/bash
set -e

mkdir actions-runner && cd actions-runner

curl -o actions-runner-linux-x64-2.301.1.tar.gz -L https://github.com/actions/runner/releases/download/v2.301.1/actions-runner-linux-x64-2.301.1.tar.gz

echo "3ee9c3b83de642f919912e0594ee2601835518827da785d034c1163f8efdf907  actions-runner-linux-x64-2.301.1.tar.gz" | shasum -a 256 -c

tar xzf ./actions-runner-linux-x64-2.301.1.tar.gz

# Get Token
# TODO: generate token once we have the ability to create PATs in github enterprise
# token=$(curl -s -XPOST -H "authorization: token ${GITHUB_TOKEN}" https://api.github.com/repos/CMS-Enterprise/sbom-harbor/actions/runners/registration-token | jq -r .token)

./config.sh --url https://github.com/CMS-Enterprise/sbom-harbor --token $GITHUB_TOKEN --labels $ENVIRONMENT

./run.sh

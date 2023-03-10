#!/usr/bin/env bash
# Used to generate client code or tests from the openapi spec.yaml file found in this directory.
# All paths are relative so this script must be run from the openapi directory.
set -eo pipefail

TESTS=0
CLIENT=0
SERVER=0
MODELS=0

VERSION="v6.2.1"
DOCKER_IMAGE="openapitools/openapi-generator-cli:${VERSION}"
GENERATE="docker run --rm --volume "$(pwd):/local" ${DOCKER_IMAGE} batch --clean"

while getopts "tcsm" arg; do
  case "${arg}" in
    t) TESTS=1
       ;;
    c) CLIENT=1
       ;;
    s) SERVER=1
       ;;
    m) MODELS=1
       ;;
    ?)
      echo "Invalid option: -$OPTARG"
      ;;
  esac
done

echo "
    TESTS: $TESTS
    CLIENT: $CLIENT
    MODELS: $MODELS
    "

if [[ $TESTS == 1 ]]; then
    echo "Generating Tests"
    $GENERATE /local/tests/config.yaml
fi

if [[ $CLIENT == 1 ]]; then
  echo "Generating API Client"
  $GENERATE /local/client/config.yaml
fi

if [[ $MODELS == 1 ]]; then
  echo "Generating Models"
  $GENERATE /local/models/config.yaml
fi

#!/usr/bin/env bash
# Used to generate clients or models from 3rd-party openapi specs.
# All paths are relative so this script must be run from this directory.
set -eo pipefail

CYCLONE_DX=0
SNYK=0

VERSION="v6.2.1"
DOCKER_IMAGE="openapitools/openapi-generator-cli:${VERSION}"
GENERATE="docker run --rm --volume "$(pwd):/local" ${DOCKER_IMAGE} batch --clean"

while getopts "cs" arg; do
  case "${arg}" in
    c) CYCLONE_DX=1
       ;;
    s) SNYK=1
       ;;
    ?)
      echo "Invalid option: -$OPTARG"
      ;;
  esac
done

echo "
    CYCLONE_DX: $CYCLONE_DX
    SNYK: $SNYK
    "

if [[ $CYCLONE_DX == 1 ]]; then
  echo "Generating CycloneDx Models..."
  $GENERATE /local/cyclonedx-config.yaml

  echo "Removing unused assets..."
  rm -rf generated/cyclonedx/.openapi-generator
  rm -rf generated/cyclonedx/docs
  rm -rf generated/cyclonedx/.gitignore
  rm -rf generated/cyclonedx/.openapi-generator-ignore
  rm -rf generated/cyclonedx/.travis.yml
  rm -rf generated/cyclonedx/Cargo.toml
  rm -rf generated/cyclonedx/git_push.sh
  rm -rf generated/cyclonedx/README.md
  rm -rf generated/cyclonedx/src/apis
  rm -rf generated/cyclonedx/src/models/_api_v2_sbom_post_request.rs
  rm -rf generated/cyclonedx/src/lib.rs

  echo "Fixing generated names..."
  mv generated/cyclonedx/src/models/cyclonedx_1_period_4.rs generated/cyclonedx/src/models/bom
fi

if [[ $SNYK == 1 ]]; then
  echo "Generating Snyk Models..."
  $GENERATE /local/snyk-config.yaml

  echo "Removing unused assets..."
  rm -rf generated/snyk/.openapi-generator
  rm -rf generated/snyk/docs
  rm -rf generated/snyk/.gitignore
  rm -rf generated/snyk/.openapi-generator-ignore
  rm -rf generated/snyk/.travis.yml
  rm -rf generated/snyk/Cargo.toml
  rm -rf generated/snyk/git_push.sh
  rm -rf generated/snyk/README.md
  rm -rf generated/snyk/src/apis/apps_api.rs
  rm -rf generated/snyk/src/apis/iac_settings_api.rs
  rm -rf generated/snyk/src/apis/invites_api.rs
  rm -rf generated/snyk/src/apis/open_api_api.rs
  rm -rf generated/snyk/src/lib.rs
fi

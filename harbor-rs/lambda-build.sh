#!/usr/bin/env bash
# set -euxo pipefail

LAMBDA=$1

if [[ -z "$LAMBDA" ]]; then
  echo "Lambda name required"
  exit 1
fi

RUN=0

while getopts "r" arg; do
  case "${arg}" in
    r) RUN=1
       ;;
    ?)
      echo "Invalid option: -$OPTARG"
      ;;
  esac
done

echo "RUN: ${RUN}"

# Clean previous release
echo "==> cleaning previous release"
cargo clean --release

# Prefetch dependencies
echo "==> fetching dependencies"
cargo fetch --target x86_64-unknown-linux-musl

# Create the builder image
echo "==> creating builder image"
BIN=$LAMBDA docker build -t harbor/lambda-builder -f Dockerfile.lambda-builder .

# Compile the binary
echo "==> compiling lambda binary"
docker run \
    --platform linux/amd64 \
    -u "$(id -u)":"$(id -g)" \
    -v ${PWD}:/code \
    -v ${PWD}/iron-rs:/code/iron-rs \
    -v ${HOME}/.cargo/registry:/cargo/registry \
    -v ${HOME}/.cargo/git:/cargo/git \
    harbor/lambda-builder

# Build the lambda image
echo "==> building lambda image"
docker build -t harbor/${LAMBDA}:latest -f Dockerfile.lambda .

if [[ $RUN == 1 ]]; then
  echo "==> running lambda container"
  # Note that as of now this requires command line credentials.
  # We need a way to get runtime configuration dynamically from
  # within the lambda both for local development and final deployment.
  docker run --rm \
    --platform linux/amd64 \
    --name $LAMBDA \
    --rm \
    -e AWS_REGION=$AWS_REGION \
    -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID \
    -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY \
    -e AWS_SESSION_TOKEN=$AWS_SESSION_TOKEN \
    -p 9000:8080 \
    -d harbor/$LAMBDA:latest
fi

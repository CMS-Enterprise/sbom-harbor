#!/usr/bin/env bash
# set -euxo pipefail
source ../deploy-preamble.sh

cd ./harbor-rs

# Clean previous release
cargo clean --release

# Prefetch dependencies
cargo fetch --target x86_64-unknown-linux-musl

# Create the builder image
docker build -t harbor/pilot-builder -f Dockerfile.pilot-builder .

# Compile the binary
docker run \
    --platform linux/amd64 \
    -u "$(id -u)":"$(id -g)" \
    -v ${PWD}:/code \
    -v ${PWD}/iron-rs:/code/iron-rs \
    -v ${HOME}/.cargo/registry:/cargo/registry \
    -v ${HOME}/.cargo/git:/cargo/git \
    harbor/pilot-builder

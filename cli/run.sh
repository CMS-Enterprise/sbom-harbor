#!/usr/bin/env bash

DI_NAME=test-mongo
docker stop $DI_NAME
docker rm $DI_NAME
docker run -d --name $DI_NAME \
  -e MONGO_INITDB_ROOT_USERNAME=root \
  -e MONGO_INITDB_ROOT_PASSWORD=harbor \
  --network="host" -p 27017:27017 mongo:5.0.15

cargo build && ../target/debug/main --provider github

#!/bin/bash

CLEAR_MONGO="cm"

DI_NAME=test-mongo
docker stop $DI_NAME
docker rm $DI_NAME
docker run -d --name $DI_NAME --network="host" -p 27017:27017 mongo:5.0.15

clear
cargo build && ../target/debug/main --account 123456  --env none start

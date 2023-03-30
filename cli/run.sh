#!/usr/bin/env bash

DI_NAME=test-mongo
docker stop $DI_NAME
docker rm $DI_NAME
docker run -d --name $DI_NAME \
  -e MONGO_INITDB_ROOT_USERNAME=root \
  -e MONGO_INITDB_ROOT_PASSWORD=harbor \
  --network="host" -p 27017:27017 mongo:5.0.15

#echo "Creating mongo users..."
#mongo admin --host localhost \
#  -u USER_PREVIOUSLY_DEFINED \
#  -p PASS_YOU_PREVIOUSLY_DEFINED \
#  --eval "db.createUser({user: 'root', pwd: 'harbor', roles: [{role: 'readWrite', db: 'harbor'}]});"
#echo "Mongo users created."

clear
cargo build && ../target/debug/main --account 123456  --env none start

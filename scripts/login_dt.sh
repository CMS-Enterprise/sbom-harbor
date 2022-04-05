#!/bin/bash

curl \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -H 'Accept: text/plain' \
  -X POST \
  http://localhost:8081/api/v1/user/login \
  -d '{ "username": "admin", "password": "admin" }'

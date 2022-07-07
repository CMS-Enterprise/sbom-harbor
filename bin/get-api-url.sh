#!/bin/bash

get_domain() {
  domain=$(
    aws apigatewayv2 get-apis --output json \
    | jq '.Items|.[0]|.ApiEndpoint' \
    | sed -e 's/"//g'
  )
  echo "$domain"
}

echo "$(get_domain)"

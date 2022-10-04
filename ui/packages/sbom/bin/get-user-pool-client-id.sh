#!/bin/bash

get_user_pool_web_client_id() {
  local user_pool=$(aws cognito-idp list-user-pools --max-results 1 --output json | jq '.UserPools|.[0]|.Id' | sed -e 's/"//g')
  local user_pool_client=$(aws cognito-idp list-user-pool-clients \
    --user-pool-id "${user_pool}" \
    --max-results 1 --output json \
    | jq '.UserPoolClients|.[0]|.ClientId' \
    | sed -e 's/"//g'
  )
  echo "$user_pool_client"
}

COGNITO_USER_POOL_CLIENT_ID="$(get_user_pool_web_client_id)"

echo "${COGNITO_USER_POOL_CLIENT_ID}"

#!/bin/bash

get_user_pool_id() {
  local user_pool=$(aws cognito-idp list-user-pools --max-results 1 --output json)
  echo $(echo $user_pool | jq '.UserPools|.[0]|.Id' | sed -e 's/"//g')
}

echo "$(get_user_pool_id)"

#!/bin/bash


function strip_quotes() {
  local str_with_quotes=$1
  echo ${str_with_quotes} | sed -e 's/"//g'
}

cidp="aws cognito-idp"
username=sbomadmin
email=${username}@aquia.us
user_pool_id=$( ${cidp} list-user-pools --max-results 1 | jq '.UserPools|.[0]|.Id')
user_pool_id=$(strip_quotes ${user_pool_id})

echo User Pool Id: ${user_pool_id}

create_user_resp=$(${cidp} admin-create-user  \
  --user-pool-id ${user_pool_id} \
  --username ${email} \
  --desired-delivery-mediums EMAIL \
  --user-attributes Name=email,Value=${email})

user_name=$(echo ${create_user_resp} | jq '.User|.Username')

echo Username: ${user_name}

${cidp} admin-set-user-password \
  --user-pool-id $( strip_quotes ${user_pool_id} )  \
  --username $( strip_quotes ${user_name} ) \
  --password "L0g1nTe5tP@55!" \
  --permanent


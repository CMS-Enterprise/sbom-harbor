#!/bin/bash

get_user_pool() {
  echo $(aws cognito-idp list-user-pools --max-results 1 --output json)
}

get_user_pool_id() {
  local user_pool=$(aws cognito-idp list-user-pools --max-results 1 --output json)
  echo $(echo $user_pool | jq '.UserPools|.[0]|.Id' | sed -e 's/"//g')
}

# Creates a new user in the user pool
create_user() {
  if [ "$#" != 1 ]; then
    echo "Incorrect number of arguments." && exit 1
  fi

  local user_pool_id=$(get_user_pool_id)

  aws cognito-idp admin-create-user  \
    --user-pool-id ${USER_POOL_ID} \
    --username "$1" \
    --desired-delivery-mediums EMAIL \
    --user-attributes Name=email,Value="$1"
}

# Changes the password of a user in the user pool
# Params:
#   $1: username - the email address of the user
#   $2: password - the new password for the user
set_user_password() {
  if [ "$#" != 2 ]; then
    echo "Incorrect number of arguments" && exit 1
  fi

  local user_pool_id=$(get_user_pool_id)

  aws cognito-idp admin-set-user-password \
    --user-pool-id "${USER_POOL_ID}"  \
    --username "${ADMIN_USERNAME}" \
    --password "${ADMIN_PASSWORD}" \
    --permanent
}

update_user() {
  aws cognito-idp admin-update-user-attributes \
    --user-pool-id ${USER_POOL_ID} \
    --username ${ADMIN_USERNAME} \
    --user-attributes \
      Name="custom:teams",Value="d54ad85a-249e-4da2-9735-6bcfb09244f3\,4a434ea8-dbcd-430f-8a15-0d8f851c9751"
}

setup_admin_user() {
  USER_POOL_ID=$(aws cognito-idp list-user-pools --max-results 1 --output json | jq '.UserPools|.[0]|.Id' | sed -e 's/"//g')

  aws cognito-idp admin-create-user  \
    --user-pool-id ${USER_POOL_ID} \
    --username "${ADMIN_USERNAME}" \
    --desired-delivery-mediums EMAIL \
    --user-attributes \
      Name='email',Value="${ADMIN_USERNAME}" \
      Name='name',Value="Admin User"

  aws cognito-idp admin-set-user-password \
    --user-pool-id "${USER_POOL_ID}"  \
    --username "${ADMIN_USERNAME}" \
    --password "${ADMIN_PASSWORD}" \
    --permanent
}

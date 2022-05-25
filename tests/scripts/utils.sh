#!/bin/bash

get_user_pool() {
  echo $(aws cognito-idp list-user-pools --max-results 1 --output json)
}

get_user_pool_id() {
  echo $(get_user_pool | jq '.UserPools|.[0]|.Id' | sed -e 's/"//g')
}

# Creates a new user in the user pool
create_user() {
  if [ "$#" != 1 ]; then
    echo "Incorrect number of arguments." && exit 1
  fi

  local user_pool_id=$(get_user_pool_id)

  aws cognito-idp admin-create-user  \
    --user-pool-id ${COGNITO_USER_POOL_ID} \
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
    --user-pool-id "${COGNITO_USER_POOL_ID}"  \
    --username "$1" \
    --password "$2" \
    --permanent --debug
}

#******************************************************************************
#* Set environment variables whenever this file is sourced in another         *
#******************************************************************************

_concat_paths() {
  base_path=${1}
  sub_path=${2}
  full_path="${base_path:+$base_path/}$sub_path"
  real_path=$(realpath ${full_path})
  echo $real_path
}

# Get the path of the .env file
dotenv_path=$(_concat_paths "$pwd" "$(dirname $0)/../../.env")

# If the .env file does not exist, error out
if [ ! -f "$dotenv_path" ]; then
  echo "ERROR: $dotenv_path does not exist."
  exit 1
fi

# Source environment variables from .env file
source "$dotenv_path"

# Get the User Pool ID and export it as an environment variable
export COGNITO_USER_POOL_ID=$(get_user_pool_id)

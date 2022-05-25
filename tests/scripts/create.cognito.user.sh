#!/bin/bash

source "$(dirname $(realpath "$BASH_SOURCE"))/utils.sh"

for user_name in ${COGNITO_USERNAMES};
do
  email_username="${user_name}@aquia.io"
  create_user_resp=$(create_user "${email_username}")

  if [ $? -ne 0 ]; then
    echo "\nERROR: Failed to create user."
    exit 1
  fi

  set_user_password "${email_username}" "${COGNITO_USER_PASSWORD}"
done
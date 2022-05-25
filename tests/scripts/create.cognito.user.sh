#!/bin/bash

source "$(dirname $(realpath "$BASH_SOURCE"))/utils.sh"

create_user_resp=$(create_user "${ADMIN_USERNAME}")

if [ $? -ne 0 ]; then
  echo "\nERROR: Failed to create user."
  exit 1
fi

set_user_password "${ADMIN_USERNAME}" "${ADMIN_PASSWORD}"

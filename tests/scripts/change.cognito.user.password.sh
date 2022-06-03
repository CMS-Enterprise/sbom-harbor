#!/bin/bash

source "$(dirname $(realpath $BASH_SOURCE))/utils.sh"

set_user_password "${ADMIN_USERNAME}" "${ADMIN_PASSWORD}"

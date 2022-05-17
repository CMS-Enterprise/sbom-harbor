#!/bin/bash

aws cognito-idp admin-set-user-password \
  --user-pool-id "us-east-1_FlYvZPlkT"  \
  --username "d102a666-2e70-4f05-ab35-75fb5d192148" \
  --password "L0g1nTe5tP@55!" \
  --permanent --debug


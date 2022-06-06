#!/bin/bash

get_domain() {
  distros=$()
  domain=$(
    aws cloudfront list-distributions --output json \
    | jq '.DistributionList|.Items|.[0]|.DomainName' \
    | sed -e 's/"//g'
  )
  echo "https://$domain"
}

echo "$(get_domain)"

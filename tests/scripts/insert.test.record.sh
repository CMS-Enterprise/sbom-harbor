#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "${DIR}" || exit

for json in "good.token" "disabled.token" "expired.token" "team.members" "team.serialized"; do
  aws dynamodb batch-write-item --request-items=file://${json}.json
done

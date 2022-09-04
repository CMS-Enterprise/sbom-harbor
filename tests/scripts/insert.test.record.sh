#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
cd "${DIR}" || exit

for json in *.item.json ; do
  file_name=${DIR}/${json}
  echo "Writing Item in ${file_name}"
  aws dynamodb batch-write-item --request-items=file://"${file_name}"
done

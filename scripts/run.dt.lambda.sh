#!/bin/bash

echo "Invoking Lambda"

aws lambda invoke   \
  --function-name SBOMApiStack-DependencyTrackInterfaceLambdaC39E846-b80T1KxCmbCT   \
      --cli-binary-format raw-in-base64-out  \
          --payload '{"key1": "value1", "key2": "value2", "key3": "value3"}' output.txt


echo "/Invoking Lambda"

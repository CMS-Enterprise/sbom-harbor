#!/bin/bash

poetry run pip install --upgrade -t tmp dist/*.whl
cd tmp; 
zip -r ../dist/lambda.zip . -x '*.pyc'
cd ..
rm -rf tmp

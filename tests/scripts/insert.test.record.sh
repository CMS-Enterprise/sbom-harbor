#!/bin/bash

aws dynamodb batch-write-item --request-items=file://team.seralized.json

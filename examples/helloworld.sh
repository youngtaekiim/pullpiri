#!/bin/bash

#BODY=$(< ./resources/helloworld.yaml)
BODY=$(< ./resources/helloworld_no_condition.yaml)

curl -X POST 'http://172.31.26.216:8080/api/v1/yaml' \
--header 'Content-Type: text/plain' \
--data "${BODY}"
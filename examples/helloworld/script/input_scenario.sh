#!/bin/bash

echo "1-start, 2-stop :"
read input
A=$((input))
BODY=""

if [ "$A" -eq 1 ]; then
	BODY="helloworld/helloworld"
elif [ "$A" -eq 2 ]; then
	BODY="helloworld/helloworld-stop"
fi

curl --location 'http://0.0.0.0:47099/scenario' \
--header 'Content-Type: text/plain' \
--data "${BODY}"

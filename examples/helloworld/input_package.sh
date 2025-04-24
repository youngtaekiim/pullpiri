#!/bin/bash

PACKAGES="helloworld"

for package in $PACKAGES
do
    curl --location 'http://0.0.0.0:47099/package' \
    --header 'Content-Type: text/plain' \
    --data "${package}"
done
#!/bin/bash
BINARY_FOLDER=$(pwd)/target/debug
BINARY="api-server etcd piccoloctl piccoloyaml statemanager yamlparser test-grpc-sender"
LINK_FOLDER=$(pwd)/bin
EXAMPLE_FOLDER=$(pwd)/example

if [ ! -d "$LINK_FOLDER" ]; then
	mkdir bin
	cp -r $EXAMPLE_FOLDER $LINK_FOLDER
fi

for exe in $BINARY
do
	EXE_PATH=$LINK_FOLDER/$exe
	if [ ! -e "$EXE_PATH" ]; then
		ln -s $BINARY_FOLDER/$exe $EXE_PATH
	fi
done
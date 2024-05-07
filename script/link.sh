#!/bin/bash
BINARY_FOLDER=$(pwd)/target/debug
BINARY="api-server statemanager yamlparser"
TEST_BINARY_FOLDER=$(pwd)/tools/target/debug
TEST_BINARY="piccoloctl piccoloyaml test-grpc-sender"
LINK_FOLDER=$(pwd)/bin
EXAMPLE_FOLDER=$(pwd)/doc/examples

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

for exe in $TEST_BINARY
do
	EXE_PATH=$LINK_FOLDER/$exe
	if [ ! -e "$EXE_PATH" ]; then
		ln -s $TEST_BINARY_FOLDER/$exe $EXE_PATH
	fi
done
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

FROM python:3.9.19-slim

WORKDIR /app

RUN apt update && apt install -y python3-pyqt5
RUN pip install cyclonedds PyQt5 grpcio grpcio-tools

COPY ./pyqt-grpc-sender/ .

RUN python -m grpc_tools.protoc --python_out=. --grpc_python_out=. --proto_path=. ./yamlparser.proto

CMD python3 /app/main.py


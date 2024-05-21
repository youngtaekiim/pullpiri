# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

FROM python:3.9.19-slim

WORKDIR /app

RUN apt update && apt install -y python3-pyqt5
RUN pip install cyclonedds PyQt5
COPY ./pyqt-dds-sender .

CMD python3 /app/main.py


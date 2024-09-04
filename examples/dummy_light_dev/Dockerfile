# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

FROM python:3.9.19-slim

WORKDIR /app

#RUN apt update
RUN pip install cyclonedds
COPY . .

CMD python3 -u /app/main.py
#CMD sh


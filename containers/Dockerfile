# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.77.2-slim AS builder
WORKDIR /piccolo-bluechi

COPY . .

RUN apt update -y && \
    apt upgrade -y && \
    apt install -y libdbus-1-dev pkg-config protobuf-compiler && \
    cargo build --release


FROM alpine:3.19.1
WORKDIR /piccolo-bluechi

COPY --from=builder /piccolo-bluechi/target/release/api-server ./
#COPY --from=builder /piccolo-bluechi/target/release/gateway ./
COPY --from=builder /piccolo-bluechi/target/release/statemanager ./
COPY --from=builder /piccolo-bluechi/target/release/yamlparser ./
COPY --from=builder /lib /lib
COPY --from=builder /lib64 /lib64

CMD sh
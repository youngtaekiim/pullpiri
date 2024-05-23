# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

FROM rust:1.77.2-slim AS builder
WORKDIR /piccolo-bluechi

COPY ./src .

RUN apt update -y && \
    apt upgrade -y && \
    apt install -y libdbus-1-dev pkg-config protobuf-compiler && \
    cargo build --release


FROM alpine:3.19.1
WORKDIR /piccolo-bluechi

COPY ./piccolo.ini .

COPY --from=builder \
    /piccolo-bluechi/target/release/api-server \
    /piccolo-bluechi/target/release/statemanager \
    /piccolo-bluechi/target/release/yamlparser ./

COPY --from=builder /lib64 /lib64
COPY --from=builder \
    /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 \
    /lib/x86_64-linux-gnu/libgcc_s.so.1 \
    /lib/x86_64-linux-gnu/libdbus-1.so.3 \
    /lib/x86_64-linux-gnu/libdbus-1.so.3.32.4 \
    /lib/x86_64-linux-gnu/libm.so.6 \
    /lib/x86_64-linux-gnu/libc.so.6 \
    /lib/x86_64-linux-gnu/libsystemd.so.0 \
    /lib/x86_64-linux-gnu/libsystemd.so.0.35.0 \
    /lib/x86_64-linux-gnu/libcap.so.2 \
    /lib/x86_64-linux-gnu/libcap.so.2.66 \
    /lib/x86_64-linux-gnu/libgcrypt.so.20 \
    /lib/x86_64-linux-gnu/libgcrypt.so.20.4.1 \
    /lib/x86_64-linux-gnu/liblzma.so.5 \
    /lib/x86_64-linux-gnu/liblzma.so.5.4.1 \
    /lib/x86_64-linux-gnu/libzstd.so.1 \
    /lib/x86_64-linux-gnu/libzstd.so.1.5.4 \
    /lib/x86_64-linux-gnu/liblz4.so.1 \
    /lib/x86_64-linux-gnu/liblz4.so.1.9.4 \
    /lib/x86_64-linux-gnu/libgpg-error.so.0 \
    /lib/x86_64-linux-gnu/libgpg-error.so.0.33.1 /lib/x86_64-linux-gnu/

CMD sh
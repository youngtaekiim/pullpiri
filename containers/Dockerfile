FROM rust:1.77.2-slim AS builder
WORKDIR /piccolo-bluechi

COPY .. .

RUN apt update -y && \
    apt upgrade -y && \
    apt install -y libdbus-1-dev pkg-config protobuf-compiler && \
    cargo build --release


FROM rust:1.77.2-slim
WORKDIR /piccolo-bluechi
RUN apt update -y && \
    apt upgrade -y && \
    apt install -y libdbus-1-dev pkg-config protobuf-compiler
COPY --from=builder /piccolo-bluechi/target/release ./

CMD sh
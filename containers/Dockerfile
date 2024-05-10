FROM rust:1.77.2-slim AS builder
WORKDIR /piccolo-bluechi

COPY ../Cargo.toml ./Cargo.toml
COPY ../Cargo.lock ./Cargo.lock

COPY ../api ./api
COPY ../api-server ./api-server
COPY ../common ./common
COPY ../statemanager ./statemanager
COPY ../yamlparser ./yamlparser
COPY ../gateway ./gateway

RUN apt update -y && \
    apt upgrade -y && \
    apt install -y libdbus-1-dev pkg-config protobuf-compiler
RUN cargo build --release


FROM rust:1.77.2-slim
WORKDIR /piccolo-bluechi
RUN apt update -y && \
    apt upgrade -y && \
    apt install -y libdbus-1-dev pkg-config protobuf-compiler
COPY --from=builder /piccolo-bluechi/target/release ./

CMD sh
.PHONY: default build
build: fmt
	cargo build

.PHONY: release
release: fmt
	cargo build --release

.PHONY: tool
tool:
	cargo build --manifest-path=tools/Cargo.toml

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test

.PHONY: clean
clean: fmt
	cargo clean && \
	cargo clean --manifest-path=tools/Cargo.toml && \
	rm -rf bin

.PHONY: up
up:
	docker compose up -d

.PHONY: tup
tup:
	docker compose -f tools/py-tools/docker-compose.yaml up -d

.PHONY: down
down:
	docker compose down

.PHONY: tdown
tdown:
	docker compose -f tools/py-tools/docker-compose.yaml down
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

.PHONY: link
link: build tool
	$(shell ./script/link.sh)

.PHONY: default build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: tool
tool:
	cargo build --manifest-path=tools/Cargo.toml

.PHONY: clean
clean:
	cargo clean && \
	cargo clean --manifest-path=tools/Cargo.toml

# Section for podman-kube workload - START
.PHONY: image
image:
	podman build -t piccolo:1.0 -f containers/Dockerfile .

.PHONY: install
install:
	cp -r ./containers /etc/containers/systemd/piccolo/ && \
	cp -r ./etcd-data /etc/containers/systemd/piccolo/etcd-data/ && \
	cp -r ./doc/examples/scenario /etc/containers/systemd/piccolo/example/ && \
	systemctl daemon-reload && \
	systemctl start piccolo

.PHONY: uninstall
uninstall:
	systemctl stop piccolo && \
	rm -rf /etc/containers/systemd/piccolo/ && \
	systemctl daemon-reload
# Section for podman-kube workload - END

# Section for docker-compose - START
.PHONY: up
up:
	docker compose -f containers/docker-compose.yaml up -d

.PHONY: tup
tup:
	docker compose -f tools/py-tools/docker-compose.yaml up -d

.PHONY: down
down:
	docker compose -f containers/docker-compose.yaml down

.PHONY: tdown
tdown:
	docker compose -f tools/py-tools/docker-compose.yaml down
# Section for docker-compose - END

# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

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
	podman build -t piccolo-gateway:1.0 -f containers/Dockerfile-gateway .

.PHONY: install
install:
	cp -r ./containers /etc/containers/systemd/piccolo/ && \
	cp -r ./etcd-data /etc/containers/systemd/piccolo/etcd-data/ && \
	cp -r ./doc/examples/version-display/scenario /etc/containers/systemd/piccolo/example/ && \
	systemctl daemon-reload && \
	systemctl start piccolo

.PHONY: uninstall
uninstall:
	systemctl stop piccolo && \
	rm -rf /etc/containers/systemd/piccolo/ && \
	systemctl daemon-reload

.PHONY: tinstall
tinstall:
	mkdir /etc/containers/systemd/piccolo-test/ && \
	cp -r ./doc/examples/version-display/qt-msg-sender/qt-sender.* /etc/containers/systemd/piccolo-test/ && \
	systemctl daemon-reload && \
	systemctl start qt-sender

.PHONY: tuninstall
tuninstall:
	systemctl stop qt-sender && \
	rm -rf /etc/containers/systemd/piccolo-test/ && \
	systemctl daemon-reload && \
	systemctl stop version-display.service
# Section for podman-kube workload - END

# [DEBUGGING ONLY] Section for docker-compose - START
.PHONY: cleanup
cleanup:
	docker compose -f containers/docker-compose.yaml build --no-cache && \
	docker compose -f containers/docker-compose.yaml up --build -d

.PHONY: up
up:
	docker compose -f containers/docker-compose.yaml up -d

.PHONY: tup
tup:
	docker compose -f doc/examples/version-display/py-tools/docker-compose.yaml up -d

.PHONY: down
down:
	docker compose -f containers/docker-compose.yaml down

.PHONY: tdown
tdown:
	docker compose -f doc/examples/version-display/py-tools/docker-compose.yaml down
# [DEBUGGING ONLY] Section for docker-compose - END

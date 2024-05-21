<!--
SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.

SPDX-License-Identifier: Apache-2.0
-->

# Getting started

## System requirements
Piccolo with Bluechi has been tested with CentOS Stream 9.  
[Bluechi](https://github.com/eclipse-bluechi/bluechi/tree/main) is required for Piccolo.  
[Podman](https://podman.io/) needs to be installed as this is used as container runtime (see Podman installation instructions).  
Also, [Rust](https://www.rust-lang.org) is required for development.

## Preliminary Info
There is a [piccolo.ini](/piccolo.ini) for configuration.
```bash
HOST_IP=10.157.19.218
YAML_STORAGE=/root/piccolo_yaml/
BLUECHI_HOST_NODE=master
BLUECHI_GUEST_NODE=worker1
# more items will be added
```
- HOST_IP : Each modules use this IP address for gRPC communications.
- YAML_STORAGE : For making systemd service with podman, we need `.kube` and `.yaml` files. yamlparser module makes these files in this directory.
- BLUECHI_HOST_NAME : To deliver `bluechi` command, we need node name.

## Limitations
- Multi-node system and the resulting node-selectors have not yet been fully considered.
- For better operation, recommend operating with `root` user.
- `/etc/containers/systemd` folder is used for piccolo systemd service files. This cannot be changed.

## Installation
All Piccolo applications with test app will start in container.  
If you are familiar with container, you will find it easy to use.  
Although it supports `docker compose`, `podman play via Bluechi` uses podman as its container runtime, so `Piccolo` also uses `podman play` by default.
If this is your first time, I recommend following [Example](/doc/examples/version-display/README.md) first.

### Podman-kube
Before starting, you must build Piccolo container image,
```sh
make image
```

For starting,
```sh
make install
```

For stoping,
```sh
make uninstall
```

### (Optional) Docker compose
For starting,
```sh
make up
```

For stoping,
```sh
make down
```

Also refer to [Makefile](/Makefile).
<!--
SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.

SPDX-License-Identifier: Apache-2.0
-->

# Getting started

## System requirements
Piccolo has been tested with CentOS Stream 9.

[Bluechi](https://github.com/eclipse-bluechi/bluechi/tree/main) is required for Piccolo.  
[Podman](https://podman.io/) needs to be installed as this is used as container runtime (Maybe podman is already installed in CentOS Stream 9).  
Also, [Rust](https://www.rust-lang.org) is required to test without using a container.

## Preliminary Info

### Piccolo Configuration
There is a [piccolo.ini](/piccolo.ini) for configuration. Modify this to suit your system.
```ini
HOST_IP=192.168.50.239
YAML_STORAGE=/root/piccolo_yaml/
HOST_NODE=master
GUEST_NODE=worker1
# more items will be added
```
- HOST_IP : Each modules use this IP address for gRPC communications.
- YAML_STORAGE : For making systemd service with podman, we need `.kube` and `.yaml` files. yamlparser module makes these files in this directory.
- HOST_NAME : To deliver systemd command with `bluechi`, we need node name.

Also you need to modify `HOST_IP` address in `yaml` file.
```
# in containers/piccolo.yaml, there are 2 host IP env like below.
...
value: "192.168.50.239"
...

# in doc/examples/version-display/qt-msg-sender/qt-sender.yaml,
...
value: "192.168.50.239"
...
```
### Piccolo modules
Piccolo consists of many modules.
For each modules, refer to [Structure](/doc/docs/developments.md#structure).  
And the [example](/doc/examples/version-display/README.md) would be helpful.

## Limitations
- Multi-node system and the resulting node-selectors have not yet been fully considered.
- For better operation, recommend operating with `root` user.
- `/etc/containers/systemd` folder is used for piccolo systemd service files. This cannot be changed.

## Installation

### Before installation
need some packages, disable firewall, disable selinux
```bash
# disable firewall
systemctl stop firewalld
systemctl disable firewalld
# install package
dnf install git-all make gcc -y
# disable selinux
setenforce 0
```
For modifying configuration, see [configuration](#piccolo-configuration).

### Install process
All Piccolo applications with test app will start in container.
If you are familiar with container, you will find it easy to use.
`Piccolo` also uses `podman play` by default.
If this is your first time, I recommend following [Example](/doc/examples/version-display/README.md) first.

Before starting, you must build Piccolo container image,
```sh
make image
```
If you have errors during `apt update`, then check dns nameserver.

For starting,
```sh
make install
```

For stoping,
```sh
make uninstall
```

Also refer to [Makefile](/Makefile).
# Getting started

## Installation
Piccolo does not require any special installation process.

## System requirements
Piccolo with Bluechi has been tested with CentOS Stream 9.  
[Bluechi](https://github.com/eclipse-bluechi/bluechi/tree/main) is required for Piccolo.  
[Podman](https://podman.io/) needs to be installed as this is used as container runtime (see Podman installation instructions).  
Also, [Rust](https://www.rust-lang.org) is required for development.

## Quick start
All applications with test app will start in container.  
If you are familiar with container, you will find it easy to use.  
Although it supports `docker compose`, `Bluechi` uses podman as its container runtime, so `Piccolo` also uses `podman-kube` by default.

### 1. Podman-kube
Before starting, you must build container image,
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

### 2. Docker compose
For starting,
```sh
make up
```

For stoping,
```sh
make down
```

Also refer to [Makefile](/Makefile).
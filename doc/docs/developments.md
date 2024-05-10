# UNDER CONSTRUCTION (from bluechi docs)

# Piccolo-Bluechi
- [Development](#development)
  - [Environment Setup](#environment-setup)
    - [Prerequisites](#prerequisites)
    - [Dependencies Installation](#dependencies-installation)
  - [Code Style](#code-style)
  - [Linting](#linting)
  - [Build](#build)
    - [Build options](#build-options)
    - [Bindings](#bindings)
  - [Static Code Analysis](#static-code-analysis)
  - [Debug](#debug)
  - [Unit Tests](#unit-tests)
  - [Integration Tests](#integration-tests)
  - [Running](#running)
    - [Ports map](#ports-map)
    - [Assumed setup](#assumed-setup)
    - [bluechi](#bluechi-controller)
    - [bluechi-agent](#bluechi-agent)
    - [bluechictl](#bluechictl)
- [Documentation](#documentation)

## Development

### Environment Setup

#### Prerequisites

To build the project on CentOS Stream 9 you need to enable CodeReady Build repository:

```bash
sudo dnf install dnf-plugin-config-manager
sudo dnf config-manager --set-enabled crb
sudo dnf install -y epel-release
```

#### Dependencies installation

```bash
sudo dnf install \
    bzip2 \
    clang-tools-extra \
    gcc \
    gcc-c++ \
    git \
    golang-github-cpuguy83-md2man \
    make \
    meson \
    systemd-devel \
    selinux-policy-devel
```

[markdownlint-cli2](https://github.com/DavidAnson/markdownlint-cli2) can be used for static analysis of markdown files.
Check the [installation guide](https://github.com/DavidAnson/markdownlint-cli2#install) and use the most appropriate way
of installation for your setup.


### Code Style

[rustfmt](https://github.com/rust-lang/rustfmt) is used.
```bash
cargo fmt
```

### Linting

[Clippy](https://doc.rust-lang.org/nightly/clippy/) is used.
```bash
cargo clippy
```

### Build

The project is using [meson](https://mesonbuild.com/) as its primary build system.

The binaries and other artifacts (such as the man pages) can be built via:

```bash
meson setup builddir
meson compile -C builddir
```

After successfully compiling the binaries, they can be installed into a destination directory (by default
`/usr/local/bin`) using:

```bash
meson install -C builddir
```

To install it into `builddir/bin` use:

```bash
meson install -C builddir --destdir bin
```

After building, the following binaries are available:

- `bluechi-controller`: the systemd service controller which is run on the main machine, sending commands to the agents
  and monitoring the progress
- `bluechi-agent`: the node agent unit which connects with the controller and executes commands on the node machine
- `bluechi-proxy`: an internally used application to resolve cross-node dependencies
- `bluechictl`: a helper (CLI) program to send an commands to the controller

#### Build options

BlueChi can be built with configurable options as listed in [meson_options.txt](./meson_options.txt). The value for
those settings can either be changed directly in the file or via

```bash
# assuming an initial "meson setup builddir"
meson configure -D<option-name>=<option-value> builddir
```

Current options include:

- `with_analyzer`: This option enables the [gcc option for static analysis](https://gcc.gnu.org/onlinedocs/gcc-13.2.0/gcc/Static-Analyzer-Options.html)
- `with_coverage`: This option ensures that BlueChi is built to collect coverage when running a BlueChi binary
- `with_man_pages`: This option enables building man pages as a part of the project build
- `with_selinux`: This option includes building the SELinux policy for BlueChi

#### Bindings

Bindings for the D-Bus API of `BlueChi` are located in [src/bindings](./src/bindings/). Please refer to the
[README.md](./src/bindings/README.md) for more details.

A complete set of typed python bindings for the D-Bus API is auto-generated. On any change to any of the [interfaces](./data/),
these need to be re-generated via

```bash
./build-scripts/generate-bindings.sh python
```

### Static Code Analysis
TBD

### Debug

In some cases, developers might need a debug session with tools like gdb, here an example:

First, make sure **meson.build** contains **debug=true**.

Rebuild the BlueChi project with debug symbols included:

```bash
bluechi> make clean
bluechi> meson install -C builddir --dest=bin
bluechi> gdb --args ./builddir/bin/usr/local/libexec/bluechi-controller -c /etc/bluechi/controller.conf
```

### Unit tests
Unit tests can be executed using following commands:
```bash
cargo test
```

will produce a coverage report in `builddir/meson-logs/coveragereport/index.html`

### Integration tests
TBD

### Running

The following sections describe how to run the built application(s) locally on one machine. For this, the assumed setup
used is described in the first section.

#### Ports map
```
api-server : 47001
gateway : 47002
statemanager: 47003
yamlparser : 47004
etcd : 2379
```

#### Assumed setup

The project has been build with the following command sequence:

```bash
meson setup builddir
meson compile -C builddir
meson install -C builddir --destdir bin
```

Meson will output the artifacts to `./builddir/bin/usr/local/`. This directory is referred to in the following sections
simply as `<builddir>`.

To allow `bluechi-controller` and `bluechi-agent` to own a name on the local system D-Bus, the provided configuration
files need to be copied (if not already existing):

```bash
cp <builddir>/share/dbus-1/system.d/org.eclipse.bluechi.Agent.conf /etc/dbus-1/system.d/
cp <builddir>/share/dbus-1/system.d/org.eclipse.bluechi.conf /etc/dbus-1/system.d/
```

**Note:** Make sure to reload the dbus service so these changes take effect: `systemctl reload dbus-broker.service` (or
`systemctl reload dbus.service`)

#### bluechi-controller

The newly built controller can simply be run via `./<builddir>/bin/bluechi-controller`, but it is recommended to use a
specific configuration for development. This file can be passed in with the `-c` CLI option:

```bash
./<builddir>/bin/bluechi-controller -c <path-to-cfg-file>
```

#### bluechi-agent

Before starting the agent, it is best to have the `bluechi-controller` already running. However, `bluechi-agent` will
try to reconnect in the configured heartbeat interval.

Similar to `bluechi-controller`, it is recommended to use a dedicated configuration file for development:

```bash
./<builddir>/bin/bluechi-agent -c <path-to-cfg-file>
```

#### bluechictl

The newly built `bluechictl` can be used via:

```bash
./<builddir>/bin/bluechictl COMMANDS
```

## Documentation

Files for documentation of this project are located in the [docs](/doc/) directory comprising:

- [api examples](./doc/api-examples/): directory containing source files for different programming languages that use
the D-Bus API of BlueChi, e.g. for starting a systemd unit
- [man](./doc/man/): directory containing the markdown files for generating the man pages
(see [Building MAN pages](#building-man-pages) for more information)
- readthedocs files for building the documentation website of BlueChi (see [the README](./doc/README.md) for further information)
- [diagrams.drawio](./doc/diagrams.drawio) file containing all diagrams used for BlueChi

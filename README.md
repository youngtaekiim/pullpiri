<!--
SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.

SPDX-License-Identifier: Apache-2.0
-->

<img src="doc/images/Piccolo.jpg" 
width="50%"
height="50%"
/>

# Piccolo-Bluechi
Piccolo based on [Bluechi](https://github.com/eclipse-bluechi/bluechi/tree/main).

## Piccolo
- **Package Service on Cloud**  
: Compose an application by combining services  
: Define service relationships, select execution location  
: Application Model Structure (Redundant Structure, Monitoring, Proxy, Etc.)

- **Service Orchestrator on Vehicle**  
: Execute scenarios depending on vehicle status  
: Determination of Service Transition Time (Start, Stop, Restart, Switching, Etc.)  
: Manager text-defined scenarios and policies (Supports UX updates after release)

## Getting started
Refer to [Getting Started](/doc/docs/getting-started.md).

## Development
Refer to [Development](/doc/docs/developments.md).

## License
The [LICENSES](/LICENSES) directory contains all the licenses used by the PICCOLO Project.

Piccolo itself uses the [Apache-2.0](/LICENSES/Apache-2.0.txt) license.

The following packages (Rust Crate) use the [MIT](/LICENSES/MIT.txt) license.
- clap
- dbus
- etcd-client
- serde
- serde_yaml
- tokio
- tonic
- tonic-build

The following packages (Rust Crate) use the [Apache-2.0](/LICENSES/Apache-2.0.txt) license.
- clap
- dbus
- etcd-client
- prost
- serde
- serde_yaml

The following packages (Rust Crate) use the [zlib](/LICENSES/zlib.txt) license.
- const_format
# Structure

<img alt="piccolo overview" src="../images/overview.png"
width="75%"
height="75%"
/>

```bash
.
├── containers      # files for binary container
├── doc
│   ├── docs
│   ├── examples
│   └── images
├── etcd-data       # dummy folder for etcd DB
├── LICENSES
└── src
    ├── api         # gRPC proto files
    ├── api-server
    ├── common      # common library
    ├── gateway
    ├── statemanager
    ├── target
    ├── tools
    │   ├── piccoloctl
    │   ├── piccoloyaml
    │   ├── target
    │   ├── test-grpc-sender
    │   └── workloadconverter
    └── yamlparser
```

## yamlparser

 Yamlparser is responsible for receiving the scenario file and parsing it into the necessary items for piccolo.

 Specifically, create a `.kube` file and a `.yaml` file in the PICCOLO_YAML path, separate the scenario into condition and action, and pass it to the api-server.

## api-server

Api-server acts similarly to api-server in k8s.
Write the action and condition passed from the yamlparser to etcd, so that the gateway can recognize the condition.
Apart from this, there is a direct access to the statemanager for testing purposes.

## gateway

The gateway receives a vehicle message according to the condition received from the api-server and notifies the statemanager when the condition is satisfied.
It is currently written in C++, but there are plans to rewrite it in Rust.

## statemanager

The statemanager calls the other workload orchestrator API based on a message from the gateway or api-server.
Therefore, it is the destination that must be passed through in order to execute the workload.
Specifically, when it receives a notification from the gateway that the condition has been satisfied, it pulls out the corresponding action from etcd and executes it.

## etcd

The etcd stores data that is commonly used by each Piccolo module.
Writes are made only from the api-server, and the gateway and statemanager read them to perform the necessary actions.

## others

TBD

<!-- markdownlint-disable-file MD033 -->
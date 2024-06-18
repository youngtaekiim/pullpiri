# version-display example

In this example, we will run a simple app as container that displays the version name.
At this time, it shows the process of updating or rolling back the app according to pre-specified conditions through `Piccolo`.

## Test summary

![sequence diagram - launch](/doc/images/sequence-launch.png)

### Send scenario to piccolo (yamlparser)

All test applications are available in below chapter.

First, `Qt gRPC sender` transfers the update (or rollback) scenario file to yamlparser.
This file is as follows: [Link to file](/examples/version-display/scenario/update-scenario.yaml)

```yaml
apiVersion: v1
kind: Scenario
metadata:
  name: version-display
spec:
  conditions:
    express: Equal
    value: "parking"
    operands:
      type: DDS
      name: gear_state
      value: "rt/piccolo/gear_state"
  actions:
    - operation: update
      podSpec:
        containers:
        - name: display-container
          image: localhost/version-display:2.0
          volumeMounts:
          - name: x11
            mountPath: /tmp/.X11-unix
          env:
          - name: DISPLAY
            value: :0
        volumes:
        - name: x11
          hostPath:
            path: /tmp/.X11-unix
```

All items may change in the future.

The items to pay attention to now are the `metadata-name`, `spec-conditions` and `spec-actions`.

- condition : This item is used for gateways.
In the example, the condition is satisfied if a DDS message is received each time the gear state changes and the system enters `Parking`.

- action : This item is used by statemanager.
When a condition is met at the gateway, a notification is sent to the statemanager and the statemanager takes action.
In this example, the `update` is performed using the workload defined in the podSpec.
However, since they are currently not clearly defined, there is no significant difference in deployment, update, and rollback operations.

- name : In an update scenario, it is difficult to manage with the systemd service file because both the existing and new workloads have the same name and only the image version is different.
Currently Piccolo places all required `.kube .yaml` files in [YAML_STORAGE](/piccolo.ini) folder and creates symbolic links in `/etc/containers/systemd` path.
It can be replaced by a better method at any time.  
A symbolic link is created in statemanager, and in yamlparser, a folder is created with name and the necessary files are created.
This example has the following structure:

```text
/root/piccolo_yaml/
└── version-display
    ├── version-display_1.0.kube
    ├── version-display_1.0.yaml
    ├── version-display_2.0.kube
    └── version-display_2.0.yaml
```

`_1.0` is created when a rollback scenario is entered, and `_2.0` is created when an update scenario is entered.

```sh
# cat  /root/piccolo_yaml/version-display/version-display_2.0.kube 
[Unit]
Description=A kubernetes yaml based version-display service
Before=local-fs.target

[Install]
# Start by default on boot
WantedBy=multi-user.target default.target

[Kube]
Yaml=/root/piccolo_yaml/version-display/version-display_2.0.yaml
```

```yaml
# cat  /root/piccolo_yaml/version-display/version-display_2.0.yaml 
apiVersion: v1
kind: Pod
metadata:
  name: version-display
spec:
  containers:
  - name: display-container
    image: localhost/version-display:2.0
    volumeMounts:
    - name: x11
      mountPath: /tmp/.X11-unix
    env:
    - name: DISPLAY
      value: :0
  volumes:
  - name: x11
    hostPath:
      path: /tmp/.X11-unix
```

### Save action & condition in etcd

Yamlparser sends the parsed actions, conditions, and names to the api-server in struct format via gRPC.
api-server stores each data in etcd using scenario/{name}/conditions and scenario/{name}/action as keys.

```yaml
# etcdctl --endpoints=192.168.50.239:2379 --prefix=true get "scenario/version"
# scenario/version-display/action
operation: update
podSpec:
  containers:
  - name: display-container
    image: sdv.lge.com/library/version_display:2.0
    volumeMounts:
    - name: x11
      mountPath: /tmp/.X11-unix
    env:
    - name: DISPLAY
      value: :0
  volumes:
  - name: x11
    hostPath:
      path: /tmp/.X11-unix

# scenario/version-display/conditions
express: Equal
value: parking
operands:
  type: DDS
  name: gear_state
  value: rt/piccolo/gear_state
```

And api-server sends the condition key to the gateway.

### Wait until condition is satisfied

When the gateway receives a condition key, it creates some kind of filter for that condition.  
In this example, a thread is created to receive DDS messages with topic `rt/piccolo/gear_state`.  
When `parking` is received for the topic, the scenario name is sent to statemanager.  
This process can also be performed through the `Qt button example` app in the chapter below.

### make symbolic link and run workload

When statemanager receives the scenario name from the gateway, an action can be taken out from etcd.
Statemanager can extract the scenario name, image name, and version from here, find the corresponding file, and create a symbolic link.
(Although not mentioned, stopping existing workload and removing existing link is also performed.)

```sh
# ls -al /etc/containers/systemd/version-display.kube 
/etc/containers/systemd/version-display.kube -> /root/piccolo_yaml/version-display/version-display_2.0.kube
```

Now, send the reload and start unit commands to systemd and the entire process will be completed.

## Run the example

### Build test application

This container image only show `version:x.0` in monitor.

```bash
# in 'examples/version-display/app' directory
podman build --no-cache --build-arg VERSION=1.0 -t version-display:1.0 .
podman build --no-cache --build-arg VERSION=2.0 -t version-display:2.0 .
```

You can see

```Text
# podman images
REPOSITORY                   TAG   IMAGE ID      CREATED        SIZE
...
localhost/version-display    2.0   d1b0cd9eb6c2  12 hours ago   566 MB
localhost/version-display    1.0   892206770723  12 hours ago   566 MB
...
```

### Build dds/grpc message sender

There are 2 msg sender application written in pyqt.

```bash
# in 'examples/version-display/qt-msg-sender' directory
podman build --no-cache -t pyqt-dds-sender:1.0 -f pyqt-dds-sender/Dockerfile .
podman build --no-cache -t pyqt-grpc-sender:1.0 -f pyqt-grpc-sender/Dockerfile .
```

You can see,

```Text
# podman images
localhost/pyqt-grpc-sender   1.0   ede75e404c1b  6 hours ago    950 MB
localhost/pyqt-dds-sender    1.0   7a9e5b7a6580  7 hours ago    917 MB
```

### Running Piccolo

If you have some problems during this section, refer to [Getting started - installation](/doc/docs/getting-started.md#installation).

```bash
# in top folder,
make image
```

Then,

```Text
# podman images
REPOSITORY                   TAG     IMAGE ID      CREATED        SIZE
localhost/piccolo-gateway    1.0     5df0f9b55414  4 hours ago    84.7 MB
localhost/piccolo            1.0     590a847ef028  4 hours ago    36.3 MB
```

Now,

```bash
# in top folder,
make install
```

You can find,

```Text
# podman ps | grep piccolo
fe7721ed  localhost/piccolo:1.0                  ...   piccolo-yamlparser
8fc4a363  localhost/piccolo:1.0                  ...   piccolo-api-server
52842bbf  localhost/piccolo:1.0                  ...   piccolo-statemanager
0083aafd  gcr.io/etcd-development/etcd:v3.5.11   ...   piccolo-etcd
1ac934af  localhost/piccolo-gateway:1.0          ...   piccolo-gateway

# tree /root/piccolo_yaml/version-display/
/root/piccolo_yaml/version-display/
├── version-display_1.0.kube
├── version-display_1.0.yaml
├── version-display_2.0.kube
└── version-display_2.0.yaml

# tree /etc/containers/systemd/
/etc/containers/systemd/
└── piccolo
    ├── etcd-data
    │   └── member
    │       ├── ...
    │       └── ...
    ├── example
    ├── piccolo.kube
    └── piccolo.yaml
```

### Running container example

```bash
make tinstall
```

Then there are two X11 applications.  
(Now, there are two more button `launch`, `terminate` in gRPC sender.)

<img alt="pyqt sender" src="../../images/pyqt-sender.png"
width="60%"
height="60%"
/>

Now, if you press the following buttons sequentially, the color of buttons are changed to green.

1. update button (in Qt gRPC sender)
2. parking (in Qt button Example...)

And, you can see version-display container window.

<img alt="version 2" src="../../images/ver-display-2.png"
width="40%"
height="40%"
/>

If you do not see this window, check the status of service.

```bash
systemctl status bluechi-agent.service
```

Simillary, if you press the following buttons sequentially, the color of buttons are changed to green.

1. rollback button (in Qt gRPC sender)
2. reverse (in Qt button Example...)

<img alt="version 1" src="../../images/ver-display-1.png"
width="40%"
height="40%"
/>

Additionally, the `launch` and `terminate` buttons also receive parking DDS message and operate similarly.

### Clean up

```bash
make uninstall
make tuninstall
```

<!-- markdownlint-disable-file MD033 -->
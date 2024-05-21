# version-display example

## Build test application
This container image only show `version:x.0` in monitor.
```bash
# in 'doc/examples/version-display/app' directory
podman build --no-cache --build-arg VERSION=1.0 -t version-display:1.0 .
podman build --no-cache --build-arg VERSION=2.0 -t version-display:2.0 .
```

You can see
```
REPOSITORY                               TAG                   IMAGE ID      CREATED        SIZE
...
localhost/version-display                2.0                   d1b0cd9eb6c2  12 hours ago   566 MB
localhost/version-display                1.0                   892206770723  12 hours ago   566 MB
...
```

## Build dds/grpc message sender

There are 2 msg sender application written in pyqt.
```bash
# in 'doc/examples/version-display/qt-msg-sender' directory
podman build --no-cache -t pyqt-dds-sender:1.0 -f pyqt-dds-sender/Dockerfile .
podman build --no-cache -t pyqt-grpc-sender:1.0 -f pyqt-grpc-sender/Dockerfile .
```
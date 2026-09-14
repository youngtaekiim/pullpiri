#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

rm -rf /etc/pullpiri/settings.yaml
rm -rf /etc/pullpiri/pullpiri_shared_rocksdb
rm -rf /run/pullpirilog

podman pod stop -t 0 pullpiri-player
podman pod rm -f --ignore pullpiri-player
podman pod stop -t 0 pullpiri-server
podman pod rm -f --ignore pullpiri-server

sleep 1

"${SCRIPT_DIR}/uninstall-agent.sh"

## Delete all containers (uncomment if you want to remove all containers)
# ids=$(podman ps -aq)
# if [ -n "$ids" ]; then
#   podman stop -t 0 $ids
#   podman rm -f $ids
# else
#   echo "No containers to remove."
# fi
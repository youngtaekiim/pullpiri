#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

# SET Piccolo Master node IP address
if [ -n "${1:-}" ]; then
	MASTER_IP="$1"
else
	MASTER_IP="$(hostname -I | awk '{print $1}')"
fi

# Make rocksdb folder
mkdir -p /etc/piccolo/pullpiri_shared_rocksdb
chown 1001:1001 /etc/piccolo/pullpiri_shared_rocksdb

# Make /etc/piccolo folder
mkdir -p /etc/piccolo

# Make logd socket folder
mkdir -p /run/piccololog

# Create settings.yaml file in /etc/piccolo/
echo "Creating settings.yaml file..."
cat > /etc/piccolo/settings.yaml << EOF
host:
  name: HPC
  ip: ${MASTER_IP}
  type: vehicle
  role: master
dds:
  idl_path: src/vehicle/dds/idl
  domain_id: 100
EOF

./containers/devonly/scripts/piccolo-server.sh ${MASTER_IP}
./containers/devonly/scripts/piccolo-player.sh ${MASTER_IP}

sleep 1

./containers/devonly/install-agent.sh ${MASTER_IP} ${MASTER_IP}
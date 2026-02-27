#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

# SET Piccolo Master node IP address
DEFAULT_IP=$(hostname -I | awk '{print $1}')
MASTER_IP="${1:-$DEFAULT_IP}"

# Make rocksdb folder
mkdir -p /etc/piccolo/pullpiri_shared_rocksdb
chown 1001:1001 /etc/piccolo/pullpiri_shared_rocksdb

# Make /etc/piccolo folder
mkdir -p /etc/piccolo

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

./scripts/piccolo-server.sh ${MASTER_IP}
./scripts/piccolo-player.sh ${MASTER_IP}

sleep 1

./install-agent.sh ${MASTER_IP} ${MASTER_IP}
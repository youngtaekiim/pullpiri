#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# SET Pullpiri Master node IP address - Read carefully below paragraph
if [ -n "${1:-}" ]; then
	MASTER_IP="$1"
else
	MASTER_IP="$(hostname -I | awk '{print $1}')"
fi
HOST_NAME="$(hostname)"
# If you want to hardcode the IPs for testing, you can
# uncomment the lines below and comment out the argument parsing above
# MASTER_IP="127.0.0.1"  # First argument - Pullpiri master IP address

# Warn (not fail) if Podman is not using the cgroupfs cgroup manager,
# since some workloads (e.g. GPU/device passthrough) require it.
# The warning is repeated at the end of the script so it isn't missed
# among the output of the sub-scripts run below.
CGROUP_WARNING=""
if command -v podman >/dev/null 2>&1; then
	CGROUP_MANAGER="$(podman info --format '{{.Host.CgroupManager}}' 2>/dev/null)"
	if [ -n "${CGROUP_MANAGER}" ] && [ "${CGROUP_MANAGER}" != "cgroupfs" ]; then
		CGROUP_WARNING="WARNING: Podman cgroup manager is '${CGROUP_MANAGER}', not 'cgroupfs'.\nSome Pullpiri workloads require the 'cgroupfs' cgroup manager.\nSee doc/guides/getting-started.md for how to configure it:\n  sudo systemctl edit podman.service"
		echo "-----------------------------------------------------------------------"
		echo -e "${CGROUP_WARNING}"
		echo "-----------------------------------------------------------------------"
	fi
fi

# Make rocksdb folder
mkdir -p /etc/pullpiri/pullpiri_shared_rocksdb
chown 1001:1001 /etc/pullpiri/pullpiri_shared_rocksdb

# Make /etc/pullpiri folder
mkdir -p /etc/pullpiri

# Make logd socket folder
mkdir -p /run/pullpirilog

# Create settings.yaml file in /etc/pullpiri/
echo "Creating settings.yaml file..."
cat > /etc/pullpiri/settings.yaml << EOF
host:
  name: ${HOST_NAME}
  ip: ${MASTER_IP}
  type: vehicle
  role: master
dds:
  idl_path: src/vehicle/dds/idl
  domain_id: 100
EOF

"${SCRIPT_DIR}/scripts/pullpiri-server.sh" ${MASTER_IP}
"${SCRIPT_DIR}/scripts/pullpiri-player.sh" ${MASTER_IP}

sleep 1

"${SCRIPT_DIR}/install-agent.sh" ${MASTER_IP} ${MASTER_IP}

# Re-show the cgroup manager warning last, so it isn't lost above the
# output produced by the server/player/agent install scripts.
if [ -n "${CGROUP_WARNING}" ]; then
	echo "-----------------------------------------------------------------------"
	echo -e "${CGROUP_WARNING}"
	echo "-----------------------------------------------------------------------"
fi
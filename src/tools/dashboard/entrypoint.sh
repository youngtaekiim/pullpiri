#!/bin/sh
set -e

echo "[INFO] DASHBOARD_ENV=$DASHBOARD_ENV"

# Check vite config
if [ ! -f /app/vite.config.js ]; then
	echo "[ERROR] vite config (vite.config.js) not found in /app. Check that the file is copied into the image." >&2
	ls -al /app | head -50
fi

# Recheck the directory (created when building the image, but safe to rerun)
mkdir -p /tmp/nginx/logs /tmp/nginx/tmp /tmp/nginx/run

# Check if node_modules are writable
if [ ! -L /app/node_modules ] && [ -d /tmp/node_modules ]; then
	echo "[INFO] relinking node_modules to /tmp/node_modules" >&2
	rm -rf /app/node_modules && ln -s /tmp/node_modules /app/node_modules || echo "[ERROR] relink failed" >&2
fi
if [ ! -d /tmp/node_modules ]; then
	echo "[WARN] /tmp/node_modules missing; reinstalling" >&2
	cd /app && npm install --no-audit --no-fund || echo "[ERROR] npm install failed" >&2
fi
chmod -R 777 /tmp/node_modules || echo "[WARN] chmod on /tmp/node_modules failed" >&2

# Replace Vite cache directory (in case default .vite-temp creation fails)
export VITE_TEMP_DIR=/tmp/vite-cache
mkdir -p "$VITE_TEMP_DIR" || echo "[WARN] failed to create $VITE_TEMP_DIR" >&2
export TMPDIR="$VITE_TEMP_DIR"
export VITE_DISABLE_OPTIMIZE_DEPS=1
echo "[INFO] Using Vite cache dir: $VITE_TEMP_DIR (optimizeDeps disabled)"

start_nginx() {
	echo "[INFO] starting nginx"
	nginx -c /etc/nginx/nginx.conf -p /tmp/nginx -g 'daemon off;' &
}

start_vite() {
	echo "[INFO] starting vite dev server (env=$DASHBOARD_ENV)"
	if [ "$DASHBOARD_ENV" = "ec2" ]; then
		npm run dev -- --host --config /app/vite.config.js
	else
		npm run dev -- --config /app/vite.config.js
	fi
}

start_nginx
start_vite

#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail  # Exit on error, undefined variable, or pipe failure

# Initialize paths and log files
LOG_FILE="clippy_results.log"
TMP_FILE="clippy_output.txt"
mkdir -p dist/reports/clippy
REPORT_FILE="dist/reports/clippy/clippy_summary.md"

# Clean up old results
rm -f "$LOG_FILE" "$TMP_FILE" "$REPORT_FILE"

echo "🔍 Running Cargo clippy..." | tee -a "$LOG_FILE"

# Set project root (use GitHub workspace if defined, else fallback to current directory)
PROJECT_ROOT=${GITHUB_WORKSPACE:-$(pwd)}
cd "$PROJECT_ROOT"

resolve_manifest() {
  local primary="$1"
  local fallback="${primary#src/}"

  if [[ -f "$primary" ]]; then
    echo "$primary"
  elif [[ -f "$fallback" ]]; then
    echo "$fallback"
  else
    echo ""
  fi
}

FAILED_TOTAL=0  # Count how many manifests failed Clippy

# Declare paths to Cargo.toml manifests of components
MAJOR_MANIFEST=$(resolve_manifest "src/Cargo.toml")
NODEAGENT_MANIFEST=$(resolve_manifest "src/agent/nodeagent/Cargo.toml")
ROCKSDBSERVICE_MANIFEST=$(resolve_manifest "src/server/rocksdbservice/Cargo.toml")
TOOLS_MANIFEST=$(resolve_manifest "src/tools/Cargo.toml")

# Function to run clippy on a component and track results
run_clippy() {
  local manifest="$1"   # Path to Cargo.toml
  local label="$2"      # Human-readable name (e.g., "major")

  echo "🧪 Running Clippy for $label ($manifest)" | tee -a "$LOG_FILE"

  local clippy_passed=false

  # Run Clippy and capture output to temp file
  if cargo clippy --manifest-path="$manifest" --all-targets --all-features | tee "$TMP_FILE"; then
    echo "✅ Clippy for $label passed clean." | tee -a "$LOG_FILE"
    clippy_passed=true
  else
    echo "::error ::❌ Clippy for $label failed! Found warnings/errors." | tee -a "$LOG_FILE"
    # Optional: uncomment below to include filtered warnings/errors in the log
    # grep -E "warning:|error:" "$TMP_FILE" | tee -a "$LOG_FILE"
  fi

  # Append status to markdown summary report
  if $clippy_passed; then
    echo "✅ Clippy for \`$label\`: **PASSED**" >> "$REPORT_FILE"
  else
    echo "❌ Clippy for \`$label\`: **FAILED**" >> "$REPORT_FILE"
    FAILED_TOTAL=$((FAILED_TOTAL + 1))  # Increment failure count safely under set -e
  fi
}

# === Clippy runs per module ===

if [[ -n "$MAJOR_MANIFEST" ]]; then
  run_clippy "$MAJOR_MANIFEST" "major"
else
  echo "::warning ::src/Cargo.toml not found, skipping..."
fi

if [[ -n "$NODEAGENT_MANIFEST" ]]; then
  run_clippy "$NODEAGENT_MANIFEST" "nodeagent"
else
  echo "::warning ::src/agent/nodeagent/Cargo.toml not found, skipping..."
fi

if [[ -n "$ROCKSDBSERVICE_MANIFEST" ]]; then
  run_clippy "$ROCKSDBSERVICE_MANIFEST" "rocksdbservice"
else
  echo "::warning ::src/server/rocksdbservice/Cargo.toml not found, skipping..."
fi

if [[ -n "$TOOLS_MANIFEST" ]]; then
  run_clippy "$TOOLS_MANIFEST" "tools"
else
  echo "::warning ::src/tools/Cargo.toml not found, skipping..."
fi

# Optional: exit with failure if any component had Clippy errors
if [[ "$FAILED_TOTAL" -gt 0 ]]; then
  echo "::error ::🚨 Clippy failed for $FAILED_TOTAL component(s)." | tee -a "$LOG_FILE"
  exit 1
fi

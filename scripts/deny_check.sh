#!/bin/bash
# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail  # Exit on error, undefined variables, or pipe failure

# Initialize log and report files
LOG_FILE="deny_results.log"
TMP_FILE="deny_output.txt"
mkdir -p dist/reports/deny
REPORT_FILE="dist/reports/deny/deny_summary.md"

# Remove old logs and report files
rm -f "$LOG_FILE" "$TMP_FILE" "$REPORT_FILE"

echo "🔍 Running Cargo Deny checks..." | tee -a "$LOG_FILE"

# Determine project root directory
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

FAILED_TOTAL=0   # Count of manifests that failed deny check
PASSED_TOTAL=0   # Count of manifests that passed deny check

# Define paths to Cargo.toml manifests to check
MAJOR_MANIFEST=$(resolve_manifest "src/Cargo.toml")
NODEAGENT_MANIFEST=$(resolve_manifest "src/agent/nodeagent/Cargo.toml")
ROCKSDBSERVICE_MANIFEST=$(resolve_manifest "src/server/rocksdbservice/Cargo.toml")
TOOLS_MANIFEST=$(resolve_manifest "src/tools/Cargo.toml")

# Function to run cargo-deny on a given manifest and log results
run_deny() {
  local manifest="$1"    # Path to Cargo.toml
  local label="$2"       # Human-readable label for logging

  echo "🚨 Running deny check for $label ($manifest)" | tee -a "$LOG_FILE"

  local deny_passed=false

  # Run cargo deny check; capture all output to temp file
  if cargo deny --manifest-path="$manifest" check 2>&1 | tee "$TMP_FILE"; then
    echo "✅ deny check for $label passed clean." | tee -a "$LOG_FILE"
    deny_passed=true
  else
    # If cargo deny failed, output error message and extract relevant lines
    echo "::error ::Deny check for $label failed! Issues found." | tee -a "$LOG_FILE"
    grep -E "error:|warning:" "$TMP_FILE" | tee -a "$LOG_FILE"
  fi

  # Append pass/fail status to markdown summary report
  if $deny_passed; then
    echo "✅ deny check for $label: PASSED" >> "$REPORT_FILE"
    PASSED_TOTAL=$((PASSED_TOTAL + 1))
  else
    echo "❌ deny check for $label: FAILED" >> "$REPORT_FILE"
    FAILED_TOTAL=$((FAILED_TOTAL + 1))
  fi
}

# Run cargo-deny on desired manifests
# Uncomment manifests as needed

if [[ -n "$MAJOR_MANIFEST" ]]; then
  run_deny "$MAJOR_MANIFEST" "major"
else
  echo "::warning ::src/Cargo.toml not found, skipping..."
fi

if [[ -n "$NODEAGENT_MANIFEST" ]]; then
  run_deny "$NODEAGENT_MANIFEST" "nodeagent"
else
  echo "::warning ::src/agent/nodeagent/Cargo.toml not found, skipping..."
fi

if [[ -n "$ROCKSDBSERVICE_MANIFEST" ]]; then
  run_deny "$ROCKSDBSERVICE_MANIFEST" "rocksdbservice"
else
  echo "::warning ::src/server/rocksdbservice/Cargo.toml not found, skipping..."
fi

if [[ -n "$TOOLS_MANIFEST" ]]; then
  run_deny "$TOOLS_MANIFEST" "tools"
else
  echo "::warning ::src/tools/Cargo.toml not found, skipping..."
fi

# Print final summary report to console and log
echo -e "\n📄 Summary:" | tee -a "$LOG_FILE"
cat "$REPORT_FILE" | tee -a "$LOG_FILE"

echo -e "\n🔢 Total Passed: $PASSED_TOTAL" | tee -a "$LOG_FILE"
echo "🔢 Total Failed: $FAILED_TOTAL" | tee -a "$LOG_FILE"

# Fail the script if any cargo-deny check failed
if [[ "$FAILED_TOTAL" -gt 0 ]]; then
  echo "::error ::One or more cargo-deny checks failed."
  exit 1
fi

echo "✅ All cargo-deny checks passed!"

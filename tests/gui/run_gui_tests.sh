#!/usr/bin/env bash
set -euo pipefail

# Automated GUI Test Runner using xvfb-run
# Fulfills Constitution Principle V (GUI Interaction Verification & Reusable Test Harness)

echo "=== Running Headless GTK GUI Tests via xvfb-run ==="

if command -v xvfb-run >/dev/null 2>&1; then
    xvfb-run -a cargo test --test gui_tests -- --nocapture
else
    echo "Warning: xvfb-run not found. Falling back to DISPLAY=${DISPLAY:-:0}..."
    cargo test --test gui_tests -- --nocapture
fi

echo "=== Headless GUI Tests Passed Successfully ==="

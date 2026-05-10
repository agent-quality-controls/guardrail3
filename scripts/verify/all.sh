#!/usr/bin/env bash
# Roll-up: run every layer verifier and produce a single tally.
# Exit non-zero if any layer fails.

set -u

HERE="$(cd "$(dirname "$0")" && pwd)"

LAYERS=(
  "1-tree:layer1-tree.py"
  "2-cli:layer2-cli.py"
  "3-rules:layer3-rules.py"
  "3c-hook-patterns:layer3-hook-patterns.py"
  "4-deps:layer4-deps.py"
  "5-regressions:layer5-regressions.py"
  "7-test-pinning:layer7-test-pinning.py"
  "8-real-artifact-tests:layer8-real-artifact-tests.py"
)

overall=0
for layer in "${LAYERS[@]}"; do
  name="${layer%%:*}"
  script="${layer##*:}"
  output="$(python3 "$HERE/$script" 2>&1)"
  rc=$?
  echo "$output"
  if [ "$rc" -ne 0 ]; then
    overall=1
  fi
  echo
done

if [ "$overall" -eq 0 ]; then
  echo "verify-all: PASS"
else
  echo "verify-all: FAIL"
fi
exit "$overall"

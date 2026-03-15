#!/bin/sh
# compare.sh — Compare current guardrail3 self-validation output against golden snapshot.
#
# Usage: ./golden-tests/compare.sh
#
# Exit 0 on match (PASS), exit 1 on diff (FAIL).

set -eu

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GOLDEN_DIR="$SCRIPT_DIR/golden"
NORMALIZE="$SCRIPT_DIR/normalize.sh"
GOLDEN_FILE="$GOLDEN_DIR/self-validate.json"

if [ ! -f "$GOLDEN_FILE" ]; then
  echo "FAIL: Golden snapshot not found at $GOLDEN_FILE"
  echo "Run ./golden-tests/run-golden.sh first to capture it."
  exit 1
fi

echo "Building guardrail3 (release)..."
cargo build --release --manifest-path "$PROJECT_ROOT/Cargo.toml" 2>&1 | tail -1

BINARY="$PROJECT_ROOT/target/release/guardrail3"
ACTUAL_FILE="$(mktemp)"

trap 'rm -f "$ACTUAL_FILE"' EXIT

echo "Running self-validation..."
"$BINARY" validate --format json "$PROJECT_ROOT" 2>/dev/null | \
  sh "$NORMALIZE" "$PROJECT_ROOT" > "$ACTUAL_FILE"

echo "Comparing against golden snapshot..."
if diff -u "$GOLDEN_FILE" "$ACTUAL_FILE" > /dev/null 2>&1; then
  CHECKS=$(jq '[.sections[].results[]] | length' "$GOLDEN_FILE")
  echo "PASS: Output matches golden snapshot ($CHECKS checks)"
  exit 0
else
  echo "FAIL: Output differs from golden snapshot"
  echo ""
  echo "--- Diff (golden vs actual) ---"
  diff -u "$GOLDEN_FILE" "$ACTUAL_FILE" || true
  echo ""
  echo "To update the golden snapshot, run: ./golden-tests/run-golden.sh"
  exit 1
fi

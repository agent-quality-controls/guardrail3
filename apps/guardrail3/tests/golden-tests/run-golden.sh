#!/bin/sh
# run-golden.sh — Capture golden snapshot for guardrail3 self-validation.
#
# Usage: ./tests/golden-tests/run-golden.sh
#
# Builds guardrail3 in release mode, runs self-validation with JSON output,
# normalizes the output, and saves it as the golden snapshot.

set -eu

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PROJECT_ROOT="$(cd "$CRATE_ROOT/../.." && pwd)"
GOLDEN_DIR="$SCRIPT_DIR/golden"
NORMALIZE="$SCRIPT_DIR/normalize.sh"

mkdir -p "$GOLDEN_DIR"

echo "Building guardrail3 (release)..."
cargo build --release --manifest-path "$CRATE_ROOT/Cargo.toml" 2>&1 | tail -1

BINARY="$PROJECT_ROOT/target/release/guardrail3"

echo "Running self-validation..."
"$BINARY" validate --format json "$PROJECT_ROOT" 2>/dev/null | \
  sh "$NORMALIZE" "$PROJECT_ROOT" > "$GOLDEN_DIR/self-validate.json"

CHECKS=$(jq '[.sections[].results[]] | length' "$GOLDEN_DIR/self-validate.json")
echo "Golden snapshot saved: $GOLDEN_DIR/self-validate.json ($CHECKS checks)"

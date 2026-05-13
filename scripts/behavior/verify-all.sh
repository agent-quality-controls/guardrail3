#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$HERE/../.." && pwd)"
VALIDATE_REPO_MANIFEST="$REPO_ROOT/.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml"

python3 "$HERE/verify-fixtures.py"
python3 "$HERE/verify-baselines.py"
python3 "$HERE/verify-compaction.py"
python3 "$HERE/verify-fixtures.py" --manifest "$VALIDATE_REPO_MANIFEST"
python3 "$HERE/verify-baselines.py" --manifest "$VALIDATE_REPO_MANIFEST"
python3 "$HERE/verify-rule-coverage.py"
python3 "$HERE/verify-ledger.py"
python3 "$HERE/verify-test-deletion.py"

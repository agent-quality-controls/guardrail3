#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$HERE/../.." && pwd)"
VALIDATE_REPO_MANIFEST="$REPO_ROOT/.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml"

python3 "$HERE/verify-fixtures.py" --manifest "$VALIDATE_REPO_MANIFEST"
fixture3 check --all
python3 "$HERE/verify-fixture3-migration.py"
python3 "$HERE/verify-rule-coverage.py"
python3 "$HERE/verify-ledger.py"
python3 "$HERE/verify-unclassified-fixture-coverage.py"
python3 "$HERE/verify-test-fixture-ledger.py" --strict
python3 "$HERE/classify-kept-test-dispositions.py" --check
python3 "$HERE/verify-kept-test-dispositions.py"
python3 "$HERE/verify-fixture-contract-language.py"
python3 "$HERE/verify-family-rule-fixtures.py"
python3 "$HERE/verify-g3ts-family-rule-fixtures.py"
python3 "$HERE/verify-g3ts-rule-coverage.py"
python3 "$HERE/verify-test-deletion.py"

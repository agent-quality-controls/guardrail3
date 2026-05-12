#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"

python3 "$HERE/verify-fixtures.py"
python3 "$HERE/verify-baselines.py"

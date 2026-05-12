#!/usr/bin/env bash
set -u

HERE="$(cd "$(dirname "$0")" && pwd)"

python3 "$HERE/verify-fixtures.py"

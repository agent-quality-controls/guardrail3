#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from collections import Counter
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
PLAN_MANIFEST = REPO_ROOT / ".plans" / "2026-05-15-143306-g3rs-kept-test-disposition-audit.md.manifest.toml"
SOURCE_LEDGER = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
DISPOSITION_LEDGER = REPO_ROOT / "behavior" / "migration" / "g3rs-kept-test-disposition.toml"


def main() -> int:
    failures = verify()
    if failures:
        print("kept-test-dispositions: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print("kept-test-dispositions: PASS")
    return 0


def verify() -> list[str]:
    failures: list[str] = []
    manifest = load_toml(PLAN_MANIFEST)
    source_rows = load_toml(SOURCE_LEDGER).get("test", [])
    disposition_rows = load_toml(DISPOSITION_LEDGER).get("test", [])
    if not isinstance(source_rows, list):
        return ["source ledger must define [[test]] rows"]
    if not isinstance(disposition_rows, list):
        return ["disposition ledger must define [[test]] rows"]

    kept_keys = {
        test_key(row)
        for row in source_rows
        if isinstance(row, dict) and row.get("status") == "kept_compile_contract"
    }
    disposition_by_key: dict[tuple[str, str], dict[str, Any]] = {}
    for row in disposition_rows:
        if not isinstance(row, dict):
            failures.append("disposition row must be a table")
            continue
        key = test_key(row)
        if key in disposition_by_key:
            failures.append(f"{key[0]}::{key[1]}: duplicate disposition row")
        disposition_by_key[key] = row
        if not non_empty_string(row.get("disposition")):
            failures.append(f"{key[0]}::{key[1]}: disposition must be non-empty")
        if not non_empty_string(row.get("reason")):
            failures.append(f"{key[0]}::{key[1]}: reason must be non-empty")

    for key in sorted(kept_keys - set(disposition_by_key)):
        failures.append(f"{key[0]}::{key[1]}: kept row missing disposition")
    for key in sorted(set(disposition_by_key) - kept_keys):
        failures.append(f"{key[0]}::{key[1]}: disposition row is not a kept compile-contract row")

    expected_kept = manifest["disposition_audit"]["kept_rows"]
    if len(kept_keys) != expected_kept:
        failures.append(f"kept row count drifted: expected {expected_kept}, got {len(kept_keys)}")

    expected_counts = {
        row["name"]: row["expected_rows"]
        for row in manifest.get("disposition", [])
        if isinstance(row, dict) and isinstance(row.get("name"), str)
    }
    actual_counts = Counter(
        row.get("disposition")
        for row in disposition_rows
        if isinstance(row, dict)
    )
    if set(actual_counts) != set(expected_counts):
        failures.append(f"disposition set drifted: expected {sorted(expected_counts)}, got {sorted(actual_counts)}")
    for name, expected in sorted(expected_counts.items()):
        actual = actual_counts.get(name, 0)
        if actual != expected:
            failures.append(f"{name}: expected {expected}, got {actual}")

    check = subprocess.run(
        ["python3", "scripts/behavior/classify-kept-test-dispositions.py", "--check"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if check.returncode != 0:
        failures.append(check.stdout.strip() or check.stderr.strip() or "disposition classifier check failed")
    return failures


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def test_key(row: dict[str, Any]) -> tuple[str, str]:
    test_path = row.get("test_path")
    test_name = row.get("test_name")
    if not isinstance(test_path, str) or not isinstance(test_name, str):
        return ("", "")
    return (test_path, test_name)


def non_empty_string(value: object) -> bool:
    return isinstance(value, str) and bool(value.strip())


if __name__ == "__main__":
    sys.exit(main())

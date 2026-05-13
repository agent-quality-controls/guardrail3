#!/usr/bin/env python3
from __future__ import annotations

import sys
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-test-ledger.toml"


def main() -> int:
    ledger_path = path_from_argv(sys.argv[1:])
    ledger = load_toml(ledger_path)
    rows = ledger.get("test", [])
    failures = verify_test_deletion(rows)
    if failures:
        print("behavior-test-deletion: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print(f"behavior-test-deletion: PASS rows:{len(rows)}")
    return 0


def path_from_argv(argv: list[str]) -> Path:
    if not argv:
        return DEFAULT_LEDGER_PATH
    if len(argv) == 2 and argv[0] == "--ledger":
        path = Path(argv[1])
        return path if path.is_absolute() else REPO_ROOT / path
    raise SystemExit("usage: verify-test-deletion.py [--ledger <path>]")


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify_test_deletion(rows: Any) -> list[str]:
    if not isinstance(rows, list):
        return ["ledger [[test]] entries must be a list"]
    failures: list[str] = []
    for row_index, row in enumerate(rows):
        if not isinstance(row, dict):
            failures.append(f"test row {row_index}: row must be a table")
            continue
        old_test_path = row.get("old_test_path")
        status = row.get("status")
        if not isinstance(old_test_path, str) or not old_test_path:
            failures.append(f"test row {row_index}: old_test_path must be a non-empty string")
            continue
        if Path(old_test_path).is_absolute() or ".." in Path(old_test_path).parts:
            failures.append(f"{old_test_path}: path must be repo-relative and must not escape repo")
            continue
        exists = (REPO_ROOT / old_test_path).exists()
        if isinstance(status, str) and status.startswith("deleted_"):
            if exists:
                failures.append(f"{old_test_path}: status {status} requires file to be deleted")
        elif isinstance(status, str) and status.startswith("kept_"):
            if not exists:
                failures.append(f"{old_test_path}: status {status} requires file to exist")
        elif status == "unclassified":
            if not exists:
                failures.append(f"{old_test_path}: unclassified file must still exist")
        else:
            failures.append(f"{old_test_path}: status {status} is not a deletion-verifiable status")
    return failures


if __name__ == "__main__":
    sys.exit(main())

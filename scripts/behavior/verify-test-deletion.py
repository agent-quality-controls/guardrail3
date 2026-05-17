#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections import Counter
from dataclasses import dataclass
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
DEFAULT_FIXTURE_LEDGER = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
DEFAULT_DISPOSITION_LEDGER = REPO_ROOT / "behavior" / "migration" / "g3rs-kept-test-disposition.toml"
DEFAULT_PLAN_MANIFEST = (
    REPO_ROOT / ".plans" / "2026-05-16-131516-function-level-test-deletion-gate.md.manifest.toml"
)

FIXTURE_COVERED_STATUSES = {"covered_hit", "covered_non_hit"}
DISPOSITION_COVERED = {"covered_by_cli_output", "covered_by_renderer_output"}
KNOWN_KEPT_DISPOSITIONS = {
    "keep_internal_unit_test",
    "keep_public_api_contract",
    "needs_family_runner_output",
    "needs_rule_fixture_or_golden_output",
    "needs_validate_command_output",
}


@dataclass(frozen=True, order=True)
class TestKey:
    path: str
    name: str

    def label(self) -> str:
        return f"{self.path}::{self.name}"


def main() -> int:
    args = parse_args()
    failures = verify(args.fixture_ledger, args.disposition_ledger, args.plan_manifest)
    if failures:
        print("behavior-test-deletion: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    fixture_rows = load_toml(args.fixture_ledger).get("test", [])
    active_keys = active_test_keys()
    replaceable, tracked = classify_counts(fixture_rows, load_dispositions(args.disposition_ledger))
    print(
        "behavior-test-deletion: PASS "
        f"rows:{len(fixture_rows)} active:{len(active_keys)} replaceable:{replaceable} tracked:{tracked}"
    )
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture-ledger", type=Path, default=DEFAULT_FIXTURE_LEDGER)
    parser.add_argument("--disposition-ledger", type=Path, default=DEFAULT_DISPOSITION_LEDGER)
    parser.add_argument("--plan-manifest", type=Path, default=DEFAULT_PLAN_MANIFEST)
    return parser.parse_args()


def verify(fixture_ledger_path: Path, disposition_ledger_path: Path, plan_manifest_path: Path) -> list[str]:
    failures: list[str] = []
    fixture_rows = load_test_rows(fixture_ledger_path, "fixture ledger", failures)
    disposition_rows = load_test_rows(disposition_ledger_path, "disposition ledger", failures)
    active_keys = active_test_keys()
    fixture_by_key = rows_by_key(fixture_rows, "fixture ledger", failures)
    disposition_by_key = rows_by_key(disposition_rows, "disposition ledger", failures)

    for key in sorted(active_keys):
        failures.append(f"{key.label()}: active Rust unit test remains after fixture migration")

    for key, row in sorted(fixture_by_key.items()):
        status = row.get("status")
        if status == "kept_compile_contract":
            disposition_for(key, disposition_by_key, failures)
            continue
        if status in FIXTURE_COVERED_STATUSES:
            continue
        failures.append(f"{key.label()}: unknown fixture ledger status {status}")

    for key in sorted(set(disposition_by_key) - {
        key for key, row in fixture_by_key.items() if row.get("status") == "kept_compile_contract"
    }):
        failures.append(f"{key.label()}: disposition row does not match a kept compile-contract fixture row")

    failures.extend(verify_expected_row_count(fixture_rows, plan_manifest_path))
    return failures


def verify_expected_row_count(rows: list[dict[str, Any]], plan_manifest_path: Path) -> list[str]:
    manifest = load_toml(resolve_path(plan_manifest_path))
    expected_rows = manifest.get("gate", {}).get("expected_rows")
    if not isinstance(expected_rows, int):
        return ["plan manifest gate.expected_rows must be an integer"]
    if len(rows) != expected_rows:
        return [f"fixture ledger row count drifted: expected {expected_rows}, got {len(rows)}"]
    return []


def load_test_rows(path: Path, label: str, failures: list[str]) -> list[dict[str, Any]]:
    data = load_toml(resolve_path(path))
    rows = data.get("test", [])
    if not isinstance(rows, list):
        failures.append(f"{label} must define [[test]] rows")
        return []
    output: list[dict[str, Any]] = []
    for index, row in enumerate(rows):
        if isinstance(row, dict):
            output.append(row)
        else:
            failures.append(f"{label} row {index}: row must be a table")
    return output


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def resolve_path(path: Path) -> Path:
    return path if path.is_absolute() else REPO_ROOT / path


def active_test_keys() -> set[TestKey]:
    command = [
        "python3",
        str(REPO_ROOT / "scripts" / "behavior" / "list-rust-tests.py"),
        "--format",
        "json",
    ]
    output = subprocess.check_output(command, cwd=REPO_ROOT, text=True)
    data = json.loads(output)
    return {
        TestKey(str(row["test_path"]), str(row["test_name"]))
        for row in data.get("tests", [])
        if isinstance(row, dict)
    }


def rows_by_key(
    rows: list[dict[str, Any]],
    label: str,
    failures: list[str],
) -> dict[TestKey, dict[str, Any]]:
    keys = [test_key(row, label, index, failures) for index, row in enumerate(rows)]
    counts = Counter(key for key in keys if key is not None)
    output: dict[TestKey, dict[str, Any]] = {}
    for index, row in enumerate(rows):
        key = keys[index]
        if key is None:
            continue
        if counts[key] > 1:
            failures.append(f"{key.label()}: duplicate {label} row")
            continue
        output[key] = row
    return output


def test_key(row: dict[str, Any], label: str, index: int, failures: list[str]) -> TestKey | None:
    test_path = row.get("test_path")
    test_name = row.get("test_name")
    if not isinstance(test_path, str) or not test_path:
        failures.append(f"{label} row {index}: test_path must be a non-empty string")
        return None
    if not isinstance(test_name, str) or not test_name:
        failures.append(f"{test_path}: test_name must be a non-empty string")
        return None
    if Path(test_path).is_absolute() or ".." in Path(test_path).parts:
        failures.append(f"{test_path}::{test_name}: test_path must be repo-relative")
        return None
    return TestKey(test_path, test_name)


def disposition_for(
    key: TestKey,
    disposition_by_key: dict[TestKey, dict[str, Any]],
    failures: list[str],
) -> str | None:
    row = disposition_by_key.get(key)
    if row is None:
        failures.append(f"{key.label()}: kept compile-contract row missing disposition")
        return None
    disposition = row.get("disposition")
    if not isinstance(disposition, str) or not disposition:
        failures.append(f"{key.label()}: disposition must be a non-empty string")
        return None
    if disposition not in DISPOSITION_COVERED | KNOWN_KEPT_DISPOSITIONS:
        failures.append(f"{key.label()}: unknown disposition {disposition}")
    return disposition


def load_dispositions(path: Path) -> dict[TestKey, dict[str, Any]]:
    failures: list[str] = []
    return rows_by_key(load_test_rows(path, "disposition ledger", failures), "disposition ledger", failures)


def classify_counts(
    fixture_rows: Any,
    disposition_by_key: dict[TestKey, dict[str, Any]],
) -> tuple[int, int]:
    replaceable = 0
    tracked = 0
    if not isinstance(fixture_rows, list):
        return (0, 0)
    for row in fixture_rows:
        if not isinstance(row, dict):
            continue
        status = row.get("status")
        if status in FIXTURE_COVERED_STATUSES:
            replaceable += 1
            continue
        key = test_key_without_failures(row)
        disposition = disposition_by_key.get(key, {}).get("disposition") if key else None
        if status == "kept_compile_contract" and disposition in DISPOSITION_COVERED:
            replaceable += 1
        else:
            tracked += 1
    return (replaceable, tracked)


def test_key_without_failures(row: dict[str, Any]) -> TestKey | None:
    test_path = row.get("test_path")
    test_name = row.get("test_name")
    if not isinstance(test_path, str) or not isinstance(test_name, str):
        return None
    return TestKey(test_path, test_name)


if __name__ == "__main__":
    sys.exit(main())

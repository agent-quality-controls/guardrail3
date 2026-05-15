#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
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
DEFAULT_LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
DEFAULT_FIXTURE_MANIFESTS = (
    REPO_ROOT / ".plans" / "2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml",
    REPO_ROOT / ".plans" / "2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml",
)
DEFAULT_GOLDEN_OUTPUTS = (
    REPO_ROOT / "behavior" / "golden" / "g3rs-validate" / "approved.normalized.json",
    REPO_ROOT / "behavior" / "golden" / "g3rs-validate-repo" / "approved.normalized.json",
)

ALLOWED_STATUSES = {
    "covered_hit",
    "covered_non_hit",
    "kept_compile_contract",
    "kept_replay_system",
    "not_cli_visible",
    "unclassified",
}
FINDING_STATUSES = {"covered_hit", "covered_non_hit"}


def main() -> int:
    args = parse_args()
    failures = verify(args.ledger, args.strict)
    if failures:
        print("behavior-test-fixture-ledger: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    ledger = load_toml(args.ledger)
    rows = ledger.get("test", [])
    status_counts = Counter(row.get("status") for row in rows if isinstance(row, dict))
    print(
        "behavior-test-fixture-ledger: PASS "
        f"tests:{len(rows)} "
        f"unclassified:{status_counts.get('unclassified', 0)} "
        f"strict:{str(args.strict).lower()}"
    )
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--ledger", type=Path, default=DEFAULT_LEDGER_PATH)
    parser.add_argument("--strict", action="store_true")
    return parser.parse_args()


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify(ledger_path: Path, strict: bool) -> list[str]:
    failures: list[str] = []
    if not ledger_path.is_file():
        return [f"missing ledger: {ledger_path.relative_to(REPO_ROOT)}"]

    active_tests = active_test_keys()
    ledger = load_toml(ledger_path)
    rows = ledger.get("test", [])
    if not isinstance(rows, list):
        return ["ledger must define [[test]] rows"]

    fixture_ids = load_fixture_ids()
    findings_by_fixture = load_findings_by_fixture()
    row_keys: list[tuple[str, str]] = []
    unclassified = 0

    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            failures.append(f"row {index}: row must be a table")
            continue
        test_path = row.get("test_path")
        test_name = row.get("test_name")
        status = row.get("status")
        if not isinstance(test_path, str) or not test_path:
            failures.append(f"row {index}: test_path must be a non-empty string")
            continue
        if not isinstance(test_name, str) or not test_name:
            failures.append(f"{test_path}: test_name must be a non-empty string")
            continue
        row_keys.append((test_path, test_name))
        if Path(test_path).is_absolute() or ".." in Path(test_path).parts:
            failures.append(f"{test_path}::{test_name}: test_path must be repo-relative")
        if status not in ALLOWED_STATUSES:
            failures.append(f"{test_path}::{test_name}: invalid status {status}")
            continue
        if status == "unclassified":
            unclassified += 1
        if status in FINDING_STATUSES:
            failures.extend(verify_fixture_row(row, fixture_ids, findings_by_fixture))
        if status == "not_cli_visible" and not non_empty_string(row.get("reason")):
            failures.append(f"{test_path}::{test_name}: not_cli_visible rows require reason")
        if status in {"kept_compile_contract", "kept_replay_system"} and not non_empty_string(row.get("reason")):
            failures.append(f"{test_path}::{test_name}: {status} rows require reason")

    duplicates = sorted(key for key, count in Counter(row_keys).items() if count > 1)
    for test_path, test_name in duplicates:
        failures.append(f"{test_path}::{test_name}: duplicate ledger row")

    row_key_set = set(row_keys)
    for test_path, test_name in sorted(active_tests - row_key_set):
        failures.append(f"{test_path}::{test_name}: active test missing from ledger")
    for test_path, test_name in sorted(row_key_set - active_tests):
        failures.append(f"{test_path}::{test_name}: ledger row has no active test")

    if strict and unclassified:
        failures.append(f"strict mode forbids unclassified rows: {unclassified}")

    return failures


def active_test_keys() -> set[tuple[str, str]]:
    command = [
        "python3",
        str(REPO_ROOT / "scripts" / "behavior" / "list-rust-tests.py"),
        "--format",
        "json",
    ]
    output = subprocess.check_output(command, cwd=REPO_ROOT, text=True)
    data = json.loads(output)
    return {
        (str(row["test_path"]), str(row["test_name"]))
        for row in data.get("tests", [])
        if isinstance(row, dict)
    }


def load_fixture_ids() -> set[str]:
    fixture_ids: set[str] = set()
    for manifest_path in DEFAULT_FIXTURE_MANIFESTS:
        manifest = load_toml(manifest_path)
        for row in manifest.get("fixture", []):
            if isinstance(row, dict) and isinstance(row.get("id"), str):
                fixture_ids.add(row["id"])
    return fixture_ids


def load_findings_by_fixture() -> dict[str, Counter[tuple[str, str, str, str]]]:
    findings: dict[str, Counter[tuple[str, str, str, str]]] = {}
    for golden_output in DEFAULT_GOLDEN_OUTPUTS:
        data = json.loads(golden_output.read_text(encoding="utf-8"))
        for record in data.get("records", []):
            fixture_id = record.get("fixture_id")
            if not isinstance(fixture_id, str):
                continue
            counter = findings.setdefault(fixture_id, Counter())
            for line in str(record.get("stdout", "")).splitlines():
                parsed = parse_finding_line(line)
                if parsed is not None:
                    counter[parsed] += 1
    return findings


def parse_finding_line(line: str) -> tuple[str, str, str, str] | None:
    if not line.startswith("["):
        return None
    parts = line.split(" ", 3)
    if len(parts) != 4:
        return None
    severity = parts[0].strip("[]")
    if severity not in {"Error", "Warn", "Info"}:
        return None
    rule = parts[1]
    file_path = parts[2]
    title = parts[3]
    if not rule.startswith("g3rs-"):
        return None
    return (severity, rule, title, file_path)


def verify_fixture_row(
    row: dict[str, Any],
    fixture_ids: set[str],
    findings_by_fixture: dict[str, Counter[tuple[str, str, str, str]]],
) -> list[str]:
    test_id = f"{row.get('test_path')}::{row.get('test_name')}"
    failures: list[str] = []
    fixture = row.get("fixture")
    rule = row.get("rule")
    if not isinstance(fixture, str) or fixture not in fixture_ids:
        failures.append(f"{test_id}: unknown fixture {fixture}")
        return failures
    if not isinstance(rule, str) or not rule:
        failures.append(f"{test_id}: {row.get('status')} rows require rule")
        return failures
    expected = expected_finding(row)
    fixture_findings = findings_by_fixture.get(fixture, Counter())
    if row.get("status") == "covered_hit":
        if expected is None:
            failures.append(f"{test_id}: covered_hit rows require severity, rule, title, and file")
        elif fixture_findings[expected] < 1:
            failures.append(f"{test_id}: expected fixture finding missing: {expected}")
    elif row.get("status") == "covered_non_hit":
        if expected is None:
            if any(finding[1] == rule for finding in fixture_findings):
                failures.append(f"{test_id}: expected no `{rule}` findings in {fixture}")
        elif fixture_findings[expected] > 0:
            failures.append(f"{test_id}: forbidden fixture finding present: {expected}")
    return failures


def expected_finding(row: dict[str, Any]) -> tuple[str, str, str, str] | None:
    severity = row.get("severity")
    rule = row.get("rule")
    title = row.get("title")
    file_path = row.get("file")
    if all(isinstance(value, str) and value for value in (severity, rule, title, file_path)):
        return (severity, rule, title, file_path)
    return None


def non_empty_string(value: object) -> bool:
    return isinstance(value, str) and bool(value.strip())


if __name__ == "__main__":
    sys.exit(main())

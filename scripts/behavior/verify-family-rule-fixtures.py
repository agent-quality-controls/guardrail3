#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from collections import defaultdict
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
MANIFEST_PATH = REPO_ROOT / ".plans" / "2026-05-16-185717-family-rule-cli-fixtures.md.manifest.toml"
APPROVED_OUTPUT_PATH = REPO_ROOT / "behavior" / "golden" / "g3rs-validate" / "approved.normalized.json"
DISPOSITION_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-kept-test-disposition.toml"
RULE_ID_RE = re.compile(r'const ID: &str = "([^"]+)";')


def main() -> int:
    failures: list[str] = []
    manifest = load_toml(MANIFEST_PATH)
    fixture_root = REPO_ROOT / manifest["target"]["fixture_root"]
    rule_ids = discover_rule_ids()
    fixture_rows = load_fixture_rows(fixture_root, failures)
    approved_records = load_approved_records(failures)

    verify_one_clean_golden_per_family(fixture_rows, failures)
    verify_fixture_rows(fixture_rows, rule_ids, approved_records, failures)
    verify_completed_family_rule_breakage(manifest, fixture_rows, approved_records, rule_ids, failures)
    verify_completed_family_ledger_coverage(manifest, fixture_rows, rule_ids, failures)

    if failures:
        print("family-rule-fixtures: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    families = sorted({row["rule_family"] for row in fixture_rows})
    print(
        "family-rule-fixtures: PASS "
        f"families:{','.join(families) if families else '-'} fixtures:{len(fixture_rows)}"
    )
    return 0


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def discover_rule_ids() -> set[str]:
    rule_ids: set[str] = set()
    for path in (REPO_ROOT / "packages" / "rs").glob("**/*.rs"):
        if "/target/" in path.as_posix():
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        rule_ids.update(RULE_ID_RE.findall(text))
    return rule_ids


def load_fixture_rows(fixture_root: Path, failures: list[str]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in sorted(fixture_root.glob("*/*/fixture.toml")):
        row = load_toml(path)
        row["_path"] = path.relative_to(REPO_ROOT).as_posix()
        row["_fixture_dir"] = path.parent.relative_to(REPO_ROOT).as_posix()
        rows.append(row)
    if not rows:
        failures.append(f"{fixture_root.relative_to(REPO_ROOT)}: no family-rule fixtures found")
    return rows


def load_approved_records(failures: list[str]) -> dict[str, dict[str, Any]]:
    if not APPROVED_OUTPUT_PATH.exists():
        failures.append(f"{APPROVED_OUTPUT_PATH.relative_to(REPO_ROOT)}: approved output missing")
        return {}
    import json

    data = json.loads(APPROVED_OUTPUT_PATH.read_text(encoding="utf-8"))
    records = data.get("records", [])
    if not isinstance(records, list):
        failures.append(f"{APPROVED_OUTPUT_PATH.relative_to(REPO_ROOT)}: records must be a list")
        return {}
    output: dict[str, dict[str, Any]] = {}
    for record in records:
        if isinstance(record, dict) and isinstance(record.get("fixture_id"), str):
            output[record["fixture_id"]] = record
    return output


def verify_one_clean_golden_per_family(rows: list[dict[str, Any]], failures: list[str]) -> None:
    by_family: dict[str, list[str]] = defaultdict(list)
    for row in rows:
        family = string_field(row, "rule_family", failures)
        if not family:
            continue
        if row.get("level") == "family_rule_clean_golden":
            by_family[family].append(str(row.get("id", row.get("_path"))))
    for family in sorted({str(row.get("rule_family")) for row in rows if row.get("rule_family")}):
        golden = by_family.get(family, [])
        if len(golden) != 1:
            failures.append(f"{family}: expected exactly one clean golden fixture, got {len(golden)}")


def verify_fixture_rows(
    rows: list[dict[str, Any]],
    rule_ids: set[str],
    approved_records: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    seen_ids: set[str] = set()
    for row in rows:
        path = str(row.get("_path"))
        fixture_id = string_field(row, "id", failures)
        family = string_field(row, "rule_family", failures)
        target_rules = string_list_field(row, "target_rules", failures)
        expected_findings = string_list_field(row, "expected_findings", failures)
        if fixture_id in seen_ids:
            failures.append(f"{path}: duplicate fixture id {fixture_id}")
        seen_ids.add(fixture_id)
        if family and target_rules and f"g3rs-{family}/" not in "\n".join(target_rules):
            failures.append(f"{path}: target_rules must contain g3rs-{family}/ rule IDs")
        missing_from_expected = sorted(set(target_rules) - set(expected_findings))
        for rule_id in missing_from_expected:
            failures.append(f"{path}: target rule {rule_id} missing from expected_findings")
        for rule_id in sorted(set(target_rules) | set(expected_findings)):
            if rule_id not in rule_ids:
                failures.append(f"{path}: unknown rule id {rule_id}")
        verify_approved_record(row, approved_records, failures)


def verify_approved_record(
    row: dict[str, Any],
    approved_records: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    fixture_id = str(row.get("id"))
    record = approved_records.get(fixture_id)
    path = str(row.get("_path"))
    if record is None:
        failures.append(f"{path}: fixture {fixture_id} missing from approved g3rs-validate output")
        return
    expected_exit = row.get("expected_exit")
    exit_code = record.get("exit_code")
    if expected_exit == "zero" and exit_code != 0:
        failures.append(f"{path}: expected exit 0, got {exit_code}")
    if expected_exit == "nonzero" and exit_code == 0:
        failures.append(f"{path}: expected nonzero exit, got 0")
    stdout = record.get("stdout")
    if not isinstance(stdout, str):
        failures.append(f"{path}: approved stdout must be a string")
        return
    for rule_id in string_list_field(row, "expected_findings", failures):
        if rule_id not in stdout:
            failures.append(f"{path}: approved output missing expected finding {rule_id}")
    if row.get("level") != "family_rule_clean_golden":
        verify_target_rules_are_broken(row, stdout, failures)


def verify_target_rules_are_broken(
    row: dict[str, Any],
    stdout: str,
    failures: list[str],
) -> None:
    path = str(row.get("_path"))
    broken_rule_ids = {
        match.group("rule_id")
        for match in re.finditer(
            r"^\[(?:Error|Warn)\] (?P<rule_id>g3rs-[^ ]+)",
            stdout,
            flags=re.MULTILINE,
        )
    }
    for rule_id in string_list_field(row, "target_rules", failures):
        if rule_id not in broken_rule_ids:
            failures.append(f"{path}: target rule {rule_id} did not emit Error or Warn")


def verify_completed_family_ledger_coverage(
    manifest: dict[str, Any],
    rows: list[dict[str, Any]],
    rule_ids: set[str],
    failures: list[str],
) -> None:
    completed = {
        family["name"]
        for family in manifest.get("family", [])
        if isinstance(family, dict) and family.get("status") == "completed"
    }
    if not completed:
        return
    targeted = {
        rule_id
        for row in rows
        for rule_id in row.get("target_rules", [])
        if isinstance(rule_id, str)
    }
    dispositions = load_toml(DISPOSITION_PATH).get("test", [])
    for row in dispositions:
        if not isinstance(row, dict):
            continue
        if row.get("disposition") != "needs_rule_fixture_or_golden_output":
            continue
        rule_id = rule_id_for_test_row(row)
        if rule_id is None:
            continue
        if rule_id not in rule_ids:
            continue
        family = rule_id.split("/", maxsplit=1)[0].removeprefix("g3rs-")
        if family in completed and rule_id not in targeted:
            failures.append(f"{row.get('test_path')}::{row.get('test_name')}: no fixture targets {rule_id}")


def verify_completed_family_rule_breakage(
    manifest: dict[str, Any],
    rows: list[dict[str, Any]],
    approved_records: dict[str, dict[str, Any]],
    rule_ids: set[str],
    failures: list[str],
) -> None:
    completed = {
        family["name"]
        for family in manifest.get("family", [])
        if isinstance(family, dict) and family.get("status") == "completed"
    }
    if not completed:
        return
    inventory_only = {
        row["id"]
        for row in manifest.get("inventory_only_rule", [])
        if isinstance(row, dict) and isinstance(row.get("id"), str)
    }
    for rule_id in sorted(inventory_only - rule_ids):
        failures.append(f"inventory-only rule {rule_id} is not an active rule ID")
    broken_by_family: dict[str, set[str]] = defaultdict(set)
    for row in rows:
        if row.get("level") == "family_rule_clean_golden":
            continue
        fixture_id = str(row.get("id"))
        record = approved_records.get(fixture_id)
        if record is None or not isinstance(record.get("stdout"), str):
            continue
        for rule_id in broken_rule_ids(record["stdout"]):
            family = rule_id.split("/", maxsplit=1)[0].removeprefix("g3rs-")
            broken_by_family[family].add(rule_id)
    for family in sorted(completed):
        expected = {
            rule_id
            for rule_id in rule_ids
            if rule_id.startswith(f"g3rs-{family}/")
        } - inventory_only
        missing = sorted(expected - broken_by_family.get(family, set()))
        for rule_id in missing:
            failures.append(f"{family}: completed family rule {rule_id} is not broken by any fixture")


def rule_id_for_test_row(row: dict[str, Any]) -> str | None:
    test_path = row.get("test_path")
    if not isinstance(test_path, str):
        return None
    parts = Path(test_path).parts
    if len(parts) < 7 or parts[0] != "packages" or parts[1] != "rs":
        return None
    family = parts[2]
    test_dir = Path(test_path).parent.name
    if not test_dir.endswith("_tests"):
        return None
    return f"g3rs-{family}/{test_dir.removesuffix('_tests').replace('_', '-')}"


def broken_rule_ids(stdout: str) -> set[str]:
    return {
        match.group("rule_id")
        for match in re.finditer(
            r"^\[(?:Error|Warn)\] (?P<rule_id>g3rs-[^ ]+)",
            stdout,
            flags=re.MULTILINE,
        )
    }


def string_field(row: dict[str, Any], name: str, failures: list[str]) -> str:
    value = row.get(name)
    if isinstance(value, str) and value:
        return value
    failures.append(f"{row.get('_path', '<fixture>')}: {name} must be a non-empty string")
    return ""


def string_list_field(row: dict[str, Any], name: str, failures: list[str]) -> list[str]:
    value = row.get(name)
    if isinstance(value, list) and all(isinstance(item, str) and item for item in value):
        return list(value)
    failures.append(f"{row.get('_path', '<fixture>')}: {name} must be a list of non-empty strings")
    return []


if __name__ == "__main__":
    sys.exit(main())

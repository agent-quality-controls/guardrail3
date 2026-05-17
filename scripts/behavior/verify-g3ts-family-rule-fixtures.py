#!/usr/bin/env python3
from __future__ import annotations

import json
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
MANIFEST_PATH = REPO_ROOT / ".plans" / "2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml"
APPROVED_OUTPUT_PATH = REPO_ROOT / "behavior" / "golden" / "g3ts-rule" / "approved.normalized.json"
RULE_PATTERN = re.compile(r"g3ts-[a-z0-9-]+(?:-[a-z0-9-]+)*/[a-z0-9-]+(?:-[a-z0-9-]+)*")


def main() -> int:
    failures: list[str] = []
    manifest = load_toml(MANIFEST_PATH)
    fixture_root = REPO_ROOT / manifest["target"]["fixture_root"]
    rule_ids = discover_rule_ids()
    fixture_rows = load_fixture_rows(fixture_root, failures)
    approved_records = load_approved_records(failures)

    verify_fixture_rows(fixture_rows, rule_ids, approved_records, failures)
    verify_completed_families(manifest, fixture_rows, approved_records, rule_ids, failures)

    if failures:
        print("g3ts-family-rule-fixtures: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    families = sorted({row["rule_family"] for row in fixture_rows})
    print(
        "g3ts-family-rule-fixtures: PASS "
        f"families:{','.join(families) if families else '-'} fixtures:{len(fixture_rows)}"
    )
    return 0


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def discover_rule_ids() -> set[str]:
    rule_ids: set[str] = set()
    for path in (REPO_ROOT / "packages" / "ts").glob("**/*.rs"):
        if should_skip_rule_source(path):
            continue
        rule_ids.update(RULE_PATTERN.findall(path.read_text(encoding="utf-8", errors="replace")))
    return rule_ids


def should_skip_rule_source(path: Path) -> bool:
    parts = set(path.relative_to(REPO_ROOT).parts)
    if parts & {"target", "tests", "contract_tests", "assertions"}:
        return True
    return any(part.endswith("_tests") for part in parts)


def load_fixture_rows(fixture_root: Path, failures: list[str]) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    for path in sorted(fixture_root.glob("*/*/fixture.toml")):
        row = load_toml(path)
        row["_path"] = path.relative_to(REPO_ROOT).as_posix()
        rows.append(row)
    if not rows:
        failures.append(f"{fixture_root.relative_to(REPO_ROOT)}: no G3TS family-rule fixtures found")
    return rows


def load_approved_records(failures: list[str]) -> dict[str, dict[str, Any]]:
    if not APPROVED_OUTPUT_PATH.exists():
        failures.append(f"{APPROVED_OUTPUT_PATH.relative_to(REPO_ROOT)}: approved output missing")
        return {}
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


def verify_fixture_rows(
    rows: list[dict[str, Any]],
    rule_ids: set[str],
    approved_records: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    seen_ids: set[str] = set()
    clean_by_family: dict[str, list[str]] = defaultdict(list)
    for row in rows:
        path = str(row.get("_path"))
        fixture_id = string_field(row, "id", failures)
        family = string_field(row, "rule_family", failures)
        target_rules = string_list_field(row, "target_rules", failures)
        expected_findings = string_list_field(row, "expected_findings", failures)
        if fixture_id in seen_ids:
            failures.append(f"{path}: duplicate fixture id {fixture_id}")
        seen_ids.add(fixture_id)
        if row.get("tool") != "g3ts":
            failures.append(f"{path}: tool must be g3ts")
        if row.get("level") == "family_rule_clean_golden":
            clean_by_family[family].append(fixture_id)
        if family and target_rules and f"g3ts-{family}/" not in "\n".join(target_rules):
            failures.append(f"{path}: target_rules must contain g3ts-{family}/ rule IDs")
        for rule_id in sorted(set(target_rules) - set(expected_findings)):
            failures.append(f"{path}: target rule {rule_id} missing from expected_findings")
        for rule_id in sorted(set(target_rules) | set(expected_findings)):
            if rule_id not in rule_ids:
                failures.append(f"{path}: unknown rule id {rule_id}")
        verify_approved_record(row, approved_records, failures)

    for family, clean_ids in sorted(clean_by_family.items()):
        if len(clean_ids) != 1:
            failures.append(f"{family}: expected exactly one clean golden fixture, got {len(clean_ids)}")


def verify_approved_record(
    row: dict[str, Any],
    approved_records: dict[str, dict[str, Any]],
    failures: list[str],
) -> None:
    fixture_id = str(row.get("id"))
    path = str(row.get("_path"))
    record = approved_records.get(fixture_id)
    if record is None:
        failures.append(f"{path}: fixture {fixture_id} missing from approved G3TS output")
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
        broken = broken_rule_ids(stdout)
        for rule_id in string_list_field(row, "target_rules", failures):
            if rule_id not in broken:
                failures.append(f"{path}: target rule {rule_id} did not emit Error or Warn")


def verify_completed_families(
    manifest: dict[str, Any],
    rows: list[dict[str, Any]],
    approved_records: dict[str, dict[str, Any]],
    rule_ids: set[str],
    failures: list[str],
) -> None:
    completed = {
        row["name"]
        for row in manifest.get("family", [])
        if isinstance(row, dict) and row.get("status") == "completed"
    }
    if not completed:
        return
    inventory_only = listed_rule_ids(manifest, "inventory_only_rule")
    cli_unreachable = listed_rule_ids(manifest, "cli_unreachable_rule")
    for rule_id in sorted((inventory_only | cli_unreachable) - rule_ids):
        failures.append(f"manifest-listed rule does not exist in source: {rule_id}")

    clean_by_family: dict[str, list[str]] = defaultdict(list)
    broken_by_family: dict[str, set[str]] = defaultdict(set)
    for row in rows:
        family = str(row.get("rule_family"))
        if row.get("level") == "family_rule_clean_golden":
            clean_by_family[family].append(str(row.get("id")))
            continue
        record = approved_records.get(str(row.get("id")))
        if record is None or not isinstance(record.get("stdout"), str):
            continue
        broken_by_family[family].update(broken_rule_ids(record["stdout"]))

    for family in sorted(completed):
        if len(clean_by_family.get(family, [])) != 1:
            failures.append(f"{family}: completed family must have exactly one clean fixture")
        expected = {
            rule_id
            for rule_id in rule_ids
            if rule_id.startswith(f"g3ts-{family}/")
            and rule_id not in inventory_only
            and rule_id not in cli_unreachable
        }
        missing = sorted(expected - broken_by_family.get(family, set()))
        for rule_id in missing:
            failures.append(f"{family}: no broken fixture emits {rule_id}")


def listed_rule_ids(manifest: dict[str, Any], key: str) -> set[str]:
    return {
        row["id"]
        for row in manifest.get(key, [])
        if isinstance(row, dict) and isinstance(row.get("id"), str)
    }


def broken_rule_ids(stdout: str) -> set[str]:
    return {
        match.group("rule_id")
        for match in re.finditer(
            r"^\[(?:Error|Warn)\] (?P<rule_id>g3ts-[^ ]+)",
            stdout,
            flags=re.MULTILINE,
        )
    }


def string_field(row: dict[str, Any], name: str, failures: list[str]) -> str:
    value = row.get(name)
    if isinstance(value, str) and value:
        return value
    failures.append(f"{row.get('_path', '<unknown>')}: {name} must be a non-empty string")
    return ""


def string_list_field(row: dict[str, Any], name: str, failures: list[str]) -> list[str]:
    value = row.get(name)
    if isinstance(value, list) and all(isinstance(item, str) and item for item in value):
        return value
    failures.append(f"{row.get('_path', '<unknown>')}: {name} must be a list of non-empty strings")
    return []


if __name__ == "__main__":
    sys.exit(main())

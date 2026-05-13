#!/usr/bin/env python3
from __future__ import annotations

import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
PLAN_MANIFEST_PATH = (
    REPO_ROOT / ".plans" / "2026-05-13-160231-g3rs-replay-coverage-matrix.md.manifest.toml"
)
RULE_PATTERN = re.compile(r"g3rs-[a-z0-9-]+/[a-z0-9-]+")
FINDING_PATTERN = re.compile(r"\[(Error|Warn|Info)\] (g3rs-[^ ]+)")
FINDING_LINE_PATTERN = re.compile(r"^\[(Error|Warn|Info)\] (g3rs-[^ ]+) ([^ ]+) (.+)$")


def main() -> int:
    plan_manifest_path = path_from_argv(sys.argv[1:])
    plan_manifest = load_toml(plan_manifest_path)
    failures = verify_rule_coverage(plan_manifest)
    if failures:
        print("behavior-rule-coverage: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    matrix = load_toml(REPO_ROOT / plan_manifest["coverage_matrix"]["path"])
    rows = matrix["rule"]
    covered = sum(1 for row in rows if row["coverage_status"] == "covered")
    planned = len(rows) - covered
    print(f"behavior-rule-coverage: PASS source:{len(rows)} covered:{covered} planned:{planned}")
    return 0


def path_from_argv(argv: list[str]) -> Path:
    if not argv:
        return PLAN_MANIFEST_PATH
    if len(argv) == 2 and argv[0] == "--manifest":
        path = Path(argv[1])
        return path if path.is_absolute() else REPO_ROOT / path
    raise SystemExit("usage: verify-rule-coverage.py [--manifest <path>]")


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def verify_rule_coverage(plan_manifest: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    source_ids = source_rule_ids(plan_manifest)
    baseline_state_by_fixture, baseline_findings_by_fixture = baseline_rule_state_by_fixture(plan_manifest)
    baseline_state = aggregate_rule_state(baseline_state_by_fixture)
    fixture_ids, required_results_by_fixture = fixture_contracts_from_manifests(plan_manifest, failures)
    matrix_path = REPO_ROOT / plan_manifest["coverage_matrix"]["path"]
    if not matrix_path.is_file():
        return [f"coverage matrix missing: {matrix_path.relative_to(REPO_ROOT)}"]
    matrix = load_toml(matrix_path)
    rows = matrix.get("rule", [])
    if not isinstance(rows, list):
        return ["coverage matrix must define [[rule]] rows"]
    failures.extend(verify_counts(plan_manifest, source_ids, baseline_state))
    failures.extend(
        verify_rows(
            plan_manifest,
            rows,
            source_ids,
            baseline_state,
            baseline_state_by_fixture,
            baseline_findings_by_fixture,
            fixture_ids,
            required_results_by_fixture,
        )
    )
    return failures


def source_rule_ids(plan_manifest: dict[str, Any]) -> set[str]:
    rule_ids: set[str] = set()
    for root_name in plan_manifest["coverage_matrix"]["source_roots"]:
        root = REPO_ROOT / root_name
        if not root.is_dir():
            continue
        for path in root.rglob("*.rs"):
            if not is_active_rule_source(path):
                continue
            rule_ids.update(RULE_PATTERN.findall(path.read_text(encoding="utf-8", errors="ignore")))
    return rule_ids


def is_active_rule_source(path: Path) -> bool:
    excluded_names = {"target", ".cargo-target", "tests", "rule_tests", "contract_tests", "assertions"}
    parts = set(path.relative_to(REPO_ROOT).parts)
    if parts & excluded_names:
        return False
    return not any(part.endswith("_tests") for part in parts)


def baseline_rule_state_by_fixture(
    plan_manifest: dict[str, Any],
) -> tuple[dict[str, dict[str, set[str]]], dict[str, Counter[tuple[str, str, str, str]]]]:
    states: dict[str, dict[str, set[str]]] = defaultdict(lambda: defaultdict(set))
    findings: dict[str, Counter[tuple[str, str, str, str]]] = defaultdict(Counter)
    for golden_output in plan_manifest["coverage_matrix"]["golden_outputs"]:
        path = REPO_ROOT / golden_output
        data = json.loads(path.read_text(encoding="utf-8"))
        for record in data.get("records", []):
            fixture_id = record.get("fixture_id")
            if not isinstance(fixture_id, str):
                continue
            for line in record.get("stdout", "").splitlines():
                match = FINDING_PATTERN.match(line)
                if not match:
                    continue
                severity, rule_id = match.groups()
                states[fixture_id][rule_id].add("error_or_warn" if severity in {"Error", "Warn"} else "info")
                finding_match = FINDING_LINE_PATTERN.match(line)
                if finding_match:
                    finding_severity, finding_rule_id, file_path, title = finding_match.groups()
                    findings[fixture_id][(finding_severity, finding_rule_id, title, file_path)] += 1
    return states, findings


def aggregate_rule_state(
    baseline_state_by_fixture: dict[str, dict[str, set[str]]],
) -> dict[str, set[str]]:
    states: dict[str, set[str]] = defaultdict(set)
    for fixture_state in baseline_state_by_fixture.values():
        for rule_id, rule_states in fixture_state.items():
            states[rule_id].update(rule_states)
    return states


def fixture_contracts_from_manifests(
    plan_manifest: dict[str, Any],
    failures: list[str],
) -> tuple[set[str], dict[str, Counter[tuple[str, str, str, str]]]]:
    fixture_ids: set[str] = set()
    required_results_by_fixture: dict[str, Counter[tuple[str, str, str, str]]] = defaultdict(Counter)
    for manifest_name in plan_manifest["coverage_matrix"]["fixture_manifests"]:
        path = REPO_ROOT / manifest_name
        if not path.is_file():
            failures.append(f"fixture manifest missing: {manifest_name}")
            continue
        manifest = load_toml(path)
        for row in manifest.get("fixture", []):
            if isinstance(row, dict) and isinstance(row.get("id"), str):
                fixture_id = row["id"]
                fixture_ids.add(fixture_id)
                for required_result in row.get("required_results", []):
                    if not isinstance(required_result, str):
                        continue
                    parts = required_result.split("|")
                    if len(parts) == 4:
                        required_results_by_fixture[fixture_id][(parts[0], parts[1], parts[2], parts[3])] += 1
    return fixture_ids, required_results_by_fixture


def verify_counts(
    plan_manifest: dict[str, Any],
    source_ids: set[str],
    baseline_state: dict[str, set[str]],
) -> list[str]:
    expected = plan_manifest["expected_counts"]
    baseline_ids = set(baseline_state)
    baseline_error_warn = {rule_id for rule_id, states in baseline_state.items() if "error_or_warn" in states}
    info_only = {rule_id for rule_id, states in baseline_state.items() if states == {"info"}}
    absent = source_ids - baseline_ids
    actual_counts = {
        "source_rule_ids": len(source_ids),
        "baseline_rule_ids": len(baseline_ids),
        "baseline_error_warn_rule_ids": len(baseline_error_warn),
        "info_only_rule_ids": len(info_only),
        "absent_rule_ids": len(absent),
    }
    failures: list[str] = []
    for key, actual in actual_counts.items():
        if expected.get(key) != actual:
            failures.append(f"{key}: expected {expected.get(key)}, got {actual}")
    return failures


def verify_rows(
    plan_manifest: dict[str, Any],
    rows: list[Any],
    source_ids: set[str],
    baseline_state: dict[str, set[str]],
    baseline_state_by_fixture: dict[str, dict[str, set[str]]],
    baseline_findings_by_fixture: dict[str, Counter[tuple[str, str, str, str]]],
    fixture_ids: set[str],
    required_results_by_fixture: dict[str, Counter[tuple[str, str, str, str]]],
) -> list[str]:
    allowed = plan_manifest["allowed_values"]
    failures: list[str] = []
    row_by_id: dict[str, dict[str, Any]] = {}
    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            failures.append(f"row {index}: must be a table")
            continue
        rule_id = row.get("id")
        if not isinstance(rule_id, str) or not rule_id:
            failures.append(f"row {index}: id must be a non-empty string")
            continue
        if rule_id in row_by_id:
            failures.append(f"{rule_id}: duplicate coverage row")
        row_by_id[rule_id] = row
        failures.extend(verify_row_schema(row, allowed))
        failures.extend(
            verify_row_state(
                row,
                baseline_state,
                baseline_state_by_fixture,
                baseline_findings_by_fixture,
                fixture_ids,
                required_results_by_fixture,
            )
        )

    missing = sorted(source_ids - set(row_by_id))
    extra = sorted(set(row_by_id) - source_ids)
    for rule_id in missing:
        failures.append(f"{rule_id}: source rule ID missing from coverage matrix")
    for rule_id in extra:
        failures.append(f"{rule_id}: coverage matrix row has no active source rule ID")
    return failures


def verify_row_schema(row: dict[str, Any], allowed: dict[str, list[str]]) -> list[str]:
    failures: list[str] = []
    required = ["id", "family", "coverage_status", "current_replay", "target_replay", "fixture", "reason"]
    for key in required:
        if not isinstance(row.get(key), str):
            failures.append(f"{row.get('id', '<missing>')}: {key} must be a string")
    rule_id = row.get("id")
    if isinstance(rule_id, str) and isinstance(row.get("family"), str):
        expected_family = rule_id.split("/")[0]
        if row["family"] != expected_family:
            failures.append(f"{rule_id}: family must be {expected_family}")
    for key in ("coverage_status", "current_replay", "target_replay"):
        value = row.get(key)
        if isinstance(value, str) and value not in allowed[key]:
            failures.append(f"{rule_id}: invalid {key} {value}")
    return failures


def verify_row_state(
    row: dict[str, Any],
    baseline_state: dict[str, set[str]],
    baseline_state_by_fixture: dict[str, dict[str, set[str]]],
    baseline_findings_by_fixture: dict[str, Counter[tuple[str, str, str, str]]],
    fixture_ids: set[str],
    required_results_by_fixture: dict[str, Counter[tuple[str, str, str, str]]],
) -> list[str]:
    rule_id = row.get("id")
    current = row.get("current_replay")
    target = row.get("target_replay")
    status = row.get("coverage_status")
    fixture = row.get("fixture")
    reason = row.get("reason")
    states = baseline_state.get(rule_id, set())
    failures: list[str] = []
    if current == "absent" and states:
        failures.append(f"{rule_id}: current_replay absent but baseline emits {sorted(states)}")
    if current == "info_only" and states != {"info"}:
        failures.append(f"{rule_id}: current_replay info_only but baseline emits {sorted(states)}")
    if current == "error_or_warn" and "error_or_warn" not in states:
        failures.append(f"{rule_id}: current_replay error_or_warn but baseline does not emit Error/Warn")
    inventory_covered = current == "info_only" and target == "info_inventory"
    if status == "covered" and current != target and not inventory_covered:
        failures.append(f"{rule_id}: covered row must have current_replay equal target_replay")
    if status == "covered" and not fixture:
        failures.append(f"{rule_id}: covered row must name fixture")
    if fixture and fixture not in fixture_ids:
        failures.append(f"{rule_id}: fixture {fixture} is not in fixture manifests")
    if status == "covered" and isinstance(fixture, str) and fixture in fixture_ids:
        fixture_states = baseline_state_by_fixture.get(fixture, {}).get(rule_id, set())
        failures.extend(verify_fixture_state(rule_id, current, fixture, fixture_states))
        if target == "info_inventory":
            failures.extend(
                verify_info_inventory_required_result(
                    rule_id,
                    fixture,
                    baseline_findings_by_fixture.get(fixture, Counter()),
                    required_results_by_fixture.get(fixture, Counter()),
                )
            )
    if status != "covered" and not reason:
        failures.append(f"{rule_id}: non-covered row must include reason")
    return failures


def verify_info_inventory_required_result(
    rule_id: object,
    fixture: str,
    baseline_findings: Counter[tuple[str, str, str, str]],
    required_results: Counter[tuple[str, str, str, str]],
) -> list[str]:
    if not isinstance(rule_id, str):
        return []
    info_findings = Counter(
        {finding: count for finding, count in baseline_findings.items() if finding[0] == "Info" and finding[1] == rule_id}
    )
    pinned_info_findings = Counter(
        {finding: count for finding, count in required_results.items() if finding[0] == "Info" and finding[1] == rule_id}
    )
    if not pinned_info_findings:
        return [f"{rule_id}: fixture {fixture} must pin an exact Info required_result row"]
    extra = sorted((info_findings - pinned_info_findings).items())
    if extra:
        return [f"{rule_id}: fixture {fixture} emits unpinned Info rows: {extra}"]
    missing = sorted((pinned_info_findings - info_findings).items())
    if missing:
        return [f"{rule_id}: fixture {fixture} pins Info rows not emitted by baseline: {missing}"]
    return []


def verify_fixture_state(
    rule_id: object,
    current: object,
    fixture: str,
    states: set[str],
) -> list[str]:
    if not isinstance(rule_id, str):
        return []
    failures: list[str] = []
    if current == "absent" and states:
        failures.append(f"{rule_id}: fixture {fixture} must not emit, got {sorted(states)}")
    if current == "info_only" and states != {"info"}:
        failures.append(f"{rule_id}: fixture {fixture} must emit only Info, got {sorted(states)}")
    if current == "error_or_warn" and "error_or_warn" not in states:
        failures.append(f"{rule_id}: fixture {fixture} must emit Error/Warn")
    return failures


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
from __future__ import annotations

import sys
from collections import Counter
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
PLAN_MANIFEST = REPO_ROOT / ".plans" / "2026-05-15-133355-g3rs-unclassified-fixture-coverage.md.manifest.toml"
LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
G3RS_MANIFEST = REPO_ROOT / ".plans" / "2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml"
REPO_MANIFEST = REPO_ROOT / ".plans" / "2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml"


def main() -> int:
    failures = verify()
    if failures:
        print("unclassified-fixture-coverage: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1
    print("unclassified-fixture-coverage: PASS")
    return 0


def verify() -> list[str]:
    failures: list[str] = []
    plan = load_toml(PLAN_MANIFEST)
    ledger = load_toml(LEDGER_PATH)
    g3rs_fixture_ids = fixture_ids(G3RS_MANIFEST)
    repo_fixture_ids = fixture_ids(REPO_MANIFEST)

    rows = ledger.get("test", [])
    if not isinstance(rows, list):
        return ["ledger must define [[test]] rows"]
    status_counts = Counter(row.get("status") for row in rows if isinstance(row, dict))
    if status_counts.get("unclassified", 0) != 0:
        failures.append(f"ledger still has unclassified rows: {status_counts['unclassified']}")

    contracts = plan.get("fixture_contract", [])
    if not isinstance(contracts, list):
        return ["plan manifest must define [[fixture_contract]] rows"]

    action_counts = Counter(contract.get("action") for contract in contracts if isinstance(contract, dict))
    coverage_plan = plan.get("coverage_plan", {})
    expected_g3rs = coverage_plan.get("new_g3rs_fixtures")
    expected_repo = coverage_plan.get("new_validate_repo_fixtures")
    if action_counts.get("new_g3rs_fixture", 0) != expected_g3rs:
        failures.append(
            "new_g3rs_fixture count mismatch: "
            f"manifest has {action_counts.get('new_g3rs_fixture', 0)}, expected {expected_g3rs}"
        )
    if action_counts.get("new_validate_repo_fixture", 0) != expected_repo:
        failures.append(
            "new_validate_repo_fixture count mismatch: "
            f"manifest has {action_counts.get('new_validate_repo_fixture', 0)}, expected {expected_repo}"
        )

    covered_rows = 0
    for contract in contracts:
        if not isinstance(contract, dict):
            failures.append("fixture contract row must be a table")
            continue
        fixture = contract.get("fixture")
        action = contract.get("action")
        rows_covered = contract.get("rows_covered")
        if not isinstance(fixture, str) or not fixture:
            failures.append("fixture contract has missing fixture id")
            continue
        if not isinstance(rows_covered, int) or rows_covered < 1:
            failures.append(f"{fixture}: rows_covered must be a positive integer")
        else:
            covered_rows += rows_covered

        if action in {"new_g3rs_fixture", "extend_existing", "reuse_existing"} and fixture not in g3rs_fixture_ids:
            failures.append(f"{fixture}: missing from g3rs fixture manifest")
        if action == "new_validate_repo_fixture" and fixture not in repo_fixture_ids:
            failures.append(f"{fixture}: missing from validate-repo fixture manifest")

    if covered_rows != 46:
        failures.append(f"fixture contracts cover {covered_rows} rows, expected 46")

    strict_required = coverage_plan.get("strict_required")
    if strict_required is not True:
        failures.append("coverage_plan.strict_required must be true")
    return failures


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def fixture_ids(path: Path) -> set[str]:
    manifest = load_toml(path)
    ids: set[str] = set()
    for row in manifest.get("fixture", []):
        if isinstance(row, dict) and isinstance(row.get("id"), str):
            ids.add(row["id"])
    return ids


if __name__ == "__main__":
    sys.exit(main())

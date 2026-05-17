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
APPROVED_OUTPUT_PATH = REPO_ROOT / "behavior" / "golden" / "g3ts-rule-fixtures" / "approved.normalized.json"
RULE_PATTERN = re.compile(r"g3ts-[a-z0-9-]+(?:-[a-z0-9-]+)*/[a-z0-9-]+(?:-[a-z0-9-]+)*")
FINDING_PATTERN = re.compile(r"^\[(Error|Warn|Info)\] (g3ts-[^ ]+)", re.MULTILINE)


def main() -> int:
    failures: list[str] = []
    manifest = load_toml(MANIFEST_PATH)
    rule_ids = discover_rule_ids()
    states = approved_rule_states()
    inventory_only = listed_rule_ids(manifest, "inventory_only_rule")
    cli_unreachable = listed_rule_ids(manifest, "cli_unreachable_rule")
    completed = completed_families(manifest)

    for rule_id in sorted((inventory_only | cli_unreachable) - rule_ids):
        failures.append(f"manifest-listed rule does not exist in source: {rule_id}")

    if completed:
        for family in sorted(completed):
            family_rules = {rule_id for rule_id in rule_ids if rule_id.startswith(f"g3ts-{family}/")}
            for rule_id in sorted(family_rules):
                observed = states.get(rule_id, set())
                if "error_or_warn" in observed:
                    continue
                if rule_id in inventory_only or rule_id in cli_unreachable:
                    continue
                failures.append(f"{rule_id}: completed family has no fixture coverage classification")

    if failures:
        print("g3ts-rule-coverage: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    covered = sum(1 for states_for_rule in states.values() if "error_or_warn" in states_for_rule)
    print(
        "g3ts-rule-coverage: PASS "
        f"source:{len(rule_ids)} covered_error_or_warn:{covered} "
        f"inventory_only:{len(inventory_only)} cli_unreachable:{len(cli_unreachable)}"
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


def approved_rule_states() -> dict[str, set[str]]:
    if not APPROVED_OUTPUT_PATH.exists():
        return {}
    data = json.loads(APPROVED_OUTPUT_PATH.read_text(encoding="utf-8"))
    states: dict[str, set[str]] = defaultdict(set)
    for record in data.get("records", []):
        stdout = record.get("stdout", "")
        if not isinstance(stdout, str):
            continue
        for severity, rule_id in FINDING_PATTERN.findall(stdout):
            states[rule_id].add("error_or_warn" if severity in {"Error", "Warn"} else "info")
    return states


def listed_rule_ids(manifest: dict[str, Any], key: str) -> set[str]:
    return {
        row["id"]
        for row in manifest.get(key, [])
        if isinstance(row, dict) and isinstance(row.get("id"), str)
    }


def completed_families(manifest: dict[str, Any]) -> set[str]:
    return {
        row["name"]
        for row in manifest.get("family", [])
        if isinstance(row, dict) and row.get("status") == "completed"
    }


if __name__ == "__main__":
    sys.exit(main())

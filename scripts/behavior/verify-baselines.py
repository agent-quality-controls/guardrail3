#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from datetime import datetime

from baseline_common import (
    REPO_ROOT,
    baseline_path,
    load_fixture_metadata,
    load_manifest,
    output_record,
    read_json,
)


def main() -> int:
    manifest = load_manifest()
    fixture_set = manifest["fixture_set"]
    fixture_root = REPO_ROOT / fixture_set["root"]
    baseline_root = REPO_ROOT / fixture_set["baseline_root"]
    tool = fixture_set["tool"]
    failures: list[str] = []
    checked = 0

    for entry in manifest["fixture"]:
        if not entry.get("baseline_required"):
            continue
        fixture_id = entry["id"]
        fixture_dir = fixture_root / fixture_id
        metadata = load_fixture_metadata(fixture_dir)
        for index, argv in enumerate(metadata["commands"]):
            path = baseline_path(baseline_root, fixture_id, index)
            if not path.is_file():
                failures.append(f"{fixture_id}: missing baseline {path.relative_to(REPO_ROOT)}")
                continue
            expected = read_json(path)
            actual = output_record(tool, fixture_id, fixture_dir, metadata, index, argv)
            actual["baseline_commit"] = expected.get("baseline_commit")
            actual["created_at"] = expected.get("created_at")
            failures.extend(verify_metadata(fixture_id, expected, actual))
            failures.extend(verify_exit_code(fixture_id, entry, expected))
            if actual != expected:
                failures.append(f"{fixture_id}: baseline drift in {path.relative_to(REPO_ROOT)}")
            checked += 1

    if failures:
        print("behavior-baselines: FAIL")
        for failure in failures:
            print(f"  {failure}")
        return 1

    print(f"behavior-baselines: PASS records:{checked}")
    return 0


def verify_metadata(fixture_id: str, expected: dict, actual: dict) -> list[str]:
    failures: list[str] = []
    required = [
        "tool",
        "baseline_commit",
        "fixture_hash",
        "runner_version",
        "normalizer_version",
        "output_schema_version",
        "created_at",
    ]
    for key in required:
        if key not in expected:
            failures.append(f"{fixture_id}: baseline metadata missing {key}")
    for key in ("tool", "fixture_hash", "runner_version", "normalizer_version", "output_schema_version"):
        if expected.get(key) != actual.get(key):
            failures.append(f"{fixture_id}: baseline metadata mismatch for {key}")
    if not isinstance(expected.get("baseline_commit"), str) or not re.fullmatch(
        r"[0-9a-f]{40}", expected.get("baseline_commit", "")
    ):
        failures.append(f"{fixture_id}: baseline_commit must be a git commit SHA")
    if not isinstance(expected.get("created_at"), str):
        failures.append(f"{fixture_id}: created_at must be an ISO timestamp")
    else:
        try:
            datetime.fromisoformat(expected["created_at"])
        except ValueError:
            failures.append(f"{fixture_id}: created_at must be an ISO timestamp")
    return failures


def verify_exit_code(fixture_id: str, entry: dict, expected: dict) -> list[str]:
    failures: list[str] = []
    exit_code = expected.get("exit_code")
    expected_exit = entry["expected_exit"]
    if expected_exit not in {"zero", "nonzero"}:
        failures.append(f"{fixture_id}: unknown expected_exit {expected_exit}")
    elif expected_exit == "zero" and exit_code != 0:
        failures.append(f"{fixture_id}: expected zero exit, got {exit_code}")
    elif expected_exit == "nonzero" and exit_code == 0:
        failures.append(f"{fixture_id}: expected nonzero exit, got 0")
    return failures


if __name__ == "__main__":
    sys.exit(main())

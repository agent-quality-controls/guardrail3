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
    manifest_path_from_argv,
    output_record,
    read_json,
)


def main() -> int:
    manifest = load_manifest(manifest_path_from_argv(sys.argv[1:]))
    fixture_set = manifest["fixture_set"]
    fixture_root = REPO_ROOT / fixture_set["root"]
    baseline_root = REPO_ROOT / fixture_set["baseline_root"]
    tool = fixture_set["tool"]
    is_validate_repo = fixture_set["root"] == "behavior/fixtures/g3rs-validate-repo"
    failures: list[str] = []
    checked = 0
    expected_paths = set()

    for entry in manifest["fixture"]:
        if not entry.get("baseline_required"):
            continue
        fixture_id = entry["id"]
        fixture_dir = fixture_root / fixture_id
        metadata = load_fixture_metadata(fixture_dir)
        for index, argv in enumerate(metadata["commands"]):
            path = baseline_path(baseline_root, fixture_id, index)
            expected_paths.add(path)
            if not path.is_file():
                failures.append(f"{fixture_id}: missing baseline {path.relative_to(REPO_ROOT)}")
                continue
            expected = read_json(path)
            actual = output_record(tool, fixture_id, fixture_dir, metadata, index, argv)
            actual["baseline_commit"] = expected.get("baseline_commit")
            actual["created_at"] = expected.get("created_at")
            failures.extend(verify_metadata(fixture_id, expected, actual))
            failures.extend(verify_exit_code(fixture_id, entry, expected))
            failures.extend(verify_no_outer_repo_leak(fixture_id, expected))
            if actual != expected:
                failures.append(f"{fixture_id}: baseline drift in {path.relative_to(REPO_ROOT)}")
            checked += 1

    if is_validate_repo:
        failures.extend(verify_validate_repo_semantics(baseline_root))

    for path in sorted(baseline_root.rglob("*.json")):
        if path not in expected_paths:
            failures.append(f"unexpected baseline {path.relative_to(REPO_ROOT)}")

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


def verify_no_outer_repo_leak(fixture_id: str, expected: dict) -> list[str]:
    failures: list[str] = []
    root = REPO_ROOT.as_posix()
    for key in ("stdout", "stderr"):
        value = expected.get(key)
        if isinstance(value, str) and root in value:
            failures.append(f"{fixture_id}: baseline {key} leaked outer repo path")
    return failures


def verify_validate_repo_semantics(baseline_root) -> list[str]:
    failures: list[str] = []
    r20 = read_json(baseline_root / "R20-crawlable-repo-marker-pair-policy" / "command-00.json")
    r20_stdout = r20.get("stdout", "")
    marker = "g3rs-topology/marker-pair-incomplete"
    if r20_stdout.count(marker) != 2:
        failures.append("R20-crawlable-repo-marker-pair-policy: expected exactly two marker-pair findings")
    expected_lines = {
        "[Error] g3rs-topology/marker-pair-incomplete packages/incomplete/guardrail3-rs.toml incomplete adoption marker pair",
        "[Error] g3rs-topology/marker-pair-incomplete packages/cargo-only/Cargo.toml incomplete adoption marker pair",
    }
    actual_lines = {
        line for line in r20_stdout.splitlines() if "g3rs-topology/marker-pair-incomplete" in line
    }
    if actual_lines != expected_lines:
        failures.append(
            "R20-crawlable-repo-marker-pair-policy: marker-pair finding lines do not match expected paths"
        )
    if "[Error] g3rs-topology/marker-pair-incomplete ./guardrail3-rs.toml" in r20_stdout:
        failures.append("R20-crawlable-repo-marker-pair-policy: root marker pair was reported")
    if "[Error] g3rs-topology/marker-pair-incomplete ./Cargo.toml" in r20_stdout:
        failures.append("R20-crawlable-repo-marker-pair-policy: root cargo marker pair was reported")
    if "g3rs-topology/no-nested-workspaces" in r20_stdout:
        failures.append("R20-crawlable-repo-marker-pair-policy: unrelated topology branch leaked into marker-pair fixture")
    for forbidden in (
        "marker-pair-incomplete packages/complete",
        "marker-pair-incomplete packages/absent",
        "marker-pair-incomplete behavior/fixtures",
    ):
        if forbidden in r20_stdout:
            failures.append(f"R20-crawlable-repo-marker-pair-policy: unexpected finding {forbidden}")

    r15 = read_json(baseline_root / "R15-hooks-reachable-no-root-cargo" / "command-00.json")
    r15_stdout = r15.get("stdout", "")
    if "== hooks ==" not in r15_stdout:
        failures.append("R15-hooks-reachable-no-root-cargo: hooks branch was not visible")
    if "g3rs-topology/" in r15_stdout:
        failures.append("R15-hooks-reachable-no-root-cargo: topology branch should be skipped without root Cargo.toml")

    r30 = read_json(baseline_root / "R30-root-adoption-pair-complete" / "command-00.json")
    r30_stdout = r30.get("stdout", "")
    if marker in r30_stdout:
        failures.append("R30-root-adoption-pair-complete: complete root pair emitted marker-pair finding")
    if "g3rs-topology/no-nested-workspaces" not in r30_stdout:
        failures.append("R30-root-adoption-pair-complete: topology branch was not visible")

    r40 = read_json(baseline_root / "R40-default-repo-root" / "command-00.json")
    if r40.get("cwd") != "repo":
        failures.append("R40-default-repo-root: baseline did not run from fixture repo")
    return failures


if __name__ == "__main__":
    sys.exit(main())

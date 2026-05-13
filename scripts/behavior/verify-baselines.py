#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from collections import Counter
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
            failures.extend(verify_no_volatile_output(fixture_id, expected))
            failures.extend(verify_required_results(fixture_id, entry, expected))
            failures.extend(verify_no_unlisted_findings(fixture_id, entry, expected))
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


def verify_no_volatile_output(fixture_id: str, expected: dict) -> list[str]:
    failures: list[str] = []
    patterns = [
        (r"target\(s\) in [0-9.]+s", "cargo timing"),
        (r"finished in [0-9.]+s", "rust test timing"),
        (r"\.cargo-target/debug/deps/[A-Za-z0-9_]+-[0-9a-f]{16}", "test binary hash"),
        (r"/private\$REPO", "partially normalized macOS temp path"),
    ]
    for key in ("stdout", "stderr"):
        value = expected.get(key)
        if not isinstance(value, str):
            continue
        for pattern, label in patterns:
            if re.search(pattern, value):
                failures.append(f"{fixture_id}: baseline {key} contains volatile {label}")
    return failures


def verify_required_results(fixture_id: str, entry: dict, expected: dict) -> list[str]:
    failures: list[str] = []
    stdout = expected.get("stdout", "")
    if not isinstance(stdout, str):
        return [f"{fixture_id}: baseline stdout must be a string"]
    findings = parse_finding_lines(stdout, include_info=True)
    required_counter = Counter(entry.get("required_results", []))
    for required, expected_count in required_counter.items():
        parts = required.split("|")
        if len(parts) == 4:
            severity, rule_id, title, file_path = parts
        else:
            failures.append(f"{fixture_id}: required_results row must be severity|rule|title|path: {required!r}")
            continue
        actual_count = findings.count((severity, rule_id, title, file_path))
        if actual_count < expected_count:
            failures.append(f"{fixture_id}: missing required result {required}")
    return failures


def verify_no_unlisted_findings(fixture_id: str, entry: dict, expected: dict) -> list[str]:
    if not fixture_requires_closed_findings(fixture_id):
        return []
    stdout = expected.get("stdout", "")
    if not isinstance(stdout, str):
        return [f"{fixture_id}: baseline stdout must be a string"]
    required_rows = entry.get("required_results", [])
    failures: list[str] = []
    required_counter = Counter()
    for required in required_rows:
        parts = required.split("|")
        if len(parts) == 4:
            required_counter[tuple(parts)] += 1
    actual_counter = Counter()
    for severity, rule_id, title, file_path in parse_finding_lines(stdout, include_info=False):
        matched = False
        for required in required_counter:
            required_severity, required_rule_id, required_title, required_file_path = required
            if (severity, rule_id, title, file_path) == (
                required_severity,
                required_rule_id,
                required_title,
                required_file_path,
            ):
                actual_counter[required] += 1
                matched = True
                break
        if not matched:
            failures.append(f"{fixture_id}: unlisted finding {severity}|{rule_id}|{title}|{file_path}")
    for required, actual_count in actual_counter.items():
        expected_count = required_counter[required]
        if actual_count > expected_count:
            severity, rule_id, title, file_path = required
            failures.append(
                f"{fixture_id}: finding listed {expected_count} times but emitted {actual_count} times: "
                f"{severity}|{rule_id}|{title}|{file_path}"
            )
    return failures


def parse_finding_lines(stdout: str, *, include_info: bool) -> list[tuple[str, str, str, str]]:
    findings: list[tuple[str, str, str, str]] = []
    prefixes = ("[Error]", "[Warn]", "[Info]") if include_info else ("[Error]", "[Warn]")
    for line in stdout.splitlines():
        if not line.startswith(prefixes):
            continue
        parts = line.split(" ", 3)
        if len(parts) != 4:
            continue
        _severity, rule_id, file_path, title = parts
        findings.append((_severity.strip("[]"), rule_id, title, file_path))
    return findings


def fixture_requires_closed_findings(fixture_id: str) -> bool:
    if fixture_id.startswith(("L00-", "L10-", "L20-")):
        return True
    if fixture_id.startswith("L3"):
        return True
    if fixture_id.startswith(("L40-", "L50-", "L60-", "L70-", "L80-")):
        return True
    if fixture_id.startswith(("R00-", "R10-", "R15-", "R16-", "R17-", "R20-", "R30-")):
        return True
    return False


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
    r20_fixture_marker = (
        baseline_root.parents[1]
        / "fixtures"
        / "g3rs-validate-repo"
        / "R20-crawlable-repo-marker-pair-policy"
        / "repo"
        / "behavior"
        / "fixtures"
        / "g3rs"
        / "demo"
        / "repo"
        / "guardrail3-rs.toml"
    )
    if not r20_fixture_marker.is_file():
        failures.append("R20-crawlable-repo-marker-pair-policy: missing behavior fixture marker input")
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
    if "== topology ==" in r15_stdout:
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

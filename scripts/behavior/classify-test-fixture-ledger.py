#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[2]
LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
GOLDEN_OUTPUTS = (
    REPO_ROOT / "behavior" / "golden" / "g3rs-validate" / "approved.normalized.json",
    REPO_ROOT / "behavior" / "golden" / "g3rs-validate-repo" / "approved.normalized.json",
)
CLEAN_FIXTURE = "L80-project-policy-valid-clean"
HOOK_FIXTURE = "R15-hooks-reachable-no-root-cargo"
RULE_ID_RE = re.compile(r'g3rs-[a-z]+/[a-z0-9-]+')


HIT_KEYWORDS = (
    "accident",
    "ban",
    "block",
    "conflict",
    "detect",
    "duplicate",
    "emit",
    "error",
    "extra",
    "fail",
    "fire",
    "flags",
    "forbid",
    "hidden",
    "hit",
    "invalid",
    "info_when",
    "inventory",
    "inventories",
    "missing",
    "reject",
    "report",
    "require",
    "shadow",
    "unreadable",
    "warn",
    "weak",
    "wrong",
)
NON_HIT_KEYWORDS = (
    "accept",
    "allow",
    "clean",
    "correct_baseline",
    "does_not",
    "do_not",
    "ignore",
    "ignores",
    "is_not_reported",
    "matching_baseline",
    "no_findings",
    "not_report",
    "passes_when",
    "satisfies_contract",
    "quiet",
    "skip",
    "stays_allowed",
    "stays_quiet",
    "without_findings",
)
COMPILE_CONTRACT_MARKERS = (
    "/crates/io/",
    "/crates/logic/",
    "-ingestion/",
    "-types/",
    "-assertions/",
)
REPLAY_SYSTEM_MARKERS = (
    "scripts/behavior/",
    "behavior/",
)


@dataclass(frozen=True)
class Finding:
    fixture: str
    severity: str
    rule: str
    title: str
    file: str


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    rows = classify_rows()
    output = render_ledger(rows)
    if args.check:
        current = LEDGER_PATH.read_text(encoding="utf-8")
        if current != output:
            print("test fixture ledger is not classified from current sources")
            return 1
        print("test fixture ledger classification is current")
        return 0
    LEDGER_PATH.write_text(output, encoding="utf-8")
    counts = Counter(row["status"] for row in rows)
    print(
        "classified test fixture ledger "
        + " ".join(f"{status}:{count}" for status, count in sorted(counts.items()))
    )
    return 0


def classify_rows() -> list[dict[str, Any]]:
    tests = load_active_tests()
    rule_ids = load_rule_ids()
    findings = load_findings()
    findings_by_rule: dict[str, list[Finding]] = defaultdict(list)
    findings_by_fixture_rule: dict[tuple[str, str], list[Finding]] = defaultdict(list)
    for finding in findings:
        findings_by_rule[finding.rule].append(finding)
        findings_by_fixture_rule[(finding.fixture, finding.rule)].append(finding)

    rows: list[dict[str, Any]] = []
    for test in tests:
        row = {
            "test_path": test["test_path"],
            "test_name": test["test_name"],
            "line": test["line"],
        }
        row.update(classify_test(test, rule_ids, findings_by_rule, findings_by_fixture_rule))
        rows.append(row)
    return rows


def load_active_tests() -> list[dict[str, Any]]:
    output = subprocess.check_output(
        [
            "python3",
            str(REPO_ROOT / "scripts" / "behavior" / "list-rust-tests.py"),
            "--format",
            "json",
        ],
        cwd=REPO_ROOT,
        text=True,
    )
    return json.loads(output)["tests"]


def load_rule_ids() -> dict[str, str]:
    rule_ids: dict[str, str] = {}
    for rule_file in sorted((REPO_ROOT / "packages" / "rs").glob("**/rule.rs")):
        if "/target/" in rule_file.as_posix():
            continue
        text = rule_file.read_text(encoding="utf-8", errors="ignore")
        match = RULE_ID_RE.search(text)
        if match:
            rule_ids[rule_file.parent.resolve().as_posix()] = match.group(0)
    return rule_ids


def load_findings() -> list[Finding]:
    findings: list[Finding] = []
    for golden in GOLDEN_OUTPUTS:
        data = json.loads(golden.read_text(encoding="utf-8"))
        for record in data["records"]:
            fixture_id = record["fixture_id"]
            for line in record.get("stdout", "").splitlines():
                parsed = parse_finding_line(fixture_id, line)
                if parsed is not None:
                    findings.append(parsed)
    return findings


def parse_finding_line(fixture: str, line: str) -> Finding | None:
    if not line.startswith("["):
        return None
    parts = line.split(" ", 3)
    if len(parts) != 4:
        return None
    severity = parts[0].strip("[]")
    rule = parts[1]
    file_path = parts[2]
    title = parts[3]
    if severity not in {"Error", "Warn", "Info"} or not rule.startswith("g3rs-"):
        return None
    return Finding(fixture=fixture, severity=severity, rule=rule, title=title, file=file_path)


def classify_test(
    test: dict[str, Any],
    rule_ids: dict[str, str],
    findings_by_rule: dict[str, list[Finding]],
    findings_by_fixture_rule: dict[tuple[str, str], list[Finding]],
) -> dict[str, Any]:
    test_path = str(test["test_path"])
    test_name = str(test["test_name"])
    rule = rule_for_test_path(test_path, rule_ids)
    name_kind = classify_name(test_name)

    if rule is not None:
        if name_kind == "non_hit":
            forbidden = first_non_info_finding(rule, findings_by_rule)
            if forbidden is not None and not has_exact_finding(CLEAN_FIXTURE, forbidden, findings_by_fixture_rule):
                return {
                    "status": "covered_non_hit",
                    "fixture": CLEAN_FIXTURE,
                    "severity": forbidden.severity,
                    "rule": rule,
                    "title": forbidden.title,
                    "file": forbidden.file,
                }
        if name_kind == "hit":
            finding = preferred_finding(rule, findings_by_rule)
            if finding is not None:
                return finding_row("covered_hit", finding)
        if rule.endswith("/hook-contract"):
            finding = preferred_fixture_finding(HOOK_FIXTURE, rule, findings_by_fixture_rule)
            if finding is not None:
                return finding_row("covered_hit", finding)
        return {
            "status": "unclassified",
            "reason": "current fixtures do not prove this rule test's hit or non-hit behavior",
        }

    if "/rule_tests/" in test_path:
        return {
            "status": "kept_compile_contract",
            "reason": "test exercises a rule module runner or helper path that has no independent rule id in fixture output",
        }
    if any(marker in test_path for marker in REPLAY_SYSTEM_MARKERS):
        return {
            "status": "kept_replay_system",
            "reason": "test validates the replay/ledger infrastructure instead of a guardrail finding",
        }
    if is_compile_contract_path(test_path):
        return {
            "status": "kept_compile_contract",
            "reason": "test validates parser, ingestion, CLI, renderer, or orchestration behavior not represented as one fixture finding",
        }
    return {
        "status": "unclassified",
        "reason": "classifier could not map this test to fixture-backed behavior",
    }


def rule_for_test_path(test_path: str, rule_ids: dict[str, str]) -> str | None:
    absolute = (REPO_ROOT / test_path).as_posix()
    candidates: list[str] = []

    if "/rule_tests/" in absolute:
        candidates.append(absolute.split("/rule_tests/", 1)[0])

    path = Path(absolute)
    for parent in path.parents:
        name = parent.name
        if name.endswith("_tests"):
            candidates.append((parent.parent / name.removesuffix("_tests")).as_posix())

    for candidate in candidates:
        rule = rule_ids.get(candidate)
        if rule is not None:
            return rule
    return None


def classify_name(test_name: str) -> str | None:
    lowered = test_name.lower()
    if any(keyword in lowered for keyword in NON_HIT_KEYWORDS):
        return "non_hit"
    if any(keyword in lowered for keyword in HIT_KEYWORDS):
        return "hit"
    return None


def first_non_info_finding(rule: str, findings_by_rule: dict[str, list[Finding]]) -> Finding | None:
    for severity in ("Error", "Warn"):
        for finding in findings_by_rule.get(rule, []):
            if finding.severity == severity:
                return finding
    return None


def preferred_finding(rule: str, findings_by_rule: dict[str, list[Finding]]) -> Finding | None:
    for severity in ("Error", "Warn", "Info"):
        for finding in findings_by_rule.get(rule, []):
            if finding.severity == severity:
                return finding
    return None


def preferred_fixture_finding(
    fixture: str,
    rule: str,
    findings_by_fixture_rule: dict[tuple[str, str], list[Finding]],
) -> Finding | None:
    for severity in ("Error", "Warn", "Info"):
        for finding in findings_by_fixture_rule.get((fixture, rule), []):
            if finding.severity == severity:
                return finding
    return None


def has_exact_finding(
    fixture: str,
    finding: Finding,
    findings_by_fixture_rule: dict[tuple[str, str], list[Finding]],
) -> bool:
    return any(
        candidate.severity == finding.severity
        and candidate.title == finding.title
        and candidate.file == finding.file
        for candidate in findings_by_fixture_rule.get((fixture, finding.rule), [])
    )


def finding_row(status: str, finding: Finding) -> dict[str, Any]:
    return {
        "status": status,
        "fixture": finding.fixture,
        "severity": finding.severity,
        "rule": finding.rule,
        "title": finding.title,
        "file": finding.file,
    }


def is_compile_contract_path(test_path: str) -> bool:
    normalized = f"/{test_path}"
    if any(marker in normalized for marker in COMPILE_CONTRACT_MARKERS):
        return True
    return "_tests/" in normalized and "/rule_tests/" not in normalized


def render_ledger(rows: list[dict[str, Any]]) -> str:
    lines = [
        "# Generated by scripts/behavior/classify-test-fixture-ledger.py",
        "# Rows with status = \"unclassified\" still require manual or stricter fixture evidence.",
        "",
    ]
    for row in rows:
        lines.append("[[test]]")
        for key in ("test_path", "test_name", "line", "status", "fixture", "severity", "rule", "title", "file", "reason"):
            if key in row:
                lines.append(f"{key} = {toml_value(row[key])}")
        lines.append("")
    return "\n".join(lines)


def toml_value(value: object) -> str:
    if isinstance(value, int):
        return str(value)
    if isinstance(value, str):
        return json.dumps(value)
    raise TypeError(f"unsupported TOML value: {value!r}")


if __name__ == "__main__":
    sys.exit(main())

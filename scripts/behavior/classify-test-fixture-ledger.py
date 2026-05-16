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

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
DISPOSITION_LEDGER_PATH = REPO_ROOT / "behavior" / "migration" / "g3rs-kept-test-disposition.toml"
GOLDEN_OUTPUTS = (
    REPO_ROOT / "behavior" / "golden" / "g3rs-validate" / "approved.normalized.json",
    REPO_ROOT / "behavior" / "golden" / "g3rs-validate-repo" / "approved.normalized.json",
)
CLEAN_FIXTURE = "L80-project-policy-valid-clean"
HOOK_FIXTURE = "R15-hooks-reachable-no-root-cargo"
RULE_ID_RE = re.compile(r'g3rs-[a-z]+/[a-z0-9-]+')
FIXTURE_COVERED_STATUSES = {"covered_hit", "covered_non_hit"}
DISPOSITION_COVERED = {"covered_by_cli_output", "covered_by_renderer_output"}


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
EXPLICIT_ROWS: dict[tuple[str, str], tuple[str, str, str | None, str, str | None, str | None]] = {
    (
        "packages/rs/code/g3rs-code-source-checks/crates/runtime/src/many_use_imports/rule_tests/direct.rs",
        "public_reexports_count_in_non_facade_source",
    ): (
        "covered_hit",
        "L70-delegated-policy-valid-project-policy-violated",
        "Warn",
        "g3rs-code/many-use-imports",
        "many use imports",
        "src/many_use_imports.rs",
    ),
    (
        "packages/rs/code/g3rs-code-source-checks/crates/runtime/src/too_many_use_imports/rule_tests/direct.rs",
        "public_reexports_count_in_non_facade_source",
    ): (
        "covered_hit",
        "L70-delegated-policy-valid-project-policy-violated",
        "Error",
        "g3rs-code/too-many-use-imports",
        "too many use imports",
        "src/too_many_use_imports.rs",
    ),
    (
        "packages/rs/code/g3rs-code-source-checks/crates/runtime/src/unused_crate_dependencies_allow/rule_tests/direct.rs",
        "inventories_crate_level_unused_crate_dependencies_allow",
    ): (
        "covered_hit",
        "L70-delegated-policy-valid-project-policy-violated",
        "Info",
        "g3rs-code/unused-crate-dependencies-allow",
        "unused_crate_dependencies exemption",
        "src/code_policy.rs",
    ),
    (
        "packages/rs/code/g3rs-code-source-checks/crates/runtime/src/unused_crate_dependencies_allow/rule_tests/direct.rs",
        "inventories_inline_module_unused_crate_dependencies_allow",
    ): (
        "covered_hit",
        "L70-delegated-policy-valid-project-policy-violated",
        "Info",
        "g3rs-code/unused-crate-dependencies-allow",
        "unused_crate_dependencies exemption",
        "src/code_policy.rs",
    ),
    (
        "packages/rs/code/g3rs-code-source-checks/crates/runtime/src/unused_crate_dependencies_allow/rule_tests/false_positives.rs",
        "ignores_other_crate_level_allows",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-code/unused-crate-dependencies-allow", None, None),
    (
        "packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/build_dependencies_allowlisted/rule_tests/golden.rs",
        "workspace_true_external_build_dependency_is_checked",
    ): (
        "covered_hit",
        "L70-workspace-package-policy-violated",
        "Error",
        "g3rs-deps/build-dependencies-allowlisted",
        "unauthorized build dependency",
        "Cargo.toml",
    ),
    (
        "packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/edition_mismatch/rule_tests/package_edition.rs",
        "uses_package_edition_fallback_when_workspace_package_edition_is_absent",
    ): (
        "covered_hit",
        "L60-fmt-package-edition-fallback-policy-invalid",
        "Warn",
        "g3rs-fmt/edition-mismatch",
        "rustfmt edition differs from Cargo edition",
        "rustfmt.toml",
    ),
    (
        "packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/edition_mismatch/rule_tests/precedence.rs",
        "prefers_workspace_package_edition_over_package_edition",
    ): (
        "covered_hit",
        "L60-fmt-workspace-edition-precedence-policy-invalid",
        "Warn",
        "g3rs-fmt/edition-mismatch",
        "rustfmt edition differs from Cargo edition",
        "rustfmt.toml",
    ),
    (
        "packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/extra_settings/rule_tests/skip_macro_invocations.rs",
        "inventories_empty_skip_macro_invocations",
    ): (
        "covered_hit",
        "L60-fmt-package-edition-fallback-policy-invalid",
        "Info",
        "g3rs-fmt/rustfmt-extra-settings-inventory",
        "rustfmt extra setting: skip_macro_invocations",
        "rustfmt.toml",
    ),
    (
        "packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/extra_settings/rule_tests/skip_macro_invocations.rs",
        "inventories_nonempty_skip_macro_invocations",
    ): (
        "covered_hit",
        "L60-fmt-workspace-edition-precedence-policy-invalid",
        "Info",
        "g3rs-fmt/rustfmt-extra-settings-inventory",
        "rustfmt extra setting: skip_macro_invocations",
        "rustfmt.toml",
    ),
    (
        "packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/contract_required_tools_installed/rule_tests/golden.rs",
        "path_qualified_machete_and_dupes_satisfy_contract",
    ): (
        "covered_hit",
        "R18-hooks-path-qualified-safe-comments",
        "Info",
        "g3rs-hooks/contract-required-tools-installed",
        "cargo-dupes installed for hook contract",
        ".githooks/pre-commit",
    ),
    (
        "packages/rs/hooks/g3rs-hooks-config-checks/crates/runtime/src/required_tools_installed/rule_tests/cases.rs",
        "treats_path_qualified_tools_as_installed",
    ): (
        "covered_hit",
        "R18-hooks-path-qualified-safe-comments",
        "Info",
        "g3rs-hooks/required-tools-installed",
        "cargo-deny installed",
        ".githooks/pre-commit",
    ),
    (
        "packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule_tests/golden.rs",
        "cargo_metadata_locked_satisfies_concrete_lockfile_contract",
    ): (
        "covered_hit",
        "R18-hooks-path-qualified-safe-comments",
        "Info",
        "g3rs-hooks/required-contract-command-present",
        "hook contract command is present",
        ".githooks/pre-commit",
    ),
    (
        "packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/required_contract_command_present/rule_tests/golden.rs",
        "cargo_update_locked_satisfies_concrete_lockfile_contract",
    ): (
        "covered_hit",
        "R18-hooks-path-qualified-safe-comments",
        "Info",
        "g3rs-hooks/required-contract-command-present",
        "hook contract command is present",
        ".githooks/pre-commit",
    ),
    (
        "packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/declared_workspace_members_only/rule_tests/cases.rs",
        "undeclared_issue_under_nested_workspace_mentions_parent_workspace",
    ): (
        "covered_hit",
        "L38-topology-non-root-nested-context",
        "Error",
        "g3rs-topology/declared-workspace-members-only",
        "Workspace child `crates/core` must be declared explicitly",
        "crates/core/Cargo.toml",
    ),
    (
        "packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/member_paths_must_not_escape_root/rule_tests/cases.rs",
        "escaping_member_under_nested_workspace_mentions_that_workspace",
    ): (
        "covered_hit",
        "L38-topology-non-root-nested-context",
        "Error",
        "g3rs-topology/member-paths-must-not-escape-root",
        "Workspace `.` uses escaping member path `../shared`",
        "Cargo.toml",
    ),
    (
        "packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/no_nested_guardrail3_rs_toml/rule_tests/cases.rs",
        "nested_guardrail3_rs_toml_under_non_root_outer_mentions_that_outer",
    ): (
        "covered_hit",
        "L38-topology-non-root-nested-context",
        "Error",
        "g3rs-topology/no-nested-guardrail3-rs-toml",
        "Nested adopted Rust unit `inner` is forbidden",
        "inner/guardrail3-rs.toml",
    ),
    (
        "packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/no_nested_workspaces/rule_tests/cases.rs",
        "nested_workspace_under_non_root_parent_mentions_that_parent",
    ): (
        "covered_hit",
        "L38-topology-non-root-nested-context",
        "Error",
        "g3rs-topology/no-nested-workspaces",
        "Nested workspace `inner` is forbidden",
        "inner/Cargo.toml",
    ),
    (
        "packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/workspace_local_file_placement/rule_tests/cases.rs",
        "fmt_message_is_preserved_verbatim",
    ): (
        "covered_hit",
        "L38-topology-non-root-nested-context",
        "Error",
        "g3rs-topology/workspace-local-file-placement",
        "`fmt` file `crates/core/rustfmt.toml` is illegally placed",
        "crates/core/rustfmt.toml",
    ),
}

DENY_ROW_FIXTURES: dict[tuple[str, str], tuple[str, str, str | None, str, str | None, str | None]] = {
    (
        "advisories/advisories_baseline/rule_tests/missing_section.rs",
        "no_advisories_section",
    ): ("covered_hit", "L60-deny-missing-sections-policy-invalid", "Error", "g3rs-deny/advisories-baseline", "[advisories] section missing", "deny.toml"),
    (
        "advisories/deprecated_advisories/rule_tests/golden.rs",
        "no_advisories_section",
    ): ("covered_non_hit", "L60-deny-missing-sections-policy-invalid", None, "g3rs-deny/deprecated-advisories", None, None),
    (
        "advisories/deprecated_advisories/rule_tests/golden.rs",
        "no_deprecated_fields",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/deprecated-advisories", None, None),
    (
        "advisories/deprecated_advisories/rule_tests/has_deprecated.rs",
        "all_deprecated_fields_present",
    ): ("covered_hit", "L60-deny-deprecated-advisories-policy-invalid", "Warn", "g3rs-deny/deprecated-advisories", "deprecated advisory field `vulnerability`", "deny.toml"),
    (
        "advisories/deprecated_advisories/rule_tests/has_deprecated.rs",
        "notice_present",
    ): ("covered_hit", "L60-deny-deprecated-advisories-policy-invalid", "Warn", "g3rs-deny/deprecated-advisories", "deprecated advisory field `notice`", "deny.toml"),
    (
        "advisories/deprecated_advisories/rule_tests/has_deprecated.rs",
        "unsound_present",
    ): ("covered_hit", "L60-deny-deprecated-advisories-policy-invalid", "Warn", "g3rs-deny/deprecated-advisories", "deprecated advisory field `unsound`", "deny.toml"),
    (
        "advisories/deprecated_advisories/rule_tests/has_deprecated.rs",
        "vulnerability_present",
    ): ("covered_hit", "L60-deny-deprecated-advisories-policy-invalid", "Warn", "g3rs-deny/deprecated-advisories", "deprecated advisory field `vulnerability`", "deny.toml"),
    (
        "advisories/graph_all_features/rule_tests/golden.rs",
        "all_features_true",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/graph-all-features", None, None),
    (
        "advisories/graph_all_features/rule_tests/missing_section.rs",
        "no_graph_section",
    ): ("covered_hit", "L60-deny-missing-sections-policy-invalid", "Error", "g3rs-deny/graph-all-features", "[graph] section missing", "deny.toml"),
    (
        "advisories/graph_all_features/rule_tests/wrong_value.rs",
        "all_features_false",
    ): ("covered_hit", "L60-deny-wrong-values-policy-invalid", "Error", "g3rs-deny/graph-all-features", "graph all-features must be true", "deny.toml"),
    (
        "advisories/graph_no_default_features/rule_tests/golden.rs",
        "no_default_features_false",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/graph-no-default-features", None, None),
    (
        "advisories/graph_no_default_features/rule_tests/missing_section.rs",
        "no_graph_section",
    ): ("covered_hit", "L60-deny-missing-sections-policy-invalid", "Error", "g3rs-deny/graph-no-default-features", "[graph] section missing", "deny.toml"),
    (
        "advisories/graph_no_default_features/rule_tests/wrong_value.rs",
        "no_default_features_true",
    ): ("covered_hit", "L60-deny-wrong-values-policy-invalid", "Error", "g3rs-deny/graph-no-default-features", "graph no-default-features must be false", "deny.toml"),
    (
        "advisories/stricter_advisories_inventory/rule_tests/golden.rs",
        "matching_baseline",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/stricter-advisories-inventory", None, None),
    (
        "advisories/stricter_advisories_inventory/rule_tests/golden.rs",
        "no_advisories_section",
    ): ("covered_non_hit", "L60-deny-missing-sections-policy-invalid", None, "g3rs-deny/stricter-advisories-inventory", None, None),
    (
        "advisories/stricter_advisories_inventory/rule_tests/stricter.rs",
        "unmaintained_all_is_stricter_than_workspace_baseline",
    ): ("covered_hit", "L60-deny-wrong-values-policy-invalid", "Info", "g3rs-deny/stricter-advisories-inventory", "advisories `unmaintained` stricter than baseline", "deny.toml"),
    (
        "advisories/stricter_advisories_inventory/rule_tests/stricter.rs",
        "unmaintained_transitive_is_not_stricter_than_workspace_baseline",
    ): ("covered_non_hit", "L60-deny-nonstricter-values-policy-invalid", None, "g3rs-deny/stricter-advisories-inventory", None, None),
    (
        "advisories/stricter_advisories_inventory/rule_tests/stricter.rs",
        "yanked_allow_is_not_stricter_than_warn_baseline",
    ): ("covered_non_hit", "L60-deny-nonstricter-values-policy-invalid", None, "g3rs-deny/stricter-advisories-inventory", None, None),
    (
        "advisories/stricter_advisories_inventory/rule_tests/stricter.rs",
        "yanked_deny",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/stricter-advisories-inventory", None, None),
    (
        "bans/extra_feature_bans_inventory/rule_tests/golden.rs",
        "no_findings_when_only_tokio_entry",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/extra-feature-bans-inventory", None, None),
    (
        "bans/extra_feature_bans_inventory/rule_tests/missing_section.rs",
        "no_findings_when_bans_section_missing",
    ): ("covered_non_hit", "L60-deny-missing-sections-policy-invalid", None, "g3rs-deny/extra-feature-bans-inventory", None, None),
    (
        "bans/highlight_inventory/rule_tests/golden.rs",
        "no_findings_when_highlight_matches_baseline",
    ): ("covered_non_hit", "L80-project-policy-valid-clean", None, "g3rs-deny/highlight-inventory", None, None),
    (
        "bans/highlight_inventory/rule_tests/missing_section.rs",
        "no_findings_when_bans_section_missing",
    ): ("covered_non_hit", "L60-deny-missing-sections-policy-invalid", None, "g3rs-deny/highlight-inventory", None, None),
}

SAFE_HOOK_COMMENT_TESTS = {
    "ignores_escaped_hash_when_comment_text_looks_like_bypass_instruction",
    "ignores_escaped_space_before_hash_when_comment_text_looks_like_bypass_instruction",
    "ignores_hash_inside_quotes",
    "passes_when_no_no_verify_comment_exists",
}


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
    active_keys = {(str(test["test_path"]), str(test["test_name"])) for test in tests}
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
    rows.extend(existing_deleted_replacement_rows(active_keys))
    return rows


def existing_deleted_replacement_rows(active_keys: set[tuple[str, str]]) -> list[dict[str, Any]]:
    if not LEDGER_PATH.is_file():
        return []
    existing_rows = load_toml(LEDGER_PATH).get("test", [])
    if not isinstance(existing_rows, list):
        return []
    disposition_by_key = load_dispositions()
    preserved: list[dict[str, Any]] = []
    for row in existing_rows:
        if not isinstance(row, dict):
            continue
        key = row_key(row)
        if key is None or key in active_keys:
            continue
        if row_can_stay_after_test_deletion(row, disposition_by_key.get(key)):
            preserved.append(row)
    return preserved


def load_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as file:
            return tomllib.load(file)
    except FileNotFoundError:
        return {}


def load_dispositions() -> dict[tuple[str, str], str]:
    rows = load_toml(DISPOSITION_LEDGER_PATH).get("test", [])
    output: dict[tuple[str, str], str] = {}
    if not isinstance(rows, list):
        return output
    for row in rows:
        if not isinstance(row, dict):
            continue
        key = row_key(row)
        disposition = row.get("disposition")
        if key is not None and isinstance(disposition, str):
            output[key] = disposition
    return output


def row_key(row: dict[str, Any]) -> tuple[str, str] | None:
    test_path = row.get("test_path")
    test_name = row.get("test_name")
    if not isinstance(test_path, str) or not isinstance(test_name, str):
        return None
    return (test_path, test_name)


def row_can_stay_after_test_deletion(row: dict[str, Any], disposition: str | None) -> bool:
    if row.get("status") in FIXTURE_COVERED_STATUSES:
        return True
    return row.get("status") == "kept_compile_contract" and disposition in DISPOSITION_COVERED


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
    explicit = explicit_classification(test_path, test_name)
    if explicit is not None:
        return explicit

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


def explicit_classification(test_path: str, test_name: str) -> dict[str, Any] | None:
    row = EXPLICIT_ROWS.get((test_path, test_name))
    if row is not None:
        return explicit_row(row)

    deny_prefix = "packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/"
    if test_path.startswith(deny_prefix):
        suffix = test_path.removeprefix(deny_prefix)
        deny_row = DENY_ROW_FIXTURES.get((suffix, test_name))
        if deny_row is not None:
            return explicit_row(deny_row)

    hook_comment_path = (
        "packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/"
        "shell_safety/no_bypass_instructions/rule_tests/golden.rs"
    )
    if test_path == hook_comment_path and test_name in SAFE_HOOK_COMMENT_TESTS:
        return explicit_row(
            (
                "covered_hit",
                "R18-hooks-path-qualified-safe-comments",
                "Info",
                "g3rs-hooks/no-bypass-instructions",
                "no hook bypass instructions",
                ".githooks/pre-commit",
            )
        )
    return None


def explicit_row(row: tuple[str, str, str | None, str, str | None, str | None]) -> dict[str, Any]:
    status, fixture, severity, rule, title, file_path = row
    output: dict[str, Any] = {
        "status": status,
        "fixture": fixture,
        "rule": rule,
    }
    if severity is not None:
        output["severity"] = severity
    if title is not None:
        output["title"] = title
    if file_path is not None:
        output["file"] = file_path
    return output


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

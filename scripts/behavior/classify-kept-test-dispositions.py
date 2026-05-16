#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from collections import Counter
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib  # type: ignore


REPO_ROOT = Path(__file__).resolve().parents[2]
SOURCE_LEDGER = REPO_ROOT / "behavior" / "migration" / "g3rs-test-fixture-ledger.toml"
OUTPUT_LEDGER = REPO_ROOT / "behavior" / "migration" / "g3rs-kept-test-disposition.toml"

CARGO_CLI_COVERED_FIXTURES = {
    "errors_when_approved_allow_reason_is_too_weak": "cargo-R10-policy-violations",
    "inventories_documented_approved_allow_entries": "cargo-R10-policy-violations",
    "inventories_hybrid_root_package_approved_allow_entries": "cargo-R10-policy-violations",
    "errors_when_member_edition_is_invalid": "cargo-R10-policy-violations",
    "errors_when_member_edition_is_unrecognized": "cargo-R10-policy-violations",
    "inventories_when_member_inherits_workspace_edition": "cargo-R00-clean-golden",
    "warns_when_member_edition_is_older_than_workspace": "cargo-R10-policy-violations",
    "errors_on_documented_member_local_allow_entries": "cargo-R10-policy-violations",
    "errors_on_member_local_allow_entries": "cargo-R10-policy-violations",
    "errors_when_member_local_allow_reason_is_too_weak": "cargo-R10-policy-violations",
    "inventories_when_member_has_no_local_allow_entries": "cargo-R00-clean-golden",
    "errors_when_member_lint_table_shape_is_invalid": "cargo-R10-policy-violations",
    "errors_when_member_weakens_forbid_to_deny": "cargo-R10-policy-violations",
    "errors_when_member_weakens_workspace_lints": "cargo-R10-policy-violations",
    "inventories_when_member_does_not_weaken_workspace_lints": "cargo-R00-clean-golden",
    "errors_when_library_profile_has_no_rust_version": "cargo-R21-root-metadata-missing",
    "inventories_when_library_profile_declares_rust_version": "cargo-R00-clean-golden",
    "inventories_when_workspace_root_library_declares_rust_version": "cargo-R00-clean-golden",
    "errors_on_documented_unapproved_allow_entries": "cargo-R10-policy-violations",
    "errors_on_hybrid_root_package_allow_entries": "cargo-R10-policy-violations",
    "errors_on_unapproved_allow_entries": "cargo-R10-policy-violations",
    "errors_when_unapproved_allow_reason_is_too_weak": "cargo-R10-policy-violations",
    "inventories_when_no_unapproved_allow_entries_exist": "cargo-R00-clean-golden",
    "errors_when_member_does_not_inherit_workspace_lints": "cargo-R10-policy-violations",
    "errors_when_workspace_lint_inheritance_shape_is_invalid": "cargo-R10-policy-violations",
    "inventories_when_member_inherits_workspace_lints": "cargo-R00-clean-golden",
}

CARGO_INTERNAL_ONLY_REASONS = {
    "stands_down_when_rust_policy_is_unreadable": "rust policy unreadable is an ingestion-state branch, not distinguishable through family-rule CLI output without replacing the user-facing config error",
    "stands_down_when_rust_policy_parse_error_blocks_waiver_resolution": "rust policy parse error is an ingestion-state branch, not a cargo rule finding through the CLI",
    "stays_quiet_when_clippy_table_shape_is_invalid": "wrong-type Cargo lint tables fail before this rule can produce a cargo rule finding through the CLI",
    "stays_quiet_when_workspace_edition_shape_is_invalid_even_if_package_has_valid_fallback": "wrong-type workspace edition fails Cargo TOML parsing before member edition drift can produce a CLI finding",
    "stays_quiet_when_workspace_has_no_edition_policy": "non-hit member edition drift behavior is internal unless the CLI exposes absent-findings assertions",
    "stands_down_when_rust_policy_parse_error_blocks_member_reason_resolution": "rust policy parse error is an ingestion-state branch, not a cargo rule finding through the CLI",
    "stays_quiet_when_member_override_shape_is_invalid": "member-local allow classification intentionally stands down behind no-weakened-overrides on malformed member lint tables",
    "stays_quiet_when_workspace_policy_is_incomplete": "workspace-policy-incomplete non-hit behavior is internal unless the CLI exposes absent-findings assertions",
    "errors_when_rust_version_shape_is_invalid": "wrong-type rust-version fails Cargo TOML parsing before rust-version-policy can produce a CLI finding",
    "errors_when_workspace_root_rust_version_shape_is_invalid": "wrong-type workspace rust-version fails Cargo TOML parsing before rust-version-policy can produce a CLI finding",
    "inventories_when_non_library_omits_rust_version": "service-profile non-hit behavior needs a second clean profile fixture, which is not part of the one-golden-per-family corpus",
    "stands_down_when_rust_policy_parse_error_blocks_reason_resolution": "rust policy parse error is an ingestion-state branch, not a cargo rule finding through the CLI",
    "stays_quiet_when_rust_policy_parse_error_suppresses_clean_inventory": "rust policy parse error is an ingestion-state branch, not a cargo rule finding through the CLI",
}

CLIPPY_CLI_COVERED_FIXTURES = {
    "inventories_parseable_policy_context": "clippy-R00-clean-golden",
}

CLIPPY_INTERNAL_ONLY_REASONS = {
    "reports_policy_context_parse_errors": "the Error branch requires invalid guardrail3-rs.toml, but the public CLI rejects invalid guardrail config before clippy family checks run",
}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    rows = classify_rows()
    output = render_ledger(rows)
    if args.check:
        current = OUTPUT_LEDGER.read_text(encoding="utf-8")
        if current != output:
            print("kept test disposition ledger is not classified from current sources")
            return 1
        print("kept test disposition ledger classification is current")
        return 0

    OUTPUT_LEDGER.write_text(output, encoding="utf-8")
    counts = Counter(row["disposition"] for row in rows)
    print(
        "classified kept test dispositions "
        + " ".join(f"{status}:{count}" for status, count in sorted(counts.items()))
    )
    return 0


def classify_rows() -> list[dict[str, Any]]:
    ledger = load_toml(SOURCE_LEDGER)
    rows = ledger.get("test", [])
    output: list[dict[str, Any]] = []
    for row in rows:
        if not isinstance(row, dict) or row.get("status") != "kept_compile_contract":
            continue
        test_path = str(row["test_path"])
        test_name = str(row["test_name"])
        disposition, reason = classify_row(test_path, test_name)
        output.append(
            {
                "test_path": test_path,
                "test_name": test_name,
                "line": int(row["line"]),
                "disposition": disposition,
                "reason": reason,
            }
        )
    return output


def load_toml(path: Path) -> dict[str, Any]:
    with path.open("rb") as file:
        return tomllib.load(file)


def classify_row(test_path: str, test_name: str) -> tuple[str, str]:
    if "/contract_tests/" in test_path or "-hook-contract/" in test_path:
        return (
            "keep_public_api_contract",
            "hook-contract crates define public policy contracts that should remain compile/API tests until replaced by API snapshots",
        )
    if test_path.startswith("apps/guardrail3-rs/crates/io/inbound/cli/"):
        return (
            "covered_by_cli_output",
            "CLI parse, rejected argument, stdout, stderr, and exit behavior is captured by the g3rs-cli-output fixture3 suite",
        )
    if test_path.startswith("apps/guardrail3-rs/crates/io/outbound/report/"):
        return (
            "covered_by_renderer_output",
            "plain text report formatting behavior is captured by the g3rs-report-output fixture3 suite",
        )
    if test_path.startswith("apps/guardrail3-rs/crates/logic/validate-command/"):
        return (
            "needs_validate_command_output",
            "validate-command behavior needs CLI command output for cargo gates, staged paths, and family selection",
        )
    if test_path.startswith("apps/guardrail3-rs/crates/logic/family-runner-process/"):
        return (
            "needs_family_runner_output",
            "family runner behavior needs runner-level output for hook injection and family contract aggregation",
        )
    if "/g3rs-" in test_path and "-ingestion/" in test_path:
        return (
            "keep_internal_unit_test",
            "ingestion crates are internal module-boundary packages; keep current unit coverage until the behavior is replaced by a user-visible CLI fixture or deleted as internal-shape-only",
        )
    if test_path.startswith("packages/rs/cargo/g3rs-cargo-config-checks/"):
        return classify_cargo_config_row(test_name)
    if test_path.startswith("packages/rs/clippy/g3rs-clippy-config-checks/"):
        return classify_clippy_config_row(test_name)
    if "/run_tests/" in test_path:
        return (
            "needs_family_runner_output",
            "family run behavior needs runner-level output for dispatch, aggregation, and inactive-family behavior",
        )
    if is_rule_sidecar_behavior(test_path):
        return (
            "needs_rule_fixture_or_golden_output",
            "direct rule-sidecar behavior should be represented by fixture output or a rule-level golden snapshot",
        )
    return (
        "needs_family_runner_output",
        "remaining non-ingestion test behavior is runner or aggregation behavior unless a narrower fixture output supersedes it",
    )


def classify_cargo_config_row(test_name: str) -> tuple[str, str]:
    if test_name in CARGO_CLI_COVERED_FIXTURES:
        fixture = CARGO_CLI_COVERED_FIXTURES[test_name]
        return (
            "covered_by_cli_output",
            f"covered by g3rs-validate family-rule fixture {fixture}",
        )
    if test_name in CARGO_INTERNAL_ONLY_REASONS:
        return (
            "keep_internal_unit_test",
            CARGO_INTERNAL_ONLY_REASONS[test_name],
        )
    return (
        "needs_rule_fixture_or_golden_output",
        "direct cargo rule-sidecar behavior should be represented by family-rule CLI fixture output",
    )


def classify_clippy_config_row(test_name: str) -> tuple[str, str]:
    if test_name in CLIPPY_CLI_COVERED_FIXTURES:
        fixture = CLIPPY_CLI_COVERED_FIXTURES[test_name]
        return (
            "covered_by_cli_output",
            f"covered by g3rs-validate family-rule fixture {fixture}",
        )
    if test_name in CLIPPY_INTERNAL_ONLY_REASONS:
        return (
            "keep_internal_unit_test",
            CLIPPY_INTERNAL_ONLY_REASONS[test_name],
        )
    return (
        "needs_rule_fixture_or_golden_output",
        "direct clippy rule-sidecar behavior should be represented by family-rule CLI fixture output",
    )


def is_rule_sidecar_behavior(test_path: str) -> bool:
    path = REPO_ROOT / test_path
    if "/rule_tests/" in test_path:
        return True
    for parent in path.parents:
        name = parent.name
        if not name.endswith("_tests"):
            continue
        sibling_rule = parent.parent / f"{name.removesuffix('_tests')}.rs"
        if sibling_rule.is_file():
            return True
    return False


def render_ledger(rows: list[dict[str, Any]]) -> str:
    lines = [
        "# Generated by scripts/behavior/classify-kept-test-dispositions.py",
        "# Rows come from behavior/migration/g3rs-test-fixture-ledger.toml where status = \"kept_compile_contract\".",
        "",
    ]
    for row in rows:
        lines.append("[[test]]")
        for key in ("test_path", "test_name", "line", "disposition", "reason"):
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

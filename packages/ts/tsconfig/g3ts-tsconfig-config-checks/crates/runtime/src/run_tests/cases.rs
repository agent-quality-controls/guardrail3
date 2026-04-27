use g3ts_tsconfig_config_checks_assertions::run as assertions;

use super::helpers::{
    base_root_inline_baseline, broken_chain, external_extends, golden_extends, missing,
    parse_error, standalone_missing_inline, weak_effective_flags,
};

#[test]
fn missing_config_reports_only_exists_error() {
    let results = super::super::check(&missing());

    assertions::assert_exact(
        &results,
        &[assertions::error(
            "g3ts-tsconfig/exists",
            "tsconfig missing",
            "No root `tsconfig.json` or `tsconfig.base.json` file was found. Add a root TypeScript config.",
            None,
            false,
        )],
    );
}

#[test]
fn parse_error_reports_exists_inventory_and_parse_error() {
    let results = super::super::check(&parse_error());

    assertions::assert_exact_ids(
        &results,
        &["g3ts-tsconfig/exists", "g3ts-tsconfig/parseable"],
    );
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-tsconfig/parseable",
            "tsconfig parse error",
            "Failed to parse `tsconfig.json` as tsconfig JSONC: synthetic parse failure",
            Some("tsconfig.json"),
            false,
        )],
    );
}

#[test]
fn golden_extends_config_reports_inventory_only() {
    let results = super::super::check(&golden_extends());

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-tsconfig/exists",
            "g3ts-tsconfig/parseable",
            "g3ts-tsconfig/extends-chain-resolves",
            "g3ts-tsconfig/extends-or-inline",
            "g3ts-tsconfig/strict-baseline",
        ],
    );
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "g3ts-tsconfig/strict-baseline",
            "strict tsconfig baseline enforced",
            "The effective tsconfig keeps the required 12 strict boolean flags.",
            Some("tsconfig.json"),
            true,
        )],
    );
}

#[test]
fn broken_chain_reports_extends_error_only() {
    let results = super::super::check(&broken_chain());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-tsconfig/extends-chain-resolves",
            "tsconfig extends chain broken",
            "Local `extends` entries could not be resolved cleanly: `../../tsconfig.base.json` resolved to missing path `/tmp/tsconfig.base.json`.",
            Some("tsconfig.json"),
            false,
        )],
    );
    assertions::assert_no_findings_for_id(&results, "g3ts-tsconfig/strict-baseline");
}

#[test]
fn standalone_root_without_inline_baseline_reports_contract_error() {
    let results = super::super::check(&standalone_missing_inline());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-tsconfig/extends-or-inline",
            "standalone tsconfig misses inline strict baseline",
            "Root `tsconfig.json` does not use `extends`, so it must carry the strict baseline inline. Missing or invalid flags: noImplicitReturns, noUnusedLocals, noUnusedParameters, noUncheckedIndexedAccess, exactOptionalPropertyTypes, noPropertyAccessFromIndexSignature, noImplicitOverride, noFallthroughCasesInSwitch, forceConsistentCasingInFileNames, allowUnreachableCode, allowUnusedLabels.",
            Some("tsconfig.json"),
            false,
        )],
    );
}

#[test]
fn weak_effective_flags_report_baseline_error() {
    let results = super::super::check(&weak_effective_flags());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-tsconfig/strict-baseline",
            "strict tsconfig baseline weakened",
            "The effective tsconfig does not keep the required strict baseline. Problems: noUnusedLocals=false (expected true).",
            Some("tsconfig.json"),
            false,
        )],
    );
}

#[test]
fn external_extends_report_explicit_baseline_blocker() {
    let results = super::super::check(&external_extends());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-tsconfig/strict-baseline",
            "strict tsconfig baseline blocked by external extends",
            "The current wave cannot prove the strict baseline through external `extends` parents. Replace external inheritance with a local checked base or extend the local root directly.",
            Some("tsconfig.json"),
            false,
        )],
    );
}

#[test]
fn root_tsconfig_base_counts_as_root_surface() {
    let results = super::super::check(&base_root_inline_baseline());

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-tsconfig/exists",
            "g3ts-tsconfig/parseable",
            "g3ts-tsconfig/extends-chain-resolves",
            "g3ts-tsconfig/extends-or-inline",
            "g3ts-tsconfig/strict-baseline",
        ],
    );
    assertions::assert_contains(
        &results,
        &[
            assertions::info(
                "g3ts-tsconfig/exists",
                "tsconfig exists",
                "Found root TypeScript config `tsconfig.base.json`.",
                Some("tsconfig.base.json"),
                true,
            ),
            assertions::info(
                "g3ts-tsconfig/extends-or-inline",
                "standalone tsconfig carries strict baseline inline",
                "Root `tsconfig.base.json` does not use `extends`, but all strict baseline flags are present inline.",
                Some("tsconfig.base.json"),
                true,
            ),
            assertions::info(
                "g3ts-tsconfig/strict-baseline",
                "strict tsconfig baseline enforced",
                "The effective tsconfig keeps the required 12 strict boolean flags.",
                Some("tsconfig.base.json"),
                true,
            ),
        ],
    );
}

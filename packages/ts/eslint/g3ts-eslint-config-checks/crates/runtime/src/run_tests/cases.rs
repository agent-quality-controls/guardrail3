use g3ts_eslint_config_checks_assertions::run as assertions;

use super::helpers::{
    broken_carveouts, golden, missing, missing_rule_groups, parse_error, wrong_thresholds,
};

#[test]
fn missing_config_reports_only_exists_error() {
    let input = missing();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ESLINT-CONFIG-01",
            "eslint config missing",
            "No root `eslint.config.*` file was found. Add a root flat ESLint config.",
            None,
            false,
        )],
    );

    for id in [
        "TS-ESLINT-CONFIG-02",
        "TS-ESLINT-CONFIG-03",
        "TS-ESLINT-CONFIG-04",
        "TS-ESLINT-CONFIG-05",
        "TS-ESLINT-CONFIG-06",
        "TS-ESLINT-CONFIG-07",
        "TS-ESLINT-CONFIG-08",
        "TS-ESLINT-CONFIG-09",
        "TS-ESLINT-CONFIG-10",
        "TS-ESLINT-CONFIG-11",
        "TS-ESLINT-CONFIG-12",
        "TS-ESLINT-CONFIG-13",
        "TS-ESLINT-CONFIG-14",
        "TS-ESLINT-CONFIG-15",
    ] {
        assertions::assert_no_findings_for_id(&results, id);
    }
}

#[test]
fn parse_error_reports_exists_inventory_and_parse_error() {
    let input = parse_error();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ESLINT-CONFIG-01",
            "eslint config exists",
            "Found root ESLint config `eslint.config.mjs`.",
            Some("eslint.config.mjs"),
            true,
        )],
    );
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ESLINT-CONFIG-02",
            "eslint config parse error",
            "Failed to parse `eslint.config.mjs` through ESLint: synthetic parse failure",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn golden_config_reports_inventory_across_full_baseline() {
    let input = golden();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::info(
                "TS-ESLINT-CONFIG-01",
                "eslint config exists",
                "Found root ESLint config `eslint.config.mjs`.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-02",
                "eslint config parseable",
                "`eslint.config.mjs` parsed successfully through ESLint.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-03",
                "@typescript-eslint plugin active on TS source",
                "`@typescript-eslint` is active for the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-04",
                "projectService enabled for TS source",
                "`projectService: true` is active for the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-05",
                "no-explicit-any enforced on TS source",
                "`@typescript-eslint/no-explicit-any` is set to error on the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-06",
                "no-console enforced on TS source",
                "`no-console` is set to error on the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-07",
                "baseline thresholds and restricted imports enforced",
                "The TS source probe has the expected threshold rules and `no-restricted-imports`.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-08",
                "core TS baseline rules enforced",
                "The TS source probe has the core async and boundary baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-09",
                "type safety rules enforced",
                "The TS source probe has the extended type-safety rule baseline at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-10",
                "hygiene rules enforced",
                "The TS source probe has the extended hygiene rule baseline at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-11",
                "unicorn rules enforced",
                "The TS source probe has the required unicorn baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-12",
                "regexp rules enforced",
                "The TS source probe has the required regexp baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-13",
                "sonarjs rules enforced",
                "The TS source probe has the required sonarjs baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-14",
                "test carve-out for no-explicit-any present",
                "`@typescript-eslint/no-explicit-any` is off for the TS test probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "TS-ESLINT-CONFIG-15",
                "JS carve-out disables projectService",
                "The JS source probe does not enable `projectService: true`.",
                Some("eslint.config.mjs"),
                true,
            ),
        ],
    );
}

#[test]
fn wrong_thresholds_report_threshold_error() {
    let input = wrong_thresholds();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ESLINT-CONFIG-07",
            "baseline thresholds or restricted imports not enforced",
            "Wrong or missing threshold settings: `max-lines=400`.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_rule_groups_report_group_errors() {
    let input = missing_rule_groups();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ESLINT-CONFIG-03",
                "@typescript-eslint plugin missing on TS source",
                "The TS source probe does not include the `@typescript-eslint` plugin. Route TS source files through the typed-lint ESLint stack.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-08",
                "core TS baseline rules missing or relaxed",
                "The TS source probe must enforce these rules at error severity: `no-floating-promises`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-09",
                "type safety rules missing or relaxed",
                "The TS source probe must enforce these type-safety rules at error severity: `no-unsafe-assignment`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-10",
                "hygiene rules missing or relaxed",
                "The TS source probe must enforce these hygiene rules at error severity: `no-unnecessary-condition`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-11",
                "unicorn rules missing or relaxed",
                "The TS source probe must enforce these unicorn rules at error severity: `unicorn/no-keyword-prefix`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-12",
                "regexp rules missing or relaxed",
                "The TS source probe must enforce these regexp rules at error severity: `regexp/require-unicode-regexp`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-13",
                "sonarjs rules missing or relaxed",
                "The TS source probe must enforce these sonarjs rules at error severity: `sonarjs/cognitive-complexity`.",
                Some("eslint.config.mjs"),
                false,
            ),
        ],
    );
}

#[test]
fn broken_carveouts_report_test_and_js_errors() {
    let input = broken_carveouts();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ESLINT-CONFIG-14",
                "test carve-out for no-explicit-any missing",
                "`@typescript-eslint/no-explicit-any` must be off for the TS test probe.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-15",
                "JS carve-out for projectService missing",
                "The JS source probe must not enable `projectService: true`.",
                Some("eslint.config.mjs"),
                false,
            ),
        ],
    );
}

use g3ts_eslint_config_checks_assertions::run as assertions;

use super::helpers::{
    broken_carveouts, golden, missing, missing_plugin_stack, missing_rule_groups, parse_error,
    wrong_thresholds,
};

#[test]
fn missing_config_reports_only_exists_error() {
    let input = missing();
    let results = super::super::check(&input);

    assertions::assert_exact(
        &results,
        &[assertions::error(
            "g3ts-eslint/exists",
            "eslint config missing",
            "No root `eslint.config.*` file was found. Add a root flat ESLint config.",
            None,
            false,
        )],
    );
}

#[test]
fn parse_error_reports_exists_inventory_and_parse_error() {
    let input = parse_error();
    let results = super::super::check(&input);

    assertions::assert_exact_ids(&results, &["g3ts-eslint/exists", "g3ts-eslint/parseable"]);
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "g3ts-eslint/exists",
            "eslint config exists",
            "Found root ESLint config `eslint.config.mjs`.",
            Some("eslint.config.mjs"),
            true,
        )],
    );
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-eslint/parseable",
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

    assertions::assert_exact(
        &results,
        &[
            assertions::info(
                "g3ts-eslint/exists",
                "eslint config exists",
                "Found root ESLint config `eslint.config.mjs`.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/parseable",
                "eslint config parseable",
                "`eslint.config.mjs` parsed successfully through ESLint.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/ts-plugin-present",
                "@typescript-eslint plugin active on TS source",
                "`@typescript-eslint` is active for the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/project-service-enabled",
                "projectService enabled for TS source",
                "`projectService: true` is active for the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/no-explicit-any-error",
                "no-explicit-any enforced on TS source",
                "`@typescript-eslint/no-explicit-any` is set to error on the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/no-console-error",
                "no-console enforced on TS source",
                "`no-console` is set to error on the TS source probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/thresholds",
                "baseline thresholds and restricted imports enforced",
                "The TS source probe has the expected threshold rules and `no-restricted-imports`.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/core-baseline-rules",
                "core TS baseline rules enforced",
                "The TS source probe has the core async and boundary baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/type-safety-rules",
                "type safety rules enforced",
                "The TS source probe has the extended type-safety rule baseline at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/hygiene-rules",
                "hygiene rules enforced",
                "The TS source probe has the extended hygiene rule baseline at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/unicorn-rules",
                "unicorn rules enforced",
                "The TS source probe has the required unicorn baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/regexp-rules",
                "regexp rules enforced",
                "The TS source probe has the required regexp baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/sonarjs-rules",
                "sonarjs rules enforced",
                "The TS source probe has the required sonarjs baseline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/test-relaxations",
                "test carve-out for no-explicit-any present",
                "`@typescript-eslint/no-explicit-any` is off for the TS test probe.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/js-carveout",
                "JS carve-out disables typed linting",
                "The JS source probe does not enable `projectService: true` and does not enforce the representative typed-lint rules.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/plugin-stack",
                "TS plugin stack active on TS source",
                "The TS source probe has the required unicorn, regexp, and sonarjs plugin stack.",
                Some("eslint.config.mjs"),
                true,
            ),
            assertions::info(
                "g3ts-eslint/tsx-source-parity",
                "TSX source baseline matches TS source baseline",
                "The TSX source probe keeps the same typed-lint, threshold, plugin, and grouped-rule baseline as the TS source probe.",
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

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-eslint/exists",
            "g3ts-eslint/parseable",
            "g3ts-eslint/ts-plugin-present",
            "g3ts-eslint/project-service-enabled",
            "g3ts-eslint/no-explicit-any-error",
            "g3ts-eslint/no-console-error",
            "g3ts-eslint/thresholds",
            "g3ts-eslint/core-baseline-rules",
            "g3ts-eslint/type-safety-rules",
            "g3ts-eslint/hygiene-rules",
            "g3ts-eslint/unicorn-rules",
            "g3ts-eslint/regexp-rules",
            "g3ts-eslint/sonarjs-rules",
            "g3ts-eslint/test-relaxations",
            "g3ts-eslint/js-carveout",
            "g3ts-eslint/plugin-stack",
            "g3ts-eslint/tsx-source-parity",
        ],
    );
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-eslint/thresholds",
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

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-eslint/exists",
            "g3ts-eslint/parseable",
            "g3ts-eslint/ts-plugin-present",
            "g3ts-eslint/project-service-enabled",
            "g3ts-eslint/no-explicit-any-error",
            "g3ts-eslint/no-console-error",
            "g3ts-eslint/thresholds",
            "g3ts-eslint/core-baseline-rules",
            "g3ts-eslint/type-safety-rules",
            "g3ts-eslint/hygiene-rules",
            "g3ts-eslint/unicorn-rules",
            "g3ts-eslint/regexp-rules",
            "g3ts-eslint/sonarjs-rules",
            "g3ts-eslint/test-relaxations",
            "g3ts-eslint/js-carveout",
            "g3ts-eslint/plugin-stack",
            "g3ts-eslint/tsx-source-parity",
        ],
    );
    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "g3ts-eslint/ts-plugin-present",
                "@typescript-eslint plugin missing on TS source",
                "The TS source probe does not include the `@typescript-eslint` plugin. Route TS source files through the typed-lint ESLint stack.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/core-baseline-rules",
                "core TS baseline rules missing or relaxed",
                "The TS source probe must enforce these rules at error severity: `@typescript-eslint/no-floating-promises`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/type-safety-rules",
                "type safety rules missing or relaxed",
                "The TS source probe must enforce these type-safety rules at error severity: `@typescript-eslint/no-unsafe-assignment`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/hygiene-rules",
                "hygiene rules missing or relaxed",
                "The TS source probe must enforce these hygiene rules at error severity: `@typescript-eslint/no-unnecessary-condition`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/unicorn-rules",
                "unicorn rules missing or relaxed",
                "The TS source probe must enforce these unicorn rules at error severity: `unicorn/no-keyword-prefix`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/regexp-rules",
                "regexp rules missing or relaxed",
                "The TS source probe must enforce these regexp rules at error severity: `regexp/require-unicode-regexp`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/sonarjs-rules",
                "sonarjs rules missing or relaxed",
                "The TS source probe must enforce these sonarjs rules at error severity: `sonarjs/cognitive-complexity`.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/tsx-source-parity",
                "TSX source baseline drift detected",
                "The TSX source probe must enforce these grouped baseline rules at error severity: `@typescript-eslint/no-floating-promises`, `@typescript-eslint/no-unsafe-assignment`, `@typescript-eslint/no-unnecessary-condition`, `unicorn/no-keyword-prefix`, `regexp/require-unicode-regexp`, `sonarjs/cognitive-complexity`.",
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

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-eslint/exists",
            "g3ts-eslint/parseable",
            "g3ts-eslint/ts-plugin-present",
            "g3ts-eslint/project-service-enabled",
            "g3ts-eslint/no-explicit-any-error",
            "g3ts-eslint/no-console-error",
            "g3ts-eslint/thresholds",
            "g3ts-eslint/core-baseline-rules",
            "g3ts-eslint/type-safety-rules",
            "g3ts-eslint/hygiene-rules",
            "g3ts-eslint/unicorn-rules",
            "g3ts-eslint/regexp-rules",
            "g3ts-eslint/sonarjs-rules",
            "g3ts-eslint/test-relaxations",
            "g3ts-eslint/js-carveout",
            "g3ts-eslint/plugin-stack",
            "g3ts-eslint/tsx-source-parity",
        ],
    );
    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "g3ts-eslint/test-relaxations",
                "test carve-out for no-explicit-any missing",
                "`@typescript-eslint/no-explicit-any` must be off for the TS test probe.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "g3ts-eslint/js-carveout",
                "JS carve-out for typed linting missing",
                "The JS source probe must not enable `projectService: true`. The JS source probe must not enforce these representative typed-lint rules at error severity: `@typescript-eslint/no-unsafe-assignment`.",
                Some("eslint.config.mjs"),
                false,
            ),
        ],
    );
}

#[test]
fn missing_plugin_stack_reports_error() {
    let input = missing_plugin_stack();
    let results = super::super::check(&input);

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-eslint/exists",
            "g3ts-eslint/parseable",
            "g3ts-eslint/ts-plugin-present",
            "g3ts-eslint/project-service-enabled",
            "g3ts-eslint/no-explicit-any-error",
            "g3ts-eslint/no-console-error",
            "g3ts-eslint/thresholds",
            "g3ts-eslint/core-baseline-rules",
            "g3ts-eslint/type-safety-rules",
            "g3ts-eslint/hygiene-rules",
            "g3ts-eslint/unicorn-rules",
            "g3ts-eslint/regexp-rules",
            "g3ts-eslint/sonarjs-rules",
            "g3ts-eslint/test-relaxations",
            "g3ts-eslint/js-carveout",
            "g3ts-eslint/plugin-stack",
            "g3ts-eslint/tsx-source-parity",
        ],
    );
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-eslint/plugin-stack",
            "TS plugin stack missing on TS source",
            "The TS source probe must activate these plugins: `unicorn`, `regexp`, `sonarjs`.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

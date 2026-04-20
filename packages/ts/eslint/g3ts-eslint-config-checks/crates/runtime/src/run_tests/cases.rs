use eslint_config_parser::types::EslintRuleSeverity;
use g3ts_eslint_config_checks_assertions::run as assertions;

use super::helpers::{missing, parse_error, parsed};

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
    assertions::assert_no_findings_for_id(&results, "TS-ESLINT-CONFIG-02");
    assertions::assert_no_findings_for_id(&results, "TS-ESLINT-CONFIG-03");
    assertions::assert_no_findings_for_id(&results, "TS-ESLINT-CONFIG-04");
    assertions::assert_no_findings_for_id(&results, "TS-ESLINT-CONFIG-05");
    assertions::assert_no_findings_for_id(&results, "TS-ESLINT-CONFIG-06");
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
fn golden_parsed_input_reports_inventory_for_all_first_wave_rules() {
    let input = parsed(
        &["@typescript-eslint"],
        Some(true),
        EslintRuleSeverity::Error,
        EslintRuleSeverity::Error,
    );
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
        ],
    );
}

#[test]
fn missing_ts_plugin_reports_plugin_error() {
    let input = parsed(
        &[],
        Some(true),
        EslintRuleSeverity::Error,
        EslintRuleSeverity::Error,
    );
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ESLINT-CONFIG-03",
            "@typescript-eslint plugin missing on TS source",
            "The TS source probe does not include the `@typescript-eslint` plugin. Route TS source files through the typed-lint ESLint stack.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_project_service_and_wrong_rule_severities_report_errors() {
    let input = parsed(
        &["@typescript-eslint"],
        Some(false),
        EslintRuleSeverity::Warn,
        EslintRuleSeverity::Off,
    );
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ESLINT-CONFIG-04",
                "projectService missing for TS source",
                "The TS source probe does not enable `projectService: true`. Enable typed linting through the modern typescript-eslint project-service flow.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-05",
                "no-explicit-any not enforced on TS source",
                "`@typescript-eslint/no-explicit-any` must be set to error on the TS source probe.",
                Some("eslint.config.mjs"),
                false,
            ),
            assertions::error(
                "TS-ESLINT-CONFIG-06",
                "no-console not enforced on TS source",
                "`no-console` must be set to error on the TS source probe.",
                Some("eslint.config.mjs"),
                false,
            ),
        ],
    );
}

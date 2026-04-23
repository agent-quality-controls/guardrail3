use g3ts_astro_config_checks_assertions::run as assertions;

use super::helpers::{
    fake_astro_check_text_only, golden, missing_astro_check,
    missing_package_and_eslint_surface, missing_pipeline_wiring,
    missing_pipeline_rule_enforcement, missing_required_packages,
    optional_contracts_not_required,
};

#[test]
fn golden_config_reports_expected_inventory() {
    let input = golden();
    let results = super::super::check(&input);

    assertions::assert_exact(
        &results,
        &[
            assertions::info(
                "TS-ASTRO-CONFIG-01",
                "astro package present",
                "`package.json` includes `astro`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-02",
                "astro check present",
                "`package.json` invokes `astro check` in the app script surface.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-03",
                "astro ESLint plugin package present",
                "`package.json` includes `eslint-plugin-astro`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-04",
                "render validator package present",
                "`package.json` includes `@nuasite/checks`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-06",
                "astro pipeline ESLint plugin package present",
                "`package.json` includes `eslint-plugin-astro-pipeline`.",
                Some("package.json"),
                true,
            ),
            assertions::info(
                "TS-ASTRO-CONFIG-07",
                "astro pipeline ESLint plugin wired and effective",
                "`eslint.config.mjs` activates `astro-pipeline` and enforces the required Astro pipeline rules at error severity.",
                Some("eslint.config.mjs"),
                true,
            ),
        ],
    );
}

#[test]
fn missing_astro_check_reports_only_that_error() {
    let input = missing_astro_check();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-02",
            "astro check missing",
            "Could not prove a real `astro check` invocation from `package.json`. Add `astro check` to the app script surface.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn quoted_astro_check_text_does_not_satisfy_the_script_contract() {
    let input = fake_astro_check_text_only();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-02",
            "astro check missing",
            "Could not prove a real `astro check` invocation from `package.json`. Add `astro check` to the app script surface.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn missing_required_packages_report_package_contract_errors() {
    let input = missing_required_packages();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "astro package missing",
                "`package.json` does not include `astro`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "astro ESLint plugin package missing",
                "`package.json` does not include `eslint-plugin-astro`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-04",
                "render validator package missing",
                "`package.json` does not include the required Astro render validator package `@nuasite/checks`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "astro pipeline ESLint plugin package missing",
                "`package.json` does not include the required Astro pipeline ESLint plugin package `eslint-plugin-astro-pipeline`.",
                Some("package.json"),
                false,
            ),
        ],
    );
}

#[test]
fn missing_pipeline_wiring_reports_wiring_error() {
    let input = missing_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            "astro pipeline ESLint plugin not effective",
            "`eslint.config.mjs` does not both activate `astro-pipeline` and enforce the required Astro pipeline rules at error severity.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_rule_enforcement_reports_effectiveness_error() {
    let input = missing_pipeline_rule_enforcement();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            "astro pipeline ESLint plugin not effective",
            "`eslint.config.mjs` does not both activate `astro-pipeline` and enforce the required Astro pipeline rules at error severity.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn optional_contracts_do_not_fire_when_policy_is_disabled() {
    let input = optional_contracts_not_required();
    let results = super::super::check(&input);

    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-04");
    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-06");
    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-07");
}

#[test]
fn missing_package_and_eslint_surfaces_fail_closed() {
    let input = missing_package_and_eslint_surface();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "astro package missing",
                "`package.json` does not include `astro`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-02",
                "astro check missing",
                "Could not prove a real `astro check` invocation from `package.json`. Add `astro check` to the app script surface.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "astro ESLint plugin package missing",
                "`package.json` does not include `eslint-plugin-astro`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-04",
                "render validator package missing",
                "`package.json` does not include the required Astro render validator package `@nuasite/checks`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "astro pipeline ESLint plugin package missing",
                "`package.json` does not include the required Astro pipeline ESLint plugin package `eslint-plugin-astro-pipeline`.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-07",
                "astro pipeline ESLint plugin not effective",
                "`eslint.config.*` does not both activate `astro-pipeline` and enforce the required Astro pipeline rules at error severity.",
                Some("eslint.config.*"),
                false,
            ),
        ],
    );
}

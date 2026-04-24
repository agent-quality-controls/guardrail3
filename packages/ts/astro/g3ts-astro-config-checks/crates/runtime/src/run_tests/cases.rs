use g3ts_astro_config_checks_assertions::run as assertions;

use super::helpers::{
    astro_check_wrapper_forms, fake_astro_check_text_only, golden, missing_astro_check,
    missing_astro_plugin_wiring, missing_package_eslint_and_astro_config_surfaces,
    missing_pipeline_rule_enforcement, missing_pipeline_wiring, missing_required_packages,
    optional_contracts_not_required, route_only_pipeline_wiring,
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
                "TS-ASTRO-CONFIG-05",
                "astro ESLint plugin wired",
                "`eslint.config.mjs` activates `astro` for the required Astro source lanes.",
                Some("eslint.config.mjs"),
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
                "`eslint.config.mjs` activates `astro-pipeline` and enforces the required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes.",
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
            "Astro app scripts do not run `astro check`",
            "`package.json` does not run `astro check` in any app script. Add `astro check` to the script surface in `package.json`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked.",
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
            "Astro app scripts do not run `astro check`",
            "`package.json` does not run `astro check` in any app script. Add `astro check` to the script surface in `package.json`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked.",
            Some("package.json"),
            false,
        )],
    );
}

#[test]
fn wrapper_forms_satisfy_the_astro_check_contract() {
    let input = astro_check_wrapper_forms();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::info(
            "TS-ASTRO-CONFIG-02",
            "astro check present",
            "`package.json` invokes `astro check` in the app script surface.",
            Some("package.json"),
            true,
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
                "Astro app package is missing `astro`",
                "`package.json` does not list `astro` in dependencies or devDependencies. Add `astro` to `package.json`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "Astro app package is missing `eslint-plugin-astro`",
                "`package.json` does not list `eslint-plugin-astro` in dependencies or devDependencies. Add `eslint-plugin-astro` to `package.json`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "Astro app package is missing `eslint-plugin-astro-pipeline`",
                "`package.json` does not list `eslint-plugin-astro-pipeline` in dependencies or devDependencies. Add `eslint-plugin-astro-pipeline` to `package.json`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint.",
                Some("package.json"),
                false,
            ),
        ],
    );
}

#[test]
fn missing_pipeline_wiring_reports_wiring_error() {
    let input = missing_astro_plugin_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-05",
            "Astro ESLint lanes are not wired to the `astro` plugin",
            "`eslint.config.mjs` does not activate `astro` on the required Astro source lanes. Add the `astro` plugin to the Astro, TS, and TSX lane configs in `eslint.config.mjs`. Astro source files must run through the Astro plugin so framework lint rules actually execute.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn missing_pipeline_wiring_reports_pipeline_wiring_error() {
    let input = missing_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            "Astro ESLint lanes are not enforcing the required `astro-pipeline` rules",
            "`eslint.config.mjs` does not activate `astro-pipeline` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable the `astro-pipeline` plugin and set the required Astro pipeline rules to `error` in the Astro, TS, and TSX lane configs in `eslint.config.mjs`. Astro source files must run through the shared pipeline rules so route bypass checks and runtime MDX checks actually execute.",
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
            "Astro ESLint lanes are not enforcing the required `astro-pipeline` rules",
            "`eslint.config.mjs` does not activate `astro-pipeline` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable the `astro-pipeline` plugin and set the required Astro pipeline rules to `error` in the Astro, TS, and TSX lane configs in `eslint.config.mjs`. Astro source files must run through the shared pipeline rules so route bypass checks and runtime MDX checks actually execute.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn route_only_pipeline_wiring_still_fails_the_source_lane_contract() {
    let input = route_only_pipeline_wiring();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "TS-ASTRO-CONFIG-07",
            "Astro ESLint lanes are not enforcing the required `astro-pipeline` rules",
            "`eslint.config.mjs` does not activate `astro-pipeline` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable the `astro-pipeline` plugin and set the required Astro pipeline rules to `error` in the Astro, TS, and TSX lane configs in `eslint.config.mjs`. Astro source files must run through the shared pipeline rules so route bypass checks and runtime MDX checks actually execute.",
            Some("eslint.config.mjs"),
            false,
        )],
    );
}

#[test]
fn optional_contracts_do_not_fire_when_policy_is_disabled() {
    let input = optional_contracts_not_required();
    let results = super::super::check(&input);

    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-06");
    assertions::assert_no_findings_for_id(&results, "TS-ASTRO-CONFIG-07");
}

#[test]
fn missing_package_eslint_and_astro_config_surfaces_fail_closed() {
    let input = missing_package_eslint_and_astro_config_surfaces();
    let results = super::super::check(&input);

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "TS-ASTRO-CONFIG-01",
                "Astro app package is missing `astro`",
                "`package.json` does not list `astro` in dependencies or devDependencies. Add `astro` to `package.json`. Without that dependency entry, this app can drift away from the Astro framework contract without the package surface showing it.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-02",
                "Astro app scripts do not run `astro check`",
                "`package.json` does not run `astro check` in any app script. Add `astro check` to the script surface in `package.json`. Without that script entry, CI and local validation can pass while Astro type and content errors stay unchecked.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-03",
                "Astro app package is missing `eslint-plugin-astro`",
                "`package.json` does not list `eslint-plugin-astro` in dependencies or devDependencies. Add `eslint-plugin-astro` to `package.json`. Astro source files need the Astro ESLint plugin so Astro-specific lint rules can run.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-05",
                "Astro ESLint lanes are not wired to the `astro` plugin",
                "`eslint.config.*` does not activate `astro` on the required Astro source lanes. Add the `astro` plugin to the Astro, TS, and TSX lane configs in `eslint.config.*`. Astro source files must run through the Astro plugin so framework lint rules actually execute.",
                Some("eslint.config.*"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-06",
                "Astro app package is missing `eslint-plugin-astro-pipeline`",
                "`package.json` does not list `eslint-plugin-astro-pipeline` in dependencies or devDependencies. Add `eslint-plugin-astro-pipeline` to `package.json`. Astro source-pipeline rules must come from the shared ESLint plugin so route bypasses fail in lint.",
                Some("package.json"),
                false,
            ),
            assertions::error(
                "TS-ASTRO-CONFIG-07",
                "Astro ESLint lanes are not enforcing the required `astro-pipeline` rules",
                "`eslint.config.*` does not activate `astro-pipeline` with all required Astro pipeline rules at error severity on the Astro, TS, and TSX source lanes. Enable the `astro-pipeline` plugin and set the required Astro pipeline rules to `error` in the Astro, TS, and TSX lane configs in `eslint.config.*`. Astro source files must run through the shared pipeline rules so route bypass checks and runtime MDX checks actually execute.",
                Some("eslint.config.*"),
                false,
            ),
        ],
    );
}

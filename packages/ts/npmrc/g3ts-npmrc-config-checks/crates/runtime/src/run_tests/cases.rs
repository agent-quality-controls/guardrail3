use g3ts_npmrc_config_checks_assertions::run as assertions;

use super::helpers::{
    golden_root, missing_root, not_package_manager_root, root_missing_required_settings,
    root_parse_error, root_with_duplicate_keys, root_with_weakened_values,
};

#[test]
fn missing_root_reports_only_exists_error() {
    let results = super::super::check(&missing_root());

    assertions::assert_exact(
        &results,
        &[assertions::error(
            "g3ts-npmrc/root-exists",
            "root .npmrc missing",
            "No root `.npmrc` file was found. Add a root package-manager config.",
            None,
            false,
        )],
    );
}

#[test]
fn parse_error_reports_exists_inventory_and_parse_error() {
    let results = super::super::check(&root_parse_error());

    assertions::assert_exact_ids(
        &results,
        &["g3ts-npmrc/root-exists", "g3ts-npmrc/root-parseable"],
    );
    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-npmrc/root-parseable",
            "root .npmrc parse error",
            "Failed to parse root `.npmrc`: synthetic parse failure",
            Some(".npmrc"),
            false,
        )],
    );
}

#[test]
fn golden_root_reports_expected_inventory() {
    let results = super::super::check(&golden_root());

    assertions::assert_exact_ids(
        &results,
        &[
            "g3ts-npmrc/root-exists",
            "g3ts-npmrc/root-parseable",
            "g3ts-npmrc/duplicate-keys",
            "g3ts-npmrc/required-settings-present",
            "g3ts-npmrc/required-settings-strong-enough",
            "g3ts-npmrc/extra-settings-inventory",
        ],
    );
    assertions::assert_contains(
        &results,
        &[assertions::info(
            "g3ts-npmrc/extra-settings-inventory",
            "root .npmrc has extra setting",
            "Extra root .npmrc setting `minimum-release-age-exclude=@base-ui/react` is outside the current baseline. Keep it only if it is intentional.",
            Some(".npmrc"),
            true,
        )],
    );
}

#[test]
fn duplicate_keys_report_under_duplicate_rule() {
    let results = super::super::check(&root_with_duplicate_keys());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-npmrc/duplicate-keys",
            "root .npmrc has duplicate key",
            "Duplicate root .npmrc key `strict-peer-dependencies` is not allowed; pnpm uses the last value and can mask earlier settings.",
            Some(".npmrc"),
            false,
        )],
    );
}

#[test]
fn missing_required_settings_report_under_presence_rule() {
    let results = super::super::check(&root_missing_required_settings());

    assertions::assert_contains(
        &results,
        &[assertions::error(
            "g3ts-npmrc/required-settings-present",
            "root .npmrc baseline settings are missing",
            "The root .npmrc is missing required settings: disallow-workspace-cycles, minimum-release-age, block-exotic-subdeps, trust-policy.",
            Some(".npmrc"),
            false,
        )],
    );
}

#[test]
fn weakened_values_report_under_strength_rule() {
    let results = super::super::check(&root_with_weakened_values());

    assertions::assert_contains(
        &results,
        &[
            assertions::error(
                "g3ts-npmrc/required-settings-strong-enough",
                "root .npmrc setting is weaker than baseline",
                "Root .npmrc setting `strict-peer-dependencies` is `false` but must be `true`.",
                Some(".npmrc"),
                false,
            ),
            assertions::error(
                "g3ts-npmrc/required-settings-strong-enough",
                "root .npmrc setting is weaker than baseline",
                "Root .npmrc setting `engine-strict` is `false` but must be `true`.",
                Some(".npmrc"),
                false,
            ),
            assertions::error(
                "g3ts-npmrc/required-settings-strong-enough",
                "root .npmrc setting is weaker than baseline",
                "Root .npmrc setting `minimum-release-age` is `60` but must be `1440`.",
                Some(".npmrc"),
                false,
            ),
            assertions::error(
                "g3ts-npmrc/required-settings-strong-enough",
                "root .npmrc setting is weaker than baseline",
                "Root .npmrc setting `block-exotic-subdeps` is `false` but must be `true`.",
                Some(".npmrc"),
                false,
            ),
        ],
    );
}

#[test]
fn non_package_manager_root_produces_no_findings() {
    let results = super::super::check(&not_package_manager_root());
    assertions::assert_exact(&results, &[]);
}

use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

pub fn assert_fail_open_validate_error_for_input(input: &G3TsPackageChecksInput) {
    let results = g3ts_package_config_checks_runtime::check(input);
    assert_fail_open_validate_error(&results);
}

pub fn assert_fail_closed_validate_inventory_for_input(input: &G3TsPackageChecksInput) {
    let results = g3ts_package_config_checks_runtime::check(input);
    assert_fail_closed_validate_inventory(&results);
}

pub fn assert_fail_open_validate_error(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3ts-package/validate-script-fail-closed"
                && result.severity() == G3Severity::Error
                && result.title() == "validate script is not fail-closed"
                && result.message()
                    == "The root package manifest must define `validate` with supported shell syntax and no reachable `||` fallback."
                && result.file() == Some("package.json")
                && !result.inventory()
        }),
        "expected fail-open validate script error, got: {results:?}",
    );
}

pub fn assert_fail_closed_validate_inventory(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3ts-package/validate-script-fail-closed"
                && result.severity() == G3Severity::Info
                && result.title() == "validate script is fail-closed"
                && result.message()
                    == "The root package manifest defines `validate` with supported shell syntax and no reachable `||` fallback."
                && result.file() == Some("package.json")
                && result.inventory()
        }),
        "expected fail-closed validate script inventory, got: {results:?}",
    );
}

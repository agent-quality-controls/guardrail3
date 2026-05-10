use g3ts_package_types::G3TsPackageChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// Assert that running the runtime on `input` produces a missing-validate-script error.
///
/// # Panics
/// Panics if the expected error is missing.
pub fn assert_missing_validate_error_for_input(input: &G3TsPackageChecksInput) {
    let results = g3ts_package_config_checks_runtime::check(input);
    assert_missing_validate_error(&results);
}

/// Assert `results` contains the expected missing-validate-script error.
///
/// # Panics
/// Panics if the expected error is missing.
pub fn assert_missing_validate_error(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "g3ts-package/validate-script-present"
                && result.severity() == G3Severity::Error
                && result.title() == "validate script is missing"
                && result.message()
                    == "The root package manifest must define the standard `validate` script."
                && result.file() == Some("package.json")
                && !result.inventory()
        }),
        "expected missing validate script error, got: {results:?}",
    );
}

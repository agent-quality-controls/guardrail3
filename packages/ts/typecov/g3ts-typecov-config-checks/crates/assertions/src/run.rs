/// Asserts that the runtime emits a result with the given rule `id`.
///
/// # Panics
/// Panics when no result with `id` is present in the runtime output.
pub fn assert_runtime_has_rule(input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput, id: &str) {
    let results = g3ts_typecov_config_checks_runtime::check(input);
    assert!(
        results.iter().any(|result| result.id() == id),
        "expected check result `{id}`"
    );
}

/// Asserts that the runtime emits an error result for `id` at `file`.
///
/// # Panics
/// Panics when no result with `id` is present, when the severity is not `Error`,
/// when the file path differs, or when title/message are empty.
pub fn assert_runtime_error(
    input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let expected = guardrail3_check_types::G3Severity::Error;
    assert_runtime_severity(input, id, file, expected);
}

/// Asserts that the runtime emits an info result for `id` at `file`.
///
/// # Panics
/// Panics when no result with `id` is present, when the severity is not `Info`,
/// when the file path differs, or when title/message are empty.
pub fn assert_runtime_info(
    input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let info_severity = guardrail3_check_types::G3Severity::Info;
    assert_runtime_severity(input, id, file, info_severity);
}

/// Asserts that the runtime emits a result for `id` at `file` with `expected_severity`.
///
/// # Panics
/// Panics when no result with `id` is present, when severity differs, when the file path differs,
/// or when title/message are empty.
#[expect(
    clippy::panic,
    reason = "Assertion helpers are test-only; panicking with a clear message is the documented contract."
)]
fn assert_runtime_severity(
    input: &g3ts_typecov_types::G3TsTypecovConfigChecksInput,
    id: &str,
    file: Option<&str>,
    expected_severity: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_typecov_config_checks_runtime::check(input);
    let result = results
        .iter()
        .find(|result| result.id() == id)
        .unwrap_or_else(|| panic!("expected check result `{id}`"));
    assert_eq!(
        result.severity(),
        expected_severity,
        "severity mismatch for `{id}`"
    );
    assert_eq!(result.file(), file, "file mismatch for `{id}`");
    assert!(!result.title().is_empty(), "title must not be empty");
    assert!(!result.message().is_empty(), "message must not be empty");
}

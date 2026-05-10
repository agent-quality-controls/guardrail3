/// Asserts that the runtime emits a result with the given rule `id`.
///
/// # Panics
/// Panics when no result with `id` is present in the runtime output.
pub fn assert_runtime_has_rule(input: &g3ts_fmt_types::G3TsFmtConfigChecksInput, id: &str) {
    let results = g3ts_fmt_config_checks_runtime::check(input);
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
#[expect(
    clippy::panic,
    reason = "Assertion helpers are test-only; panicking with a clear message is the documented contract."
)]
pub fn assert_runtime_error(
    input: &g3ts_fmt_types::G3TsFmtConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let results = g3ts_fmt_config_checks_runtime::check(input);
    let result = results
        .iter()
        .find(|result| result.id() == id)
        .unwrap_or_else(|| panic!("expected check result `{id}`"));
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Error,
        "severity mismatch for `{id}`"
    );
    assert_eq!(result.file(), file, "file mismatch for `{id}`");
    assert!(!result.title().is_empty(), "title must not be empty");
    assert!(!result.message().is_empty(), "message must not be empty");
}

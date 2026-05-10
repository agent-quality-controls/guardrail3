/// Assert the spelling-config-checks runtime emits a check result with the given id.
///
/// # Panics
/// Panics if no check result with `id` is found.
pub fn assert_runtime_has_rule(
    input: &g3ts_spelling_types::G3TsSpellingConfigChecksInput,
    id: &str,
) {
    let results = g3ts_spelling_config_checks_runtime::check(input);
    assert!(
        results.iter().any(|result| result.id() == id),
        "expected check result `{id}`"
    );
}

/// Assert the runtime emits an error-severity result with the expected id and file.
///
/// # Panics
/// Panics if the result is missing or its severity/file/title/message disagree with expectations.
pub fn assert_runtime_error(
    input: &g3ts_spelling_types::G3TsSpellingConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let results = g3ts_spelling_config_checks_runtime::check(input);
    let result = results
        .iter()
        .find(|result| result.id() == id)
        .unwrap_or_else(|| unreachable!("expected check result `{id}`"));
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Error,
        "severity mismatch for `{id}`"
    );
    assert_eq!(result.file(), file, "file mismatch for `{id}`");
    assert!(!result.title().is_empty(), "title must not be empty");
    assert!(!result.message().is_empty(), "message must not be empty");
}

/// Assert the runtime emits an info-severity result with the expected id and file.
///
/// # Panics
/// Panics if the result is missing or its severity/file/title/message disagree with expectations.
pub fn assert_runtime_info(
    input: &g3ts_spelling_types::G3TsSpellingConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let results = g3ts_spelling_config_checks_runtime::check(input);
    let result = results
        .iter()
        .find(|result| result.id() == id)
        .unwrap_or_else(|| unreachable!("expected check result `{id}`"));
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Info,
        "severity mismatch for `{id}`"
    );
    assert_eq!(result.file(), file, "file mismatch for `{id}`");
    assert!(!result.title().is_empty(), "title must not be empty");
    assert!(!result.message().is_empty(), "message must not be empty");
}

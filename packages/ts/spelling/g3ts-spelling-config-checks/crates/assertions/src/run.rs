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

pub fn assert_runtime_error(
    input: &g3ts_spelling_types::G3TsSpellingConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let results = g3ts_spelling_config_checks_runtime::check(input);
    let Some(result) = results.iter().find(|result| result.id() == id) else {
        assert!(false, "expected check result `{id}`");
        return;
    };
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Error,
        "severity mismatch for `{id}`"
    );
    assert_eq!(result.file(), file, "file mismatch for `{id}`");
    assert!(!result.title().is_empty(), "title must not be empty");
    assert!(!result.message().is_empty(), "message must not be empty");
}

pub fn assert_runtime_info(
    input: &g3ts_spelling_types::G3TsSpellingConfigChecksInput,
    id: &str,
    file: Option<&str>,
) {
    let results = g3ts_spelling_config_checks_runtime::check(input);
    let Some(result) = results.iter().find(|result| result.id() == id) else {
        assert!(false, "expected check result `{id}`");
        return;
    };
    assert_eq!(
        result.severity(),
        guardrail3_check_types::G3Severity::Info,
        "severity mismatch for `{id}`"
    );
    assert_eq!(result.file(), file, "file mismatch for `{id}`");
    assert!(!result.title().is_empty(), "title must not be empty");
    assert!(!result.message().is_empty(), "message must not be empty");
}

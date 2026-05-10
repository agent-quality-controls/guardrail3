/// Asserts the rule and panics on mismatch.
///
/// # Panics
///
/// Panics when the assertion fails.
pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_seo_config_checks_runtime::check(input);
    assert_exact_ids(&results, expected);
    assert!(
        results
            .iter()
            .all(|result| result.severity() == guardrail3_check_types::G3Severity::Info),
        "expected all exact-id findings to be Info, got {results:#?}"
    );
}

/// Asserts the rule and panics on mismatch.
///
/// # Panics
///
/// Panics when the assertion fails.
pub fn assert_runtime_error_id(
    input: &g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    expected_id: &str,
) {
    let results = g3ts_astro_seo_config_checks_runtime::check(input);
    assert!(
        results.iter().any(|result| {
            result.id() == expected_id
                && result.severity() == guardrail3_check_types::G3Severity::Error
        }),
        "expected error for {expected_id}, got {results:#?}"
    );
}

/// Asserts the rule and panics on mismatch.
///
/// # Panics
///
/// Panics when the assertion fails.
pub fn assert_runtime_check_id_severity(
    input: &g3ts_astro_seo_types::G3TsAstroSeoConfigChecksInput,
    expected_id: &str,
    expected_severity: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_astro_seo_config_checks_runtime::check(input);
    assert!(
        results
            .iter()
            .any(|result| { result.id() == expected_id && result.severity() == expected_severity }),
        "expected {expected_severity:?} for {expected_id}, got {results:#?}"
    );
}

/// Asserts the rule and panics on mismatch.
///
/// # Panics
///
/// Panics when the assertion fails.
pub fn assert_exact_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

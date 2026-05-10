/// Fails the calling test when the runtime check on `input` produces ids that do not match `expected` exactly and in order.
///
/// # Panics
/// Panics when the produced id list differs from `expected`, which the assertion treats as a test failure.
pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_content_types::G3TsAstroContentConfigChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_content_config_checks_runtime::check(input);
    assert_exact_ids(&results, expected);
}

/// Fails the calling test when `results` ids do not match `expected` exactly and in order.
///
/// # Panics
/// Panics on id mismatch, which the assertion treats as a test failure.
pub fn assert_exact_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

/// Fails the calling test when the runtime check does not produce a finding with `id` at severity `expected`.
///
/// # Panics
/// Panics when no finding with `id` exists or its severity differs from `expected`, which the assertion treats as a test failure.
pub fn assert_runtime_check_id_severity(
    input: &g3ts_astro_content_types::G3TsAstroContentConfigChecksInput,
    id: &str,
    expected: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_astro_content_config_checks_runtime::check(input);
    let actual_severity = results
        .iter()
        .find(|result| result.id() == id)
        .map(guardrail3_check_types::G3CheckResult::severity);
    assert_eq!(
        actual_severity,
        Some(expected),
        "expected finding `{id}` with severity {expected:?}, got {actual_severity:?}"
    );
}

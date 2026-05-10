/// Asserts that running the runtime check on `input` produces exactly `expected` ids.
///
/// # Panics
///
/// Panics when the runtime check produces ids different from `expected`.
pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_setup_types::G3TsAstroSetupConfigChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_setup_config_checks_runtime::check(input);
    assert_exact_ids(&results, expected);
}

/// Asserts that `results` ids match `expected` exactly, in order.
///
/// # Panics
///
/// Panics when the rule ids in `results` do not match `expected`.
pub fn assert_exact_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

/// Asserts the runtime check emits a finding for `id` with `expected` severity.
///
/// # Panics
///
/// Panics when no finding with `id` is emitted, or when its severity does not match.
pub fn assert_runtime_check_id_severity(
    input: &g3ts_astro_setup_types::G3TsAstroSetupConfigChecksInput,
    id: &str,
    expected: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_astro_setup_config_checks_runtime::check(input);
    let result = results.iter().find(|result| result.id() == id);
    let Some(result) = result else {
        unreachable!("expected finding `{id}` to exist in results: {results:#?}");
    };
    assert_eq!(
        result.severity(),
        expected,
        "finding `{id}` severity mismatch"
    );
}

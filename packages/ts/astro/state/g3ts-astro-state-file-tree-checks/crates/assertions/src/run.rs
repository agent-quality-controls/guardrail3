/// Asserts that running the runtime check on `input` produces exactly `expected` ids.
///
/// # Panics
///
/// Panics when the runtime check produces ids different from `expected`.
pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_state_types::G3TsAstroStateFileTreeChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_state_file_tree_checks_runtime::check(input);
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

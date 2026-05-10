/// Run the runtime check for `input` and assert the exact ordered list of finding ids.
///
/// # Panics
///
/// Panics when the produced finding id sequence does not match `expected`.
pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_content_types::G3TsAstroContentFileTreeChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_content_file_tree_checks_runtime::check(input);
    assert_exact_ids(&results, expected);
}

/// Assert the exact ordered list of finding ids in `results`.
///
/// # Panics
///
/// Panics when the finding id sequence does not match `expected`.
pub fn assert_exact_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

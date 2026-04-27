pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_content_types::G3TsAstroContentConfigChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_content_config_checks_runtime::check(input);
    assert_exact_ids(&results, expected);
}

pub fn assert_exact_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

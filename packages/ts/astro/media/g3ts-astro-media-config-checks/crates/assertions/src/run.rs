pub fn assert_runtime_check_exact_ids(
    input: &g3ts_astro_media_types::G3TsAstroMediaConfigChecksInput,
    expected: &[&str],
) {
    let results = g3ts_astro_media_config_checks_runtime::check(input);
    assert_exact_ids(&results, expected);
}

pub fn assert_exact_ids(results: &[guardrail3_check_types::G3CheckResult], expected: &[&str]) {
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

pub fn assert_runtime_check_id_severity(
    input: &g3ts_astro_media_types::G3TsAstroMediaConfigChecksInput,
    id: &str,
    expected: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_astro_media_config_checks_runtime::check(input);
    let Some(result) = results.iter().find(|result| result.id() == id) else {
        assert!(false, "expected finding `{id}` to exist");
        return;
    };
    assert_eq!(
        result.severity(),
        expected,
        "finding `{id}` severity mismatch"
    );
}

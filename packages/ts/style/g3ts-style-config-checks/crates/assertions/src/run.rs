pub fn assert_runtime_check_exact_ids(
    input: &g3ts_style_types::G3TsStyleConfigChecksInput,
    expected: &[&str],
) {
    let results = g3ts_style_config_checks_runtime::check(input);
    let actual = results
        .iter()
        .map(guardrail3_check_types::G3CheckResult::id)
        .collect::<Vec<_>>();
    assert_eq!(actual, expected, "exact finding id order mismatch");
}

pub fn assert_runtime_check_id_severity(
    input: &g3ts_style_types::G3TsStyleConfigChecksInput,
    id: &str,
    expected: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_style_config_checks_runtime::check(input);
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

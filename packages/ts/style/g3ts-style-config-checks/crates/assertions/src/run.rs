/// Assert that the runtime check, when run on `input`, emits findings whose
/// IDs exactly equal `expected`, in order.
///
/// # Panics
///
/// Panics when the emitted finding IDs differ from `expected`.
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

/// Assert that the finding with `id` exists and carries `expected` severity.
///
/// # Panics
///
/// Panics when no finding with `id` is emitted, or when its severity does
/// not match `expected`.
pub fn assert_runtime_check_id_severity(
    input: &g3ts_style_types::G3TsStyleConfigChecksInput,
    id: &str,
    expected: guardrail3_check_types::G3Severity,
) {
    let results = g3ts_style_config_checks_runtime::check(input);
    let severities = results
        .iter()
        .filter(|result| result.id() == id)
        .map(guardrail3_check_types::G3CheckResult::severity)
        .collect::<Vec<_>>();
    assert_eq!(
        severities,
        vec![expected],
        "finding `{id}` severity mismatch"
    );
}

/// Assert that the finding with `id` exists and its message contains
/// `expected` as a substring.
///
/// # Panics
///
/// Panics when no finding with `id` is emitted, or when its message does
/// not contain `expected`.
pub fn assert_runtime_check_message_contains(
    input: &g3ts_style_types::G3TsStyleConfigChecksInput,
    id: &str,
    expected: &str,
) {
    let results = g3ts_style_config_checks_runtime::check(input);
    let messages = results
        .iter()
        .filter(|result| result.id() == id)
        .map(|result| result.message().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        messages.len(),
        1,
        "expected exactly one finding with id `{id}`, got: {messages:?}"
    );
    let actual = messages.first().map_or("", String::as_str);
    assert!(
        actual.contains(expected),
        "finding `{id}` message should contain `{expected}`, actual: `{actual}`",
    );
}

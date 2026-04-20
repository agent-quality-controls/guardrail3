/// Checks the selected-family ordering returned by the validate-command logic.
///
/// # Panics
///
/// Panics if the selected family list does not match the expected ordering.
pub fn assert_selected_families(
    actual: &[guardrail3_ts_app_types::SupportedFamily],
    expected: &[guardrail3_ts_app_types::SupportedFamily],
) {
    assert_eq!(
        actual, expected,
        "selected families should keep the expected order"
    );
}

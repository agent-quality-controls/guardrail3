use guardrail3_ts_app_types::SupportedFamily;

/// Checks that the disabled-families list contains nothing.
///
/// # Panics
///
/// Panics if the list is non-empty.
pub fn assert_no_disabled(disabled: &[SupportedFamily], context: &str) {
    assert!(
        disabled.is_empty(),
        "{context}: expected no disabled families, got {disabled:?}",
    );
}

/// Checks that the disabled-families list contains the given family.
///
/// # Panics
///
/// Panics if `family` is missing from `disabled`.
pub fn assert_contains(disabled: &[SupportedFamily], family: SupportedFamily) {
    assert!(
        disabled.contains(&family),
        "expected {family:?} in disabled list, got {disabled:?}",
    );
}

/// Checks that the disabled-families list does NOT contain the given family.
///
/// # Panics
///
/// Panics if `family` is present in `disabled`.
pub fn assert_not_contains(disabled: &[SupportedFamily], family: SupportedFamily) {
    assert!(
        !disabled.contains(&family),
        "did not expect {family:?} in disabled list, got {disabled:?}",
    );
}

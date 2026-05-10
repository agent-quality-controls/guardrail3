/// Assert that the style-policy option evaluation rejects the value.
///
/// # Panics
///
/// Panics when `actual` is true.
pub fn assert_style_policy_option_rejected(actual: bool) {
    assert!(!actual, "style policy option should be rejected");
}

/// Assert that the style-policy option evaluation accepts the value.
///
/// # Panics
///
/// Panics when `actual` is false.
pub fn assert_style_policy_option_accepted(actual: bool) {
    assert!(actual, "style policy option should be accepted");
}

pub fn assert_style_policy_option_rejected(actual: bool) {
    assert!(!actual, "style policy option should be rejected");
}

pub fn assert_style_policy_option_accepted(actual: bool) {
    assert!(actual, "style policy option should be accepted");
}

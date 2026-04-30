pub fn assert_denylist_option_rejected(actual: bool) {
    assert!(!actual, "denyList option should be rejected");
}

pub fn assert_denylist_option_accepted(actual: bool) {
    assert!(actual, "denyList option should be accepted");
}

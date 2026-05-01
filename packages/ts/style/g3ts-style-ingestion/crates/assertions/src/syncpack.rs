pub fn assert_canonical_pin_group_accepted(actual: bool) {
    assert!(actual, "canonical Syncpack pin group should be accepted");
}

pub fn assert_canonical_pin_group_rejected(actual: bool) {
    assert!(!actual, "non-canonical Syncpack pin group should be rejected");
}

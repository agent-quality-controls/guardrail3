#[test]
#[allow(
    clippy::assertions_on_constants,
    clippy::missing_assert_message,
    clippy::redundant_pattern_matching,
    reason = "fixture needs weak matches pattern to exercise g3rs-test rule"
)]
fn weak_matches_assertion_probe() {
    assert!(matches!(Some(1_u8), Some(_)));
}

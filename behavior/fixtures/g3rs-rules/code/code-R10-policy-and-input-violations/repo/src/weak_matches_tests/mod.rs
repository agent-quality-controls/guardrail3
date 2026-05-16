#[test]
fn weak_matches_assertion_probe() {
    assert!(matches!(Some(1), Some(_)));
}

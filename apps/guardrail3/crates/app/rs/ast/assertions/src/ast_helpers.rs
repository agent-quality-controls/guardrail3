pub fn assert_single_cfg_attr_allow(
    actual_count: usize,
    actual_line: usize,
    actual_lint: &str,
    actual_is_always_true: bool,
    expected_line: usize,
    expected_lint: &str,
    expected_is_always_true: bool,
) {
    assert_eq!(actual_count, 1, "should find exactly one cfg_attr allow");
    assert_eq!(actual_line, expected_line, "cfg_attr allow line mismatch");
    assert_eq!(actual_lint, expected_lint, "cfg_attr allow lint mismatch");
    assert_eq!(
        actual_is_always_true,
        expected_is_always_true,
        "cfg_attr allow truthiness mismatch"
    );
}

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
        actual_is_always_true, expected_is_always_true,
        "cfg_attr allow truthiness mismatch"
    );
}

pub fn assert_dead_code_cfg_attr(
    actual_count: usize,
    actual_line: usize,
    actual_lint: &str,
    actual_is_always_true: bool,
    expected_line: usize,
    expected_is_always_true: bool,
) {
    assert_single_cfg_attr_allow(
        actual_count,
        actual_line,
        actual_lint,
        actual_is_always_true,
        expected_line,
        "dead_code",
        expected_is_always_true,
    );
}

pub fn assert_forbidden_macro_name(macros: &[(usize, String)], expected_name: &str) {
    assert_eq!(macros.len(), 1, "should find exactly one forbidden macro");
    let (_, name) = &macros[0];
    assert_eq!(name, expected_name);
}

pub fn assert_unwrap_expect_name(items: &[(usize, String)], expected_name: &str) {
    assert_eq!(items.len(), 1, "should find exactly one unwrap/expect call");
    let (_, name) = &items[0];
    assert_eq!(name, expected_name);
}

pub fn assert_same_line_reason(actual: Option<String>, expected: Option<&str>) {
    assert_eq!(actual.as_deref(), expected);
}

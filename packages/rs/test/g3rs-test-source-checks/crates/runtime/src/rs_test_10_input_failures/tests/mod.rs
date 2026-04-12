use guardrail3_check_types::G3Severity;

#[test]
fn reports_parse_failure_as_error_result() {
    let mut results = Vec::new();

    crate::rs_test_10_input_failures::check(
        "demo",
        "tests/broken.rs",
        "expected one of: `fn`, `struct`, `enum`",
        &mut results,
    );

    assert_eq!(results.len(), 1, "{results:#?}");
    let result = &results[0];
    assert_eq!(result.id(), "RS-TEST-SOURCE-10");
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "failed to read test input");
    assert_eq!(result.file(), Some("tests/broken.rs"));
    assert_eq!(result.message(), "expected one of: `fn`, `struct`, `enum`");
}

use g3rs_test_source_checks_assertions::rs_test_10_input_failures::rule as assertions;
use guardrail3_check_types::G3Severity;

#[test]
fn reports_parse_failure_as_error_result() {
    let results = assertions::check("tests/broken.rs", "fn {");

    assertions::assert_has_result(
        &results,
        "RS-TEST-SOURCE-10",
        G3Severity::Error,
        "failed to read test input",
        "tests/broken.rs",
    );
    assertions::assert_message_contains(
        &results,
        "Failed to parse Rust source file for test-family source analysis:",
    );
}

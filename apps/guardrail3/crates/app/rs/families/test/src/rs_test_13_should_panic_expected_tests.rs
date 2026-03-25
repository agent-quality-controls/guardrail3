use guardrail3_domain_report::Severity;

use super::super::parse::TestFunctionInfo;
use super::super::test_support::function_input;
use super::check;

#[test]
fn warns_on_should_panic_without_expected() {
    let input = function_input(
        "tests/panic.rs",
        TestFunctionInfo {
            line: 8,
            name: "panics_with_helpful_message".to_owned(),
            should_panic_line: Some(7),
            should_panic_has_expected: false,
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn accepts_expected_string() {
    let input = function_input(
        "tests/panic.rs",
        TestFunctionInfo {
            line: 8,
            name: "panics_with_helpful_message".to_owned(),
            should_panic_line: Some(7),
            should_panic_has_expected: true,
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.is_empty());
}

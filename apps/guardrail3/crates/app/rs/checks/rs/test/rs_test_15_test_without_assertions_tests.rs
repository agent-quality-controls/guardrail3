use crate::domain::report::Severity;

use super::super::parse::TestFunctionInfo;
use super::super::test_support::function_input;
use super::check;

#[test]
fn warns_when_test_has_no_assertions() {
    let input = function_input(
        "tests/api.rs",
        TestFunctionInfo {
            line: 4,
            name: "returns_value_for_valid_input".to_owned(),
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn accepts_result_return_or_assertions() {
    for function in [
        TestFunctionInfo {
            line: 4,
            name: "returns_result".to_owned(),
            has_result_return: true,
            ..Default::default()
        },
        TestFunctionInfo {
            line: 4,
            name: "has_assert".to_owned(),
            has_assertion_macro: true,
            ..Default::default()
        },
        TestFunctionInfo {
            line: 4,
            name: "calls_verify".to_owned(),
            has_assert_like_call: true,
            ..Default::default()
        },
    ] {
        let input = function_input("tests/api.rs", function);
        let mut results = Vec::new();
        check(&input, &mut results);
        assert!(results.is_empty());
    }
}

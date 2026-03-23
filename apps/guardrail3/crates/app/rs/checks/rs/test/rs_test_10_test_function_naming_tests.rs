use crate::domain::report::Severity;

use super::super::parse::TestFunctionInfo;
use super::super::test_support::function_input;
use super::check;

#[test]
fn warns_on_short_name() {
    let input = function_input(
        "tests/api.rs",
        TestFunctionInfo {
            line: 4,
            name: "tiny".to_owned(),
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn warns_on_numeric_suffix_pattern() {
    let input = function_input(
        "tests/api.rs",
        TestFunctionInfo {
            line: 4,
            name: "test_1".to_owned(),
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-10");
}

#[test]
fn accepts_descriptive_name() {
    let input = function_input(
        "tests/api.rs",
        TestFunctionInfo {
            line: 4,
            name: "returns_error_for_unknown_customer".to_owned(),
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.is_empty());
}

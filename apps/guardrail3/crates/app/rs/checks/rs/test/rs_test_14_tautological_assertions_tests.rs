use crate::domain::report::Severity;

use super::super::parse::TestFunctionInfo;
use super::super::test_support::function_input;
use super::check;

#[test]
fn warns_on_literal_vs_literal_assertions() {
    let input = function_input(
        "tests/asserts.rs",
        TestFunctionInfo {
            line: 3,
            name: "asserts_literal_equality".to_owned(),
            tautological_assert_lines: vec![4],
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-14");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].line, Some(4));
}

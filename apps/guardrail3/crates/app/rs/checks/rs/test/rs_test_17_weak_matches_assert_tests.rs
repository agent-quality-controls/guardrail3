use crate::domain::report::Severity;

use super::check;
use super::super::parse::TestFunctionInfo;
use super::super::test_support::function_input;

#[test]
fn warns_on_matches_assert_with_wildcards() {
    let input = function_input(
        "tests/matches.rs",
        TestFunctionInfo {
            line: 7,
            name: "asserts_variant_only".to_owned(),
            weak_matches_lines: vec![9],
            ..Default::default()
        },
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-17");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].line, Some(9));
}

use crate::domain::report::Severity;

use super::check;
use super::super::parse::{analyze, parse_rust_file};
use super::super::test_support::file_input;

#[test]
fn warns_on_bare_ignore() {
    let content = "#[test]\n#[ignore]\nfn slow_test_case() {}";
    let ast = parse_rust_file(content).expect("parse");
    let parsed = analyze(&ast, content);
    let input = file_input("tests/slow.rs", false, true, false, content, parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-TEST-07");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].line, Some(2));
}

#[test]
fn accepts_ignore_reason_forms() {
    for content in [
        "#[test]\n#[ignore = \"network\"]\nfn slow_test_case() {}",
        "#[test]\n#[ignore] // reason: network\nfn slow_test_case() {}",
        "#[test]\n// reason: network\n#[ignore]\nfn slow_test_case() {}",
    ] {
        let ast = parse_rust_file(content).expect("parse");
        let parsed = analyze(&ast, content);
        let input = file_input("tests/slow.rs", false, true, false, content, parsed);
        let mut results = Vec::new();
        check(&input, &mut results);
        assert!(results.is_empty(), "content should be accepted: {content}");
    }
}

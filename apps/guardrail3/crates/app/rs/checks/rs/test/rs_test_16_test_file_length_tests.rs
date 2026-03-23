use crate::domain::report::Severity;

use super::check;
use super::super::parse::ParsedTestFile;
use super::super::test_support::file_input;

#[test]
fn warns_when_test_file_exceeds_threshold() {
    let content = (0..501)
        .map(|_| "assert!(true);")
        .collect::<Vec<_>>()
        .join("\n");
    let input = file_input(
        "tests/huge.rs",
        false,
        true,
        false,
        &content,
        ParsedTestFile::default(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-16");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn ignores_production_files() {
    let input = file_input(
        "src/lib.rs",
        true,
        false,
        false,
        "pub fn f() {}",
        ParsedTestFile::default(),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.is_empty());
}

use crate::domain::report::Severity;

use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;

#[test]
fn errors_when_non_test_file_exceeds_500_effective_lines() {
    let content = "fn x() {}\n".repeat(501);
    let ast = parse_rust_file(&content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content: &content,
        ast: &ast,
        is_test: false,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-09");
    assert_eq!(result.severity, Severity::Error);
    assert_eq!(result.file.as_deref(), Some("src/foo.rs"));
}

#[test]
fn skips_test_files() {
    let content = "fn x() {}\n".repeat(600);
    let ast = parse_rust_file(&content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "tests/foo_tests.rs",
        content: &content,
        ast: &ast,
        is_test: true,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

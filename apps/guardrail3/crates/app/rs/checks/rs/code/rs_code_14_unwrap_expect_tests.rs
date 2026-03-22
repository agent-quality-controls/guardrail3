use crate::domain::report::Severity;

use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;

#[test]
fn warns_on_unwrap_usage() {
    let content = "fn foo() { let _ = some_option().unwrap(); }";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-14");
    assert_eq!(result.severity, Severity::Warn);
}

#[test]
fn skips_allow_scoped_unwrap_usage() {
    let content = r#"
#[allow(clippy::unwrap_used)]
fn foo() { let _ = some_option().unwrap(); }
"#;
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(results.is_empty());
}

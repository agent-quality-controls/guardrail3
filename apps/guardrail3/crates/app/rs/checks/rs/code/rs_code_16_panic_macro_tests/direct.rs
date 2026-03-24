use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn warns_on_panic_macro_in_non_test_code() {
    let content = "fn foo() { panic!(\"boom\"); }";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-16");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "panic! macro");
    assert_eq!(
        results[0].message,
        "`panic!()` macro found: fn foo() { panic!(\"boom\"); }."
    );
}

#[test]
fn warns_on_qualified_panic_macro_in_non_test_code() {
    let content = "fn foo() { core::panic!(\"boom\"); }";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-16");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "panic! macro");
    assert_eq!(
        results[0].message,
        "`panic!()` macro found: fn foo() { core::panic!(\"boom\"); }."
    );
}

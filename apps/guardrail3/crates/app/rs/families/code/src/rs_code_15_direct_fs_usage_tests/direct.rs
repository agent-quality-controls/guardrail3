use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_std_fs_import() {
    let content = "use std::fs;\nfn main() {}";
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
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "direct std::fs import");
    assert_eq!(
        results[0].message,
        "Direct `use std::fs` import found: `use std::fs;`."
    );
}

#[test]
fn errors_on_inline_std_fs_call() {
    let content = "fn main() { let _ = std::fs::read_to_string(\"foo\"); }";
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
    assert_eq!(results[0].id, "RS-CODE-15");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].title, "direct std::fs call");
    assert_eq!(
        results[0].message,
        "Direct `std::fs::*` call found: `fn main() { let _ = std::fs::read_to_string(\"foo\"); }`."
    );
}

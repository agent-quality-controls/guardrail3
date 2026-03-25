use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_plain_include_bypass() {
    let content = "include!(\"../generated.rs\");";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-23");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "include! bypass");
    assert_eq!(
        results[0].message,
        "`include!()` pulls in Rust code outside the scanned file boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn inventories_build_script_include_pattern() {
    let content = "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-23");
    assert_eq!(results[0].severity, Severity::Info);
    assert_eq!(results[0].title, "build-script include! inventory");
    assert_eq!(
        results[0].message,
        "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(results[0].inventory);
}

#[test]
fn warns_on_include_path_traversal() {
    let content = "const BYTES: &[u8] = include_bytes!(\"../fixtures/payload.bin\");";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/lib.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: Some("library"),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-23");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "include path traversal");
    assert_eq!(
        results[0].message,
        "`include_bytes!()` uses a path containing `..`."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

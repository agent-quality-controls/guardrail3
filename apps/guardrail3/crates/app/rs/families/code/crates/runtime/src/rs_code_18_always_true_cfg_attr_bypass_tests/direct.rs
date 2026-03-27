use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_exhaustive_unix_windows_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(any(unix, windows), allow(clippy::unwrap_used))]
fn foo() {}
"#;
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
    assert_eq!(results[0].id, "RS-CODE-18");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].title, "always-true cfg_attr bypass");
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(clippy::unwrap_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
    );
}

#[test]
fn errors_on_empty_all_cfg_attr_allow() {
    let content = r#"
#[cfg_attr(all(), allow(clippy::expect_used))]
fn foo() {}
"#;
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
    assert_eq!(results[0].id, "RS-CODE-18");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].title, "always-true cfg_attr bypass");
    assert_eq!(
        results[0].message,
        "`#[cfg_attr(..., allow(clippy::expect_used))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead."
    );
}

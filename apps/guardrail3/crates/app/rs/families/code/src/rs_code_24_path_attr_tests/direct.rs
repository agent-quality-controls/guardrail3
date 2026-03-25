use guardrail3_domain_report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_path_attr_without_reason() {
    let content = "#[path = \"generated.rs\"]\nmod generated;";
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
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] without reason");
    assert_eq!(
        results[0].message,
        "`#[path = \"generated.rs\"]` changes module resolution and requires `// reason:` on the same line."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn warns_on_path_attr_with_reason() {
    let content = "#[path = \"generated.rs\"] // reason: generated facade shim\nmod generated;";
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
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Warn);
    assert_eq!(results[0].title, "#[path] usage");
    assert_eq!(
        results[0].message,
        "#[path = \"generated.rs\"] reason: generated facade shim"
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_parent_escaping_path_attr() {
    let content = "#[path = \"../generated.rs\"]\nmod generated;";
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
    assert_eq!(results[0].id, "RS-CODE-24");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[path] escapes parent directory");
    assert_eq!(
        results[0].message,
        "`#[path = \"../generated.rs\"]` escapes the standard module boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

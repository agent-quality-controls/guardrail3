use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn errors_on_undocumented_deny_attr() {
    let content = "#[deny(clippy::panic)]\nfn foo() {}";
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
    assert_eq!(results[0].id, "RS-CODE-22");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[deny]/#[forbid] without reason");
    assert_eq!(
        results[0].message,
        "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn inventories_crate_level_forbid_unsafe_code() {
    let content = "#![forbid(unsafe_code)]\nfn foo() {}";
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
    assert_eq!(results[0].id, "RS-CODE-22");
    assert_eq!(results[0].severity, Severity::Info);
    assert_eq!(results[0].title, "forbid(unsafe_code)");
    assert_eq!(
        results[0].message,
        "`forbid(unsafe_code)` strengthens the local safety boundary."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(results[0].inventory);
}

#[test]
fn errors_on_crate_level_deny_warnings() {
    let content = "#![deny(warnings)]\nfn foo() {}";
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
    assert_eq!(results[0].id, "RS-CODE-22");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[deny]/#[forbid] without reason");
    assert_eq!(
        results[0].message,
        "`#[deny(warnings)]` changes local lint policy without documenting why. Add `// reason:` on the same line."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_non_inner_forbid_unsafe_code() {
    let content = "#[forbid(unsafe_code)]\nfn foo() {}";
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
    assert_eq!(results[0].id, "RS-CODE-22");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[deny]/#[forbid] without reason");
    assert_eq!(
        results[0].message,
        "`#[forbid(unsafe_code)]` changes local lint policy without documenting why. Add `// reason:` on the same line."
    );
    assert_eq!(results[0].line, Some(1));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

#[test]
fn errors_on_grouped_deny_lints_without_reason() {
    let content = "#[deny(clippy::panic, clippy::expect_used)]\nfn foo() {}";
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

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.id == "RS-CODE-22"));
    assert!(
        results
            .iter()
            .all(|result| result.severity == Severity::Error)
    );
    assert!(
        results
            .iter()
            .all(|result| result.title == "#[deny]/#[forbid] without reason")
    );
    assert_eq!(
        results
            .iter()
            .map(|result| result.message.as_str())
            .collect::<Vec<_>>(),
        vec![
            "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            "`#[deny(clippy::expect_used)]` changes local lint policy without documenting why. Add `// reason:` on the same line."
        ]
    );
    assert_eq!(
        results.iter().map(|result| result.line).collect::<Vec<_>>(),
        vec![Some(1), Some(1)]
    );
    assert!(
        results
            .iter()
            .all(|result| result.file.as_deref() == Some("src/lib.rs"))
    );
    assert!(results.iter().all(|result| !result.inventory));
}

#[test]
fn errors_on_trait_item_deny_attr() {
    let content = "trait Api {\n    #[deny(clippy::panic)]\n    fn run();\n}";
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
    assert_eq!(results[0].id, "RS-CODE-22");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].title, "#[deny]/#[forbid] without reason");
    assert_eq!(
        results[0].message,
        "`#[deny(clippy::panic)]` changes local lint policy without documenting why. Add `// reason:` on the same line."
    );
    assert_eq!(results[0].line, Some(2));
    assert_eq!(results[0].file.as_deref(), Some("src/lib.rs"));
    assert!(!results[0].inventory);
}

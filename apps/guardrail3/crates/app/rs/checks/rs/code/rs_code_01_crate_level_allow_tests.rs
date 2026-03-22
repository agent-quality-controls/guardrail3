use crate::domain::report::Severity;

use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;

#[test]
fn errors_on_crate_level_allow_in_non_test_file() {
    let content = "#![allow(clippy::unwrap_used)]\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/main.rs",
        content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-CODE-01");
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn info_on_crate_level_allow_in_test_file() {
    let content = "#![allow(clippy::unwrap_used)]\nfn main() {}";
    let ast = parse_rust_file(content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "tests/foo_tests.rs",
        content,
        ast: &ast,
        is_test: true,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Info);
}

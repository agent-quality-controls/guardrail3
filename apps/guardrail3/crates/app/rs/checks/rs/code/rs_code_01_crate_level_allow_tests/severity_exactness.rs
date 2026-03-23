use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn uses_info_severity_for_test_files() {
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
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-01");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(
        result.message,
        "Crate/module-wide allow for `clippy::unwrap_used` is test-file exempt."
    );
}

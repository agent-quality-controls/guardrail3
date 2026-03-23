use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn inventories_unused_crate_dependencies_allow_exactly() {
    let content = "#![allow(unused_crate_dependencies)]\nfn main() {}";
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
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-02");
    assert_eq!(result.severity, Severity::Info);
    assert_eq!(result.file.as_deref(), Some("src/main.rs"));
    assert_eq!(result.line, Some(1));
}

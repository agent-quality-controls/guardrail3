use crate::domain::report::Severity;

use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;

#[test]
fn infos_on_unused_crate_dependencies_allow() {
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
    assert_eq!(results[0].id, "RS-CODE-02");
    assert_eq!(results[0].severity, Severity::Info);
}

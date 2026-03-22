use crate::domain::report::Severity;

use super::super::inputs::RustCodeFileInput;
use super::super::parse::parse_rust_file;
use super::check;

#[test]
fn inventories_conditional_cfg_attr_allow() {
    let content = "#[cfg_attr(test, allow(clippy::unwrap_used))]\nfn foo() {}";
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
    assert_eq!(results[0].id, "RS-CODE-08");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}

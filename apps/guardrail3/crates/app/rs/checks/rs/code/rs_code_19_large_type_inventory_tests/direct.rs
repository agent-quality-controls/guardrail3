use crate::domain::report::Severity;

use super::super::super::inputs::RustCodeFileInput;
use super::super::super::parse::parse_rust_file;
use super::super::check;

#[test]
fn inventories_large_structs() {
    let mut fields = String::new();
    for index in 0..16 {
        fields.push_str(&format!("field_{index}: i32,\n"));
    }
    let content = format!("struct Big {{\n{fields}}}");
    let ast = parse_rust_file(&content).expect("valid rust");
    let input = RustCodeFileInput {
        rel_path: "src/foo.rs",
        content: &content,
        ast: &ast,
        is_test: false,
        profile_name: None,
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CODE-19");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
}

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
    assert_eq!(result.file.as_deref(), Some("src/foo.rs"));
    assert_eq!(result.line, Some(1));
    assert_eq!(result.title, "large type inventory");
    assert_eq!(
        result.message,
        "struct `Big` has 16 fields (inventory threshold 15)."
    );
}

#[test]
fn inventories_large_enums() {
    let mut variants = String::new();
    for index in 0..21 {
        variants.push_str(&format!("Variant{index},\n"));
    }
    let content = format!("enum BigEnum {{\n{variants}}}");
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
    assert_eq!(result.file.as_deref(), Some("src/foo.rs"));
    assert_eq!(result.line, Some(1));
    assert_eq!(result.title, "large type inventory");
    assert_eq!(
        result.message,
        "enum `BigEnum` has 21 items (inventory threshold 20)."
    );
}

#[test]
fn inventories_large_tuple_structs() {
    let mut fields = String::new();
    for index in 0..16 {
        if index > 0 {
            fields.push_str(", ");
        }
        fields.push_str("i32");
    }
    let content = format!("struct BigTuple({fields});");
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
    assert_eq!(result.file.as_deref(), Some("src/foo.rs"));
    assert_eq!(result.line, Some(1));
    assert_eq!(result.title, "large type inventory");
    assert_eq!(
        result.message,
        "struct `BigTuple` has 16 fields (inventory threshold 15)."
    );
}

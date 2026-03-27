use guardrail3_domain_report::Severity;

use guardrail3_app_rs_family_code_assertions::rs_code_19_large_type_inventory::{assert_normalized_len, findings};
use super::super::check_source;

#[test]
fn inventories_large_structs() {
    let mut fields = String::new();
    for index in 0..16 {
        fields.push_str(&format!("field_{index}: i32,\n"));
    }
    let content = format!("struct Big {{\n{fields}}}");
    let raw_results = check_source("src/foo.rs", &content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
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
    let raw_results = check_source("src/foo.rs", &content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
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
    let raw_results = check_source("src/foo.rs", &content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
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

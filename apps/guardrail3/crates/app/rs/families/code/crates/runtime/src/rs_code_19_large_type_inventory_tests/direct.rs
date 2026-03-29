use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_19_large_type_inventory::{
    assert_findings, RuleFinding,
};

#[test]
fn inventories_large_structs() {
    let mut fields = String::new();
    for index in 0..16 {
        fields.push_str(&format!("field_{index}: i32,\n"));
    }
    let content = format!("struct Big {{\n{fields}}}");
    let results = check_source("src/foo.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Info,
            title: "large type inventory",
            message: "struct `Big` has 16 fields (inventory threshold 15).",
            file: Some("src/foo.rs"),
            line: Some(1),
            inventory: true,
        }],
    );
}

#[test]
fn inventories_large_enums() {
    let mut variants = String::new();
    for index in 0..21 {
        variants.push_str(&format!("Variant{index},\n"));
    }
    let content = format!("enum BigEnum {{\n{variants}}}");
    let results = check_source("src/foo.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Info,
            title: "large type inventory",
            message: "enum `BigEnum` has 21 items (inventory threshold 20).",
            file: Some("src/foo.rs"),
            line: Some(1),
            inventory: true,
        }],
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
    let results = check_source("src/foo.rs", &content, false);

    assert_findings(
        &results,
        &[RuleFinding {
            severity: guardrail3_domain_report::Severity::Info,
            title: "large type inventory",
            message: "struct `BigTuple` has 16 fields (inventory threshold 15).",
            file: Some("src/foo.rs"),
            line: Some(1),
            inventory: true,
        }],
    );
}

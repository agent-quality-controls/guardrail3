use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_19_large_type_inventory::{
    RuleFinding, assert_findings,
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
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "large type inventory",
            "struct `Big` has 16 fields (inventory threshold 15).",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
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
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "large type inventory",
            "enum `BigEnum` has 21 items (inventory threshold 20).",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
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
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Warn,
            "large type inventory",
            "struct `BigTuple` has 16 fields (inventory threshold 15).",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

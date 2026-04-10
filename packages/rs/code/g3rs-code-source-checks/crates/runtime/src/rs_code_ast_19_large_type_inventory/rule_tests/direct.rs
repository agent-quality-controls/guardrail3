use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_19_large_type_inventory::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn inventories_large_struct() {
    let fields = (0..16)
        .map(|i| format!("f{i}: u8"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("struct Big {{ {fields} }}\n");

    let results = check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("large type inventory"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("struct `Big` has 16 fields (inventory threshold 15)."),
            line: Some(1),
        }],
    );
}

#[test]
fn inventories_large_enum() {
    let variants = (0..21)
        .map(|i| format!("V{i}"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("enum Big {{ {variants} }}\n");

    let results = check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("large type inventory"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("enum `Big` has 21 items (inventory threshold 20)."),
            line: Some(1),
        }],
    );
}

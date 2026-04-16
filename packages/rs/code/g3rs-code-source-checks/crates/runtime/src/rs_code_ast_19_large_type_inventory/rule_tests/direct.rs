use g3rs_code_source_checks_assertions::rs_code_ast_19_large_type_inventory::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn inventories_large_struct() {
    let fields = (0..16)
        .map(|i| format!("f{i}: u8"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("struct Big {{ {fields} }}\n");

    let results = super::super::check_source("src/lib.rs", &content, false);

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

    let results = super::super::check_source("src/lib.rs", &content, false);

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

#[test]
fn skips_large_struct_with_exact_waiver() {
    let fields = (0..16)
        .map(|i| format!("f{i}: u8"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("pub struct CargoConfigToml {{ {fields} }}\n");

    let results = super::super::check_source_with_waivers(
        "src/lib.rs",
        &content,
        false,
        &[("RS-CODE-SOURCE-19", "src/lib.rs", "struct:CargoConfigToml", "schema mirror")],
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn non_matching_waiver_does_not_suppress_large_struct() {
    let fields = (0..16)
        .map(|i| format!("f{i}: u8"))
        .collect::<Vec<_>>()
        .join(", ");
    let content = format!("pub struct CargoConfigToml {{ {fields} }}\n");

    let results = super::super::check_source_with_waivers(
        "src/lib.rs",
        &content,
        false,
        &[("RS-CODE-SOURCE-19", "src/lib.rs", "struct:Different", "wrong selector")],
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("large type inventory"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("struct `CargoConfigToml` has 16 fields (inventory threshold 15)."),
            line: Some(1),
        }],
    );
}

use g3rs_code_source_checks_assertions::rs_code_ast_04_item_level_allow_with_reason::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn inventories_allow_with_useful_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[allow(dead_code)] // reason: proc macro entrypoint\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("item-level allow with reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("#[allow(dead_code)] reason: proc macro entrypoint"),
            line: Some(1),
        }],
    );
}

#[test]
fn inventories_expect_with_useful_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[expect(dead_code)] // reason: generated bridge shim\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("item-level expect with reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("#[expect(dead_code)] reason: generated bridge shim"),
            line: Some(1),
        }],
    );
}

#[test]
fn matching_waiver_suppresses_allow_inventory() {
    let results = super::super::check_source_with_waivers(
        "src/lib.rs",
        "#[allow(dead_code)] // reason: proc macro entrypoint\nfn probe() {}\n",
        false,
        &[(
            "RS-CODE-SOURCE-04",
            "src/lib.rs",
            "lint:dead_code",
            "inventory accepted for generated boundary",
        )],
    );

    assert_rule_results(&results, &[]);
}

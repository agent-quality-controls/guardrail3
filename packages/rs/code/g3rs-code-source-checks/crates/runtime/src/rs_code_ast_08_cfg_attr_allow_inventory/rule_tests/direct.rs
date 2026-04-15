use g3rs_code_source_checks_assertions::rs_code_ast_08_cfg_attr_allow_inventory::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn inventories_conditional_cfg_attr_allow() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[cfg_attr(feature = \"cli\", allow(dead_code))]\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("conditional cfg_attr allow"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("Conditional cfg_attr allow for `dead_code`."),
            line: Some(1),
        }],
    );
}

#[test]
fn inventories_conditional_cfg_attr_expect() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[cfg_attr(feature = \"cli\", expect(dead_code))]\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("conditional cfg_attr expect"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("Conditional cfg_attr expect for `dead_code`."),
            line: Some(1),
        }],
    );
}

use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_04_item_level_allow_with_reason::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn inventories_allow_with_useful_reason() {
    let results = check_source(
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
    let results = check_source(
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

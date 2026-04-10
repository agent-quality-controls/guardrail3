use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_02_unused_crate_dependencies_allow::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn inventories_crate_level_unused_crate_dependencies_allow() {
    let results = check_source(
        "src/lib.rs",
        "#![allow(unused_crate_dependencies)]\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("unused_crate_dependencies exemption"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("unused_crate_dependencies is an approved universal exemption."),
            line: Some(1),
        }],
    );
}

#[test]
fn inventories_inline_module_unused_crate_dependencies_allow() {
    let results = check_source(
        "src/lib.rs",
        "mod nested {\n    #![allow(unused_crate_dependencies)]\n    fn probe() {}\n}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("unused_crate_dependencies exemption"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("unused_crate_dependencies is an approved universal exemption."),
            line: Some(2),
        }],
    );
}

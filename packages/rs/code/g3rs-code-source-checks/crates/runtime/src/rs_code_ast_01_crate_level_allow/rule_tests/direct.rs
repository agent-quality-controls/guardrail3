use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_01_crate_level_allow::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_crate_level_allow_in_prod_code() {
    let results = check_source("src/lib.rs", "#![allow(dead_code)]\nfn probe() {}\n", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("crate-level allow"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Crate/module-wide `allow(dead_code)` suppresses the lint too broadly. Use item-level `#[allow(dead_code)]` with a `// reason:` comment instead.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn reports_inline_module_allow() {
    let results = check_source(
        "src/lib.rs",
        "mod nested {\n    #![allow(dead_code)]\n    fn probe() {}\n}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("module-level allow in nested"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "Crate/module-wide `allow(dead_code)` suppresses the lint too broadly. Use item-level `#[allow(dead_code)]` with a `// reason:` comment instead.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn downgrades_test_files_to_inventory_info() {
    let results = check_source(
        "tests/smoke.rs",
        "#![allow(dead_code)]\nfn probe() {}\n",
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("crate-level allow"),
            file: Some("tests/smoke.rs"),
            inventory: Some(false),
            message: Some("Crate/module-wide allow for `dead_code` is test-file exempt."),
            line: Some(1),
        }],
    );
}

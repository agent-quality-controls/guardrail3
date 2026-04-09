use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_22_deny_forbid_without_reason::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_deny_without_reason() {
    let results = check_source("src/lib.rs", "#[deny(dead_code)]\nfn probe() {}\n", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("#[deny]/#[forbid] without reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[deny(dead_code)]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_weak_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[forbid(unsafe_code)] // reason: temp\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("#[deny]/#[forbid] reason too weak"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[forbid(unsafe_code)]` reason must be specific and at least two words. Weak reason `temp` found.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn inventories_crate_level_forbid_unsafe_code() {
    let results = check_source(
        "src/lib.rs",
        "#![forbid(unsafe_code)]\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("forbid(unsafe_code)"),
            file: Some("src/lib.rs"),
            inventory: Some(true),
            message: Some("`forbid(unsafe_code)` strengthens the local safety boundary."),
            line: Some(1),
        }],
    );
}

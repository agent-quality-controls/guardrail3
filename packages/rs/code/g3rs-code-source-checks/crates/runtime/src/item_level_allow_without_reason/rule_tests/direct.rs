use g3rs_code_source_checks_assertions::item_level_allow_without_reason::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_item_level_allow_without_reason() {
    let results =
        super::super::check_source("src/lib.rs", "#[allow(dead_code)]\nfn probe() {}\n", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("item-level allow without reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("`#[allow(dead_code)]` requires `// reason:` on the same line."),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_item_level_expect_without_reason() {
    let results =
        super::super::check_source("src/lib.rs", "#[expect(dead_code)]\nfn probe() {}\n", false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("item-level expect without reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("`#[expect(dead_code)]` requires `// reason:` on the same line."),
            line: Some(1),
        }],
    );
}

#[test]
fn errors_on_weak_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[allow(dead_code)] // reason: temp\nfn probe() {}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("item-level allow reason too weak"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[allow(dead_code)]` reason must be specific and at least two words. Weak reason `temp` found.",
            ),
            line: Some(1),
        }],
    );
}

use g3rs_code_source_checks_assertions::garde_skip_with_comment::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_when_comment_lacks_reason_key() {
    let results = super::super::check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)] // upstream validates this field\n    token: String,\n}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("garde(skip) comment missing reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[garde(skip)]` on non-exempt field `token: String` needs `// reason:`.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn errors_on_weak_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)] // reason: temp\n    token: String,\n}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("garde(skip) reason too weak"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[garde(skip)]` on non-exempt field `token: String` reason must be specific and at least two words. Weak reason `temp` found.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn inventories_useful_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)] // reason: validated upstream boundary\n    token: String,\n}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("garde(skip) with reason"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[garde(skip)]` on non-exempt field `token: String` reason: validated upstream boundary",
            ),
            line: Some(2),
        }],
    );
}

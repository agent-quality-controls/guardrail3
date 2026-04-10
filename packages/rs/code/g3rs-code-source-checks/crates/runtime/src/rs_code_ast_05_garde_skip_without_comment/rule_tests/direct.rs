use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_05_garde_skip_without_comment::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_non_exempt_garde_skip_without_comment() {
    let results = check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)]\n    token: String,\n}\n",
        false,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("garde(skip) without comment"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "`#[garde(skip)]` on non-exempt field `token: String` requires documentation. Add a `// reason:` comment explaining why validation is skipped.",
            ),
            line: Some(2),
        }],
    );
}

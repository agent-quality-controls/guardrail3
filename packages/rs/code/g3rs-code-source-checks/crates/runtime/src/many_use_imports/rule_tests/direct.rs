use g3rs_code_source_checks_assertions::many_use_imports::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn warns_in_threshold_band() {
    let content =
        "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15};\nfn probe() {}\n";

    let results = super::super::check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("many use imports"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("16 top-level use imports (warn at 16, max 20)."),
            line: None,
        }],
    );
}

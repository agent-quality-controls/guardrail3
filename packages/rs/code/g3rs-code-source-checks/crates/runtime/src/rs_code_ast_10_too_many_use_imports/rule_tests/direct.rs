use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_10_too_many_use_imports::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_when_use_import_count_exceeds_cap() {
    let content = "use a::{b0,b1,b2,b3,b4,b5,b6,b7,b8,b9,b10,b11,b12,b13,b14,b15,b16,b17,b18,b19,b20};\nfn probe() {}\n";

    let results = check_source("src/lib.rs", content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("too many use imports"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "21 top-level use imports (max 20). Reduce imports by consolidating or splitting the file.",
            ),
            line: None,
        }],
    );
}

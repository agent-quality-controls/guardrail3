#![allow(
    clippy::format_collect,
    clippy::format_in_format_args,
    reason = "test fixtures synthesize large source bodies via format! over an iterator; the simpler iterator-collect form is intentional"
)]

use g3rs_code_source_checks_assertions::too_many_effective_code_lines::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_when_effective_code_lines_exceed_cap() {
    let content = (0..501)
        .map(|i| format!("fn f{i}() {{}}\n"))
        .collect::<String>();

    let results = super::super::check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("too many effective code lines"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some(
                "501 effective code-bearing lines (max 500). Split this file into smaller modules.",
            ),
            line: None,
        }],
    );
}

#[test]
fn matching_waiver_suppresses_effective_code_line_error() {
    let content = (0..501)
        .map(|i| format!("fn f{i}() {{}}\n"))
        .collect::<String>();

    let results = super::super::check_source_with_waivers(
        "src/lib.rs",
        &content,
        false,
        &[(
            "g3rs-code/too-many-effective-code-lines",
            "src/lib.rs",
            "effective-code-lines",
            "state machine split tracked separately",
        )],
    );

    assert_rule_results(&results, &[]);
}

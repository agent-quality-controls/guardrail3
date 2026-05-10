#![allow(
    clippy::format_collect,
    clippy::format_in_format_args,
    reason = "test fixtures synthesize large source bodies via format! over an iterator; the simpler iterator-collect form is intentional"
)]

use g3rs_code_source_checks_assertions::large_trait_surface::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn errors_on_trait_with_thirteen_methods() {
    let methods = (0..13)
        .map(|index| format!("    fn m{index}(&self);\n"))
        .collect::<String>();
    let content = format!("pub trait Service {{\n{methods}}}");
    let results = super::super::check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("large trait surface"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("Trait `Service` has 13 methods (warn above 8, error above 12)."),
            line: Some(1),
        }],
    );
}

#[test]
fn warns_on_trait_with_nine_methods() {
    let methods = (0..9)
        .map(|index| format!("    fn m{index}(&self);\n"))
        .collect::<String>();
    let content = format!("pub trait Service {{\n{methods}}}");
    let results = super::super::check_source("src/lib.rs", &content, false);

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("large trait surface"),
            file: Some("src/lib.rs"),
            inventory: Some(false),
            message: Some("Trait `Service` has 9 methods (warn above 8, error above 12)."),
            line: Some(1),
        }],
    );
}

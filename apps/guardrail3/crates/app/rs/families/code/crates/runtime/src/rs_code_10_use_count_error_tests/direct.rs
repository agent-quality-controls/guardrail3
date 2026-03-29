use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_10_use_count_error::{Severity, 
    RuleFinding, assert_findings,
};

#[test]
fn errors_above_twenty_top_level_uses() {
    let mut lines: Vec<String> = (0..21)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(
        &check_source("src/foo.rs", &content, false),
        &[RuleFinding {
            severity: Severity::Error,
            title: "too many use statements",
            message: "21 top-level use statements (max 20).",
            file: Some("src/foo.rs"),
            line: None,
            inventory: false,
        }],
    );
}

#[test]
fn skips_exactly_twenty_top_level_uses() {
    let mut lines: Vec<String> = (0..20)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(&check_source("src/foo.rs", &content, false), &[]);
}

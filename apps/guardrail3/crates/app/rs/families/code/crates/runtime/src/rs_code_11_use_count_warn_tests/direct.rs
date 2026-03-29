use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_11_use_count_warn::{
    RuleFinding, Severity, assert_findings,
};

#[test]
fn warns_between_sixteen_and_twenty_top_level_uses() {
    let mut lines: Vec<String> = (0..16)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(
        &check_source("src/foo.rs", &content, false),
        &[RuleFinding {
            severity: Severity::Warn,
            title: "many use statements",
            message: "16 top-level use statements (warn at 16, max 20).",
            file: Some("src/foo.rs"),
            line: None,
            inventory: false,
        }],
    );
}

#[test]
fn skips_below_warn_band_in_non_test_file() {
    let mut lines: Vec<String> = (0..15)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(&check_source("src/foo.rs", &content, false), &[]);
}

#[test]
fn skips_above_warn_band_in_non_test_file() {
    let mut lines: Vec<String> = (0..21)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(&check_source("src/foo.rs", &content, false), &[]);
}

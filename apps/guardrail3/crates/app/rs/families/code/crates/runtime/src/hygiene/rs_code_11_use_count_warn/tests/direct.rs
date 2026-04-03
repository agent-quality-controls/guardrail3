use super::helpers::check_source;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_11_use_count_warn::{
    RuleFinding, Severity, assert_findings,
};

#[test]
fn warns_between_sixteen_and_twenty_top_level_use_imports() {
    let mut lines: Vec<String> = (0..16)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(
        &check_source("src/foo.rs", &content, false),
        &[RuleFinding::new(
            Severity::Warn,
            "many use imports",
            "16 top-level use imports (warn at 16, max 20).",
            Some("src/foo.rs"),
            None,
            false,
        )],
    );
}

#[test]
fn warns_grouped_imports_by_leaf_count() {
    let content = "use crate::{a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p};\nfn main() {}\n";
    assert_findings(
        &check_source("src/foo.rs", content, false),
        &[RuleFinding::new(
            Severity::Warn,
            "many use imports",
            "16 top-level use imports (warn at 16, max 20).",
            Some("src/foo.rs"),
            None,
            false,
        )],
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

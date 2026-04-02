use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::hygiene::rs_code_10_use_count_error::{
    RuleFinding, Severity, assert_findings,
};

#[test]
fn errors_above_twenty_top_level_use_imports() {
    let mut lines: Vec<String> = (0..21)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(
        &check_source("src/foo.rs", &content, false),
        &[RuleFinding::new(
            Severity::Error,
            "too many use imports",
            "21 top-level use imports (max 20).",
            Some("src/foo.rs"),
            None,
            false,
        )],
    );
}

#[test]
fn errors_grouped_imports_by_leaf_count() {
    let content = "use crate::{a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u};\nfn main() {}\n";
    assert_findings(
        &check_source("src/foo.rs", content, false),
        &[RuleFinding::new(
            Severity::Error,
            "too many use imports",
            "21 top-level use imports (max 20).",
            Some("src/foo.rs"),
            None,
            false,
        )],
    );
}

#[test]
fn skips_exactly_twenty_top_level_use_imports() {
    let mut lines: Vec<String> = (0..20)
        .map(|index| format!("use crate::mod_{index};"))
        .collect();
    lines.push("fn main() {}".to_owned());
    let content = lines.join("\n");
    assert_findings(&check_source("src/foo.rs", &content, false), &[]);
}

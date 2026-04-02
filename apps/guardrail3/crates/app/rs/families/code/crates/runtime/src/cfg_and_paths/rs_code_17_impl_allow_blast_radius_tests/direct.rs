use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::cfg_and_paths::rs_code_17_impl_allow_blast_radius::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_impl_allow_covering_more_than_three_methods() {
    let content = r#"
struct Foo;

#[allow(clippy::too_many_lines)]
impl Foo {
    fn a(&self) {}
    fn b(&self) {}
    fn c(&self) {}
    fn d(&self) {}
}
"#;
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "blanket impl-level allow",
            "`#[allow(clippy::too_many_lines)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead.",
            Some("src/foo.rs"),
            Some(4),
            false,
        )],
    );
}

#[test]
fn errors_on_impl_expect_covering_more_than_three_methods() {
    let content = r#"
struct Foo;

#[expect(clippy::too_many_lines)]
impl Foo {
    fn a(&self) {}
    fn b(&self) {}
    fn c(&self) {}
    fn d(&self) {}
}
"#;
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "blanket impl-level expect",
            "`#[expect(clippy::too_many_lines)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead.",
            Some("src/foo.rs"),
            Some(4),
            false,
        )],
    );
}

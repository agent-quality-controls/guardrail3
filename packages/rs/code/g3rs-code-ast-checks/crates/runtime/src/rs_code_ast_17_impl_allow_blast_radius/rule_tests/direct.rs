use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_17_impl_allow_blast_radius::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
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

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("blanket impl-level allow"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "`#[allow(clippy::too_many_lines)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead.",
            ),
            line: Some(4),
        }],
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

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("blanket impl-level expect"),
            file: Some("src/foo.rs"),
            inventory: Some(false),
            message: Some(
                "`#[expect(clippy::too_many_lines)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead.",
            ),
            line: Some(4),
        }],
    );
}

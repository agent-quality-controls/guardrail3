use guardrail3_domain_report::Severity;

use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_17_impl_allow_blast_radius::{
    assert_normalized_len, findings,
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
    let raw_results = check_source("src/foo.rs", content, false);
    let results = findings(&raw_results);

    assert_normalized_len(&results, 1);
    assert_eq!(results[0].id, "RS-CODE-17");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some("src/foo.rs"));
    assert_eq!(results[0].line, Some(4));
    assert_eq!(results[0].title, "blanket impl-level allow");
    assert_eq!(
        results[0].message,
        "`#[allow(clippy::too_many_lines)]` covers an impl block with 4 methods. Apply lint suppressions to individual methods instead."
    );
}

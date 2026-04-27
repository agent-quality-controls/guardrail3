use g3rs_code_source_checks_assertions::impl_allow_blast_radius::rule::assert_rule_results;

#[test]
fn skips_threshold_and_method_level_cases() {
    let threshold = r#"
struct Foo;
#[allow(clippy::too_many_lines)]
impl Foo {
    fn a(&self) {}
    fn b(&self) {}
    fn c(&self) {}
}
"#;
    let method_level = r#"
struct Foo;
impl Foo {
    #[allow(clippy::too_many_lines)]
    fn a(&self) {}
    fn b(&self) {}
    fn c(&self) {}
    fn d(&self) {}
}
"#;

    assert_rule_results(
        &super::super::check_source("src/foo.rs", threshold, false),
        &[],
    );
    assert_rule_results(
        &super::super::check_source("src/foo.rs", method_level, false),
        &[],
    );
}

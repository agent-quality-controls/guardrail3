use g3rs_code_source_checks_assertions::include_bypass::rule::assert_rule_results;

#[test]
fn ignores_non_traversing_include_str() {
    let content = "const LOCAL_TEMPLATE: &str = include_str!(\"embedded_schema.json\");";
    assert_rule_results(
        &super::super::check_source("src/lib.rs", content, false),
        &[],
    );
}

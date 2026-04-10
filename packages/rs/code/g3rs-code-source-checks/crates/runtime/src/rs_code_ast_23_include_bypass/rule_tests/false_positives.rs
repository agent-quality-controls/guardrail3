use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_23_include_bypass::assert_rule_results;

#[test]
fn ignores_non_traversing_include_str() {
    let content = "const LOCAL_TEMPLATE: &str = include_str!(\"embedded_schema.json\");";
    assert_rule_results(&check_source("src/lib.rs", content, false), &[]);
}

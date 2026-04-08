use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_18_always_true_cfg_attr_bypass::assert_rule_results;

#[test]
fn skips_genuinely_conditional_cfg_attr_forms() {
    let content = r#"
#[cfg_attr(test, allow(clippy::unwrap_used))]
fn test_only_probe() {}

#[cfg_attr(feature = "serde", allow(clippy::expect_used))]
fn feature_probe() {}
"#;

    assert_rule_results(&check_source("src/foo.rs", content, false), &[]);
}

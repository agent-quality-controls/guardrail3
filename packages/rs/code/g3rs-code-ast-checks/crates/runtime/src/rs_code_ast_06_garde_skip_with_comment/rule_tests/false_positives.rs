use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_06_garde_skip_with_comment::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn skips_exempt_garde_skip_types() {
    let results = check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)] // reason: primitive passthrough\n    enabled: bool,\n}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

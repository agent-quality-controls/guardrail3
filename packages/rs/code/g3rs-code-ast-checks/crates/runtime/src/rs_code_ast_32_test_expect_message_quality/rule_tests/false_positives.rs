use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_32_test_expect_message_quality::assert_rule_results;

#[test]
fn skips_useful_and_non_test_expect_calls() {
    let useful = "fn probe() { let _ = Some(1).expect(\"backend fixture should parse\"); }";
    let non_test = "fn probe() { let _ = Some(1).expect(\"ok\"); }";
    let concat_useful =
        "fn probe() { let _ = Some(1).expect(concat!(\"backend fixture\", \" should parse\")); }";
    let helper_expect =
        "mod helpers { pub fn expect(_message: &str) {} }\nfn probe() { helpers::expect(\"ok\"); }";

    assert_rule_results(&check_source("tests/probe.rs", useful, true), &[]);
    assert_rule_results(&check_source("src/lib.rs", non_test, false), &[]);
    assert_rule_results(&check_source("tests/probe.rs", concat_useful, true), &[]);
    assert_rule_results(&check_source("tests/probe.rs", helper_expect, true), &[]);
}

use guardrail3_app_rs_family_code_assertions::rs_code_32_test_expect_message_quality::{
    assert_no_hits,
};
use super::super::check_source;

#[test]
fn skips_useful_test_expect_message() {
    let content = "fn probe() { let _ = Some(1).expect(\"backend fixture should parse\"); }";
    let results = check_source("tests/probe.rs", content, true);

    assert_no_hits(&results);
}

#[test]
fn skips_expect_in_non_test_file() {
    let content = "fn probe() { let _ = Some(1).expect(\"ok\"); }";
    let results = check_source("src/lib.rs", content, false);

    assert_no_hits(&results);
}

#[test]
fn accepts_concat_of_string_literals() {
    let content = "fn probe() { let _ = Some(1).expect(concat!(\"backend fixture\", \" should parse\")); }";
    let results = check_source("tests/probe.rs", content, true);

    assert_no_hits(&results);
}

#[test]
fn ignores_non_method_helper_named_expect() {
    let content = "mod helpers { pub fn expect(_message: &str) {} }\nfn probe() { helpers::expect(\"ok\"); }";
    let results = check_source("tests/probe.rs", content, true);

    assert_no_hits(&results);
}

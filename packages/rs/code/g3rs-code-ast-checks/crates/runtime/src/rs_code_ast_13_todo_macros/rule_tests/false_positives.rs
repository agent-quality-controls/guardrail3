use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_13_todo_macros::assert_rule_results;

#[test]
fn skips_unreachable_macro_in_test_owned_files() {
    let results = check_source("tests/foo.rs", "fn foo() { unreachable!(); }", true);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_comment_and_string_macro_text() {
    let content = "fn foo() {\n    let _ = \"todo! in string\";\n    // unimplemented! in comment\n    maybe_todo();\n}\n";
    let results = check_source("src/foo.rs", content, false);
    assert_rule_results(&results, &[]);
}

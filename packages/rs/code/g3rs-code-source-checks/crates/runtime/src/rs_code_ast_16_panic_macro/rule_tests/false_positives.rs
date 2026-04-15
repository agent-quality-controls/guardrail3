use g3rs_code_source_checks_assertions::rs_code_ast_16_panic_macro::rule::assert_rule_results;

#[test]
fn skips_panic_macro_in_test_owned_files() {
    let content = "fn probe() { panic!(\"boom\"); }\n";
    let results = super::super::check_source("tests/panic_macro.rs", content, true);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_test_panic_inside_non_test_file() {
    let content = "#[cfg(test)]\nmod cfg_probe {\n    pub fn test_only_probe() { panic!(\"prod-file cfg\"); }\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_comment_and_string_panic_text() {
    let content = "fn probe() {\n    let _ = \"panic! in string\";\n    // panic! in comment\n    todo!();\n    unimplemented!();\n    unreachable!();\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

use g3rs_code_source_checks_assertions::rs_code_ast_20_extern_allow::rule::assert_rule_results;

#[test]
fn ignores_non_covering_allow_attributes() {
    let content = r#"
#[allow(dead_code)]
fn local_probe() {}

#[allow(improper_ctypes)]
unsafe extern "C" fn extern_probe() {}

extern "C" {
    #[allow(improper_ctypes)]
    fn foreign_probe(code: i32);
}
"#;

    assert_rule_results(
        &super::super::check_source("src/ffi.rs", content, false),
        &[],
    );
}

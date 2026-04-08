use super::helpers::check_source;
use g3rs_code_ast_checks_assertions::rs_code_15_direct_fs_usage::assert_rule_results;

#[test]
fn skips_test_owned_files() {
    let content = "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n";
    let results = check_source("tests/fs_usage.rs", content, true);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_explicit_fs_boundary_module() {
    let content = "use std::fs;\npub fn allowed_probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n";
    let results = check_source("src/fs.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_test_scoped_fs_usage_in_non_test_file() {
    let content = "#[cfg(test)]\nuse std::fs;\n#[cfg(test)]\nmod cfg_probe {\n    pub fn run() { let _ = std::fs::read_to_string(\"fixture\"); }\n}\n";
    let results = check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_comment_and_string_fs_text() {
    let content = "fn text_probe() {\n    let _ = \"use std::fs\";\n    let _ = \"std::fs::read_to_string\";\n    // use std::fs\n    // std::fs::read_to_string(\"fixture\")\n}\n";
    let results = check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

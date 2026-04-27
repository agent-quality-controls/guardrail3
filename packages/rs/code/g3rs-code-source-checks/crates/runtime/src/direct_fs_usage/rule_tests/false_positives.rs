use g3rs_code_source_checks_assertions::direct_fs_usage::rule::assert_rule_results;

#[test]
fn skips_test_owned_files() {
    let content = "use std::fs;\nfn probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n";
    let results = super::super::check_source("tests/fs_usage.rs", content, true);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_explicit_fs_boundary_module() {
    let content =
        "use std::fs;\npub fn allowed_probe() { let _ = std::fs::read_to_string(\"fixture\"); }\n";
    let results = super::super::check_source("src/fs.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_test_scoped_fs_usage_in_non_test_file() {
    let content = "#[cfg(test)]\nuse std::fs;\n#[cfg(test)]\nmod cfg_probe {\n    pub fn run() { let _ = std::fs::read_to_string(\"fixture\"); }\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_comment_and_string_fs_text() {
    let content = "fn text_probe() {\n    let _ = \"use std::fs\";\n    let _ = \"std::fs::read_to_string\";\n    // use std::fs\n    // std::fs::read_to_string(\"fixture\")\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_std_alias_from_sibling_module() {
    let content = "mod first {\n    use std as s;\n}\nmod second {\n    pub fn probe() { let _ = s::fs::read_to_string(\"fixture\"); }\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_std_alias_from_inner_block_scope() {
    let content = "fn probe() {\n    {\n        use std as s;\n    }\n    let _ = s::fs::read_to_string(\"fixture\");\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_std_alias_import_from_inner_block_scope() {
    let content = "fn probe() {\n    {\n        use std as s;\n    }\n    use s::fs;\n}\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_top_level_cfg_test_std_alias_for_call() {
    let content =
        "#[cfg(test)]\nuse std as s;\nfn probe() { let _ = s::fs::read_to_string(\"fixture\"); }\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_top_level_cfg_test_std_alias_for_import() {
    let content = "#[cfg(test)]\nuse std as s;\nuse s::fs;\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_all_test_scoped_fs_usage_in_non_test_file() {
    let content = "#[cfg(all(test, unix))]\nuse std::fs;\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_grouped_std_self_alias_without_fs() {
    let content = "use std::{self as s, io};\nfn probe() { let _ = s::io::ErrorKind::Other; }\n";
    let results = super::super::check_source("src/lib.rs", content, false);
    assert_rule_results(&results, &[]);
}

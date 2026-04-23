use g3rs_code_source_checks_assertions::rs_code_ast_21_fs_glob_import::rule::assert_rule_results;

#[test]
fn skips_tests_and_non_glob_cases() {
    let test_content = "#[cfg(test)]\nuse std::fs::*;\nfn main() {}";
    let non_glob = "use std::fs::File;\nuse std::fs::{self};\nfn main() {}";
    let test_module =
        "#[cfg(test)]\nmod tests {\n    use std::fs::*;\n    fn probe() {}\n}\nfn main() {}";

    assert_rule_results(
        &super::super::check_source("src/foo.rs", test_content, false),
        &[],
    );
    assert_rule_results(
        &super::super::check_source("src/foo.rs", non_glob, false),
        &[],
    );
    assert_rule_results(
        &super::super::check_source("src/foo.rs", test_module, false),
        &[],
    );
    assert_rule_results(
        &super::super::check_source("tests/foo.rs", "use std::fs::*;", true),
        &[],
    );
    assert_rule_results(
        &super::super::check_source("src/fs.rs", "use std::fs::*;", false),
        &[],
    );
}

#[test]
fn skips_std_alias_from_sibling_module() {
    let content = "mod first {\n    use std as s;\n}\nmod second {\n    use s::fs::*;\n}\n";
    let results = super::super::check_source("src/foo.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_top_level_cfg_test_std_alias_for_glob_import() {
    let content = "#[cfg(test)]\nuse std as s;\nuse s::fs::*;\n";
    let results = super::super::check_source("src/foo.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_cfg_all_test_std_fs_glob_import() {
    let content = "#[cfg(all(test, unix))]\nuse std::fs::*;\n";
    let results = super::super::check_source("src/foo.rs", content, false);
    assert_rule_results(&results, &[]);
}

#[test]
fn skips_grouped_non_fs_std_glob_imports() {
    let direct = "use std::{*};\nfn main() {}";
    let aliased = "use std as s;\nuse s::{*};\nfn main() {}";

    assert_rule_results(&super::super::check_source("src/foo.rs", direct, false), &[]);
    assert_rule_results(&super::super::check_source("src/foo.rs", aliased, false), &[]);
}

use super::helpers::check_source;
use g3rs_code_source_checks_assertions::rs_code_21_fs_glob_import::assert_rule_results;

#[test]
fn skips_tests_and_non_glob_cases() {
    let test_content = "#[cfg(test)]\nuse std::fs::*;\nfn main() {}";
    let non_glob = "use std::fs::File;\nuse std::fs::{self};\nfn main() {}";
    let test_module =
        "#[cfg(test)]\nmod tests {\n    use std::fs::*;\n    fn probe() {}\n}\nfn main() {}";

    assert_rule_results(&check_source("src/foo.rs", test_content, false), &[]);
    assert_rule_results(&check_source("src/foo.rs", non_glob, false), &[]);
    assert_rule_results(&check_source("src/foo.rs", test_module, false), &[]);
    assert_rule_results(&check_source("tests/foo.rs", "use std::fs::*;", true), &[]);
    assert_rule_results(&check_source("src/fs.rs", "use std::fs::*;", false), &[]);
}

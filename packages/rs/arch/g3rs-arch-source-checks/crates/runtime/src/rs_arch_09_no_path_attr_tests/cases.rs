use g3rs_arch_source_checks_assertions::rs_arch_09_no_path_attr as assertions;

use super::helpers::{run_rule, source_file};

#[test]
fn cfg_test_sidecar_path_is_allowed() {
    let results = run_rule(&source_file(
        "crate_a/src/run.rs",
        "#[cfg(test)]\n#[path = \"run_tests/mod.rs\"]\nmod run_tests;\n",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn generic_tests_name_with_sidecar_path_still_fires() {
    let results = run_rule(&source_file(
        "crate_a/src/rule.rs",
        "#[cfg(test)]\n#[path = \"rule_tests/mod.rs\"]\nmod tests;\n",
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/rule.rs");
}

#[test]
fn mismatched_sidecar_name_still_fires() {
    let results = run_rule(&source_file(
        "crate_a/src/run.rs",
        "#[cfg(test)]\n#[path = \"run_tests/mod.rs\"]\nmod helper_tests;\n",
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/run.rs");
}

#[test]
fn non_test_path_attr_still_fires() {
    let results = run_rule(&source_file(
        "crate_a/src/run.rs",
        "#[path = \"run_tests/mod.rs\"]\nmod run_tests;\n",
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/run.rs");
}

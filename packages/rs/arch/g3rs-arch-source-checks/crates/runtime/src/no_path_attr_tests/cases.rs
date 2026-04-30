use g3rs_arch_source_checks_assertions::no_path_attr as assertions;

use super::helpers::{run_rule, site};

#[test]
fn cfg_test_sidecar_path_is_allowed() {
    let results = run_rule(&site(
        "crate_a/src/run.rs",
        2,
        "run_tests",
        Some("run_tests/mod.rs"),
        true,
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn facade_lib_test_sidecar_path_still_fires() {
    let results = run_rule(&site(
        "crate_a/src/lib.rs",
        2,
        "lib_tests",
        Some("lib_tests/mod.rs"),
        true,
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/lib.rs");
}

#[test]
fn generic_tests_name_with_sidecar_path_still_fires() {
    let results = run_rule(&site(
        "crate_a/src/rule.rs",
        2,
        "tests",
        Some("rule_tests/mod.rs"),
        true,
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/rule.rs");
}

#[test]
fn mismatched_sidecar_name_still_fires() {
    let results = run_rule(&site(
        "crate_a/src/run.rs",
        2,
        "helper_tests",
        Some("run_tests/mod.rs"),
        true,
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/run.rs");
}

#[test]
fn non_test_path_attr_still_fires() {
    let results = run_rule(&site(
        "crate_a/src/run.rs",
        1,
        "run_tests",
        Some("run_tests/mod.rs"),
        false,
    ));

    assertions::assert_path_attr_error(&results, "crate_a/src/run.rs");
}

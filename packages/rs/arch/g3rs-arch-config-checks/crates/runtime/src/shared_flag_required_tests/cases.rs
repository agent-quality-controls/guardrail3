use g3rs_arch_config_checks_assertions::shared_flag_required as assertions;

use super::helpers::{dependency_edge, run_rule};

#[test]
fn allows_assertions_to_depend_on_runtime_without_shared_flag() {
    let results = run_rule(&dependency_edge(
        "crates/assertions",
        "crates/runtime",
        "dependencies",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn allows_runtime_dev_dependency_on_assertions_without_shared_flag() {
    let results = run_rule(&dependency_edge(
        "crates/runtime",
        "crates/assertions",
        "dev-dependencies",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn allows_runtime_dev_dependency_on_test_support_without_shared_flag() {
    let results = run_rule(&dependency_edge(
        "crates/runtime",
        "crates/test_support",
        "dev-dependencies",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn still_requires_shared_flag_for_other_non_child_edges() {
    let results = run_rule(&dependency_edge(
        "crates/runtime",
        "crates/helpers",
        "dependencies",
    ));

    assertions::assert_shared_flag_violation(&results, "crates/runtime/Cargo.toml");
}

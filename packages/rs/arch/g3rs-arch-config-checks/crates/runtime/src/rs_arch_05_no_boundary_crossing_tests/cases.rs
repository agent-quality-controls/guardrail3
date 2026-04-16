use g3rs_arch_config_checks_assertions::rs_arch_05_no_boundary_crossing as assertions;

use super::helpers::{allow_shared_target, dependency_edge, run_rule};

#[test]
fn allows_assertions_to_depend_on_runtime() {
    let results = run_rule(&dependency_edge(
        "crates/assertions",
        "crates/runtime",
        "dependencies",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn allows_runtime_dev_dependency_on_assertions() {
    let results = run_rule(&dependency_edge(
        "crates/runtime",
        "crates/assertions",
        "dev-dependencies",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn allows_runtime_dev_dependency_on_test_support() {
    let results = run_rule(&dependency_edge(
        "crates/runtime",
        "crates/test_support",
        "dev-dependencies",
    ));

    assertions::assert_no_findings(&results);
}

#[test]
fn allows_runtime_dependency_on_shared_types_crate() {
    let results = run_rule(&allow_shared_target(dependency_edge(
        "crates/runtime",
        "crates/types",
        "dependencies",
    )));

    assertions::assert_no_findings(&results);
}

#[test]
fn allows_assertions_dependency_on_shared_types_crate() {
    let results = run_rule(&allow_shared_target(dependency_edge(
        "crates/assertions",
        "crates/types",
        "dependencies",
    )));

    assertions::assert_no_findings(&results);
}

#[test]
fn still_rejects_normal_runtime_dependency_on_assertions() {
    let results = run_rule(&dependency_edge(
        "crates/runtime",
        "crates/assertions",
        "dependencies",
    ));

    assertions::assert_boundary_violation(&results, "crates/runtime/Cargo.toml");
}

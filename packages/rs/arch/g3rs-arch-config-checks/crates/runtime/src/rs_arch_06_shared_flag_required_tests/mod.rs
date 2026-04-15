use g3rs_arch_config_checks_assertions::has_rule;

use crate::test_support::{config_crate, dependency_edge, input};

#[test]
fn allows_assertions_to_depend_on_runtime_without_shared_flag() {
    let results = crate::check(&input(
        vec![config_crate("crates/assertions"), config_crate("crates/runtime")],
        vec![dependency_edge(
            "crates/assertions",
            "crates/runtime",
            "dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-06"));
}

#[test]
fn allows_runtime_dev_dependency_on_assertions_without_shared_flag() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), config_crate("crates/assertions")],
        vec![dependency_edge(
            "crates/runtime",
            "crates/assertions",
            "dev-dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-06"));
}

#[test]
fn allows_runtime_dev_dependency_on_test_support_without_shared_flag() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), config_crate("crates/test_support")],
        vec![dependency_edge(
            "crates/runtime",
            "crates/test_support",
            "dev-dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-06"));
}

#[test]
fn still_requires_shared_flag_for_other_non_child_edges() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), config_crate("crates/helpers")],
        vec![dependency_edge(
            "crates/runtime",
            "crates/helpers",
            "dependencies",
        )],
    ));

    assert!(has_rule(&results, "RS-ARCH-CONFIG-06"));
}

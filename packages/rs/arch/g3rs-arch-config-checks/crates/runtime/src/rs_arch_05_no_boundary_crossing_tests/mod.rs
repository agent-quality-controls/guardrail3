use g3rs_arch_config_checks_assertions::has_rule;

use crate::test_support::{
    config_crate, dependency_edge, input, shared_config_crate, shared_dependency_edge,
};

#[test]
fn allows_assertions_to_depend_on_runtime() {
    let results = crate::check(&input(
        vec![config_crate("crates/assertions"), config_crate("crates/runtime")],
        vec![dependency_edge(
            "crates/assertions",
            "crates/runtime",
            "dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-05"));
}

#[test]
fn allows_runtime_dev_dependency_on_assertions() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), config_crate("crates/assertions")],
        vec![dependency_edge(
            "crates/runtime",
            "crates/assertions",
            "dev-dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-05"));
}

#[test]
fn allows_runtime_dev_dependency_on_test_support() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), config_crate("crates/test_support")],
        vec![dependency_edge(
            "crates/runtime",
            "crates/test_support",
            "dev-dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-05"));
}

#[test]
fn allows_runtime_dependency_on_shared_types_crate() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), shared_config_crate("crates/types")],
        vec![shared_dependency_edge(
            "crates/runtime",
            "crates/types",
            "dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-05"));
}

#[test]
fn allows_assertions_dependency_on_shared_types_crate() {
    let results = crate::check(&input(
        vec![
            config_crate("crates/assertions"),
            shared_config_crate("crates/types"),
        ],
        vec![shared_dependency_edge(
            "crates/assertions",
            "crates/types",
            "dependencies",
        )],
    ));

    assert!(!has_rule(&results, "RS-ARCH-CONFIG-05"));
}

#[test]
fn still_rejects_normal_runtime_dependency_on_assertions() {
    let results = crate::check(&input(
        vec![config_crate("crates/runtime"), config_crate("crates/assertions")],
        vec![dependency_edge(
            "crates/runtime",
            "crates/assertions",
            "dependencies",
        )],
    ));

    assert!(has_rule(&results, "RS-ARCH-CONFIG-05"));
}

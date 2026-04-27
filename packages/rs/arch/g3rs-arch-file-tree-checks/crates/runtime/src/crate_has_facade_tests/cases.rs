use g3rs_arch_file_tree_checks_assertions::crate_has_facade as assertions;

use super::helpers::{crate_node, run_rule};

#[test]
fn crate_with_lib_rs_inventories_facade() {
    let mut node = crate_node("crate_a");
    node.has_lib_rs = true;

    let results = run_rule(&node);

    assertions::assert_facade_inventory(&results, "crate_a/Cargo.toml");
}

#[test]
fn crate_with_main_rs_inventories_facade() {
    let mut node = crate_node("crate_a");
    node.has_main_rs = true;

    let results = run_rule(&node);

    assertions::assert_facade_inventory(&results, "crate_a/Cargo.toml");
}

#[test]
fn crate_without_lib_or_main_fires() {
    let results = run_rule(&crate_node("crate_a"));

    assertions::assert_missing_facade(&results, "crate_a/Cargo.toml");
}

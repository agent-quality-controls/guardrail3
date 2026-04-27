use g3rs_arch_config_checks_assertions::dependency_count_split as assertions;

use super::helpers::{config_crate, run_rule};

#[test]
fn exact_dependency_threshold_stays_quiet() {
    let mut node = config_crate("crate_a");
    node.production_dependency_count = 12;
    node.dev_dependency_count = 99;

    let results = run_rule(&node);

    assertions::assert_no_findings(&results);
}

#[test]
fn dependency_threshold_over_limit_fires_config_rule() {
    let mut node = config_crate("crate_a");
    node.production_dependency_count = 13;

    let results = run_rule(&node);

    assertions::assert_split_violation(&results, "crate_a/Cargo.toml");
}

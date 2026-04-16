use g3rs_clippy_filetree_checks_assertions::run::rule as assertions;
use test_support::input;

#[test]
fn clean_root_emits_only_coverage_inventory() {
    let results = super::super::check(&input(Some(".clippy.toml"), &[]));
    assertions::assert_clean_root_coverage(&results);
}

#[test]
fn plain_clippy_toml_also_counts_as_root_coverage() {
    let results = super::super::check(&input(Some("clippy.toml"), &[]));
    assertions::assert_plain_root_coverage(&results);
}

#[test]
fn missing_root_emits_only_uncovered_error() {
    let results = super::super::check(&input(None, &[]));
    assertions::assert_missing_root(&results);
}

#[test]
fn same_root_dual_config_emits_coverage_inventory_and_conflict_error() {
    let results = super::super::check(&input(
        Some(".clippy.toml"),
        &[("clippy.toml", ".clippy.toml")],
    ));
    assertions::assert_same_root_conflict(&results);
}

use g3ts_topology_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::super::ingest_for_file_tree_checks;
use super::helpers::{mkdir_p, write_guardrail3_ts_toml, write_package_json};

#[test]
fn descendant_with_sibling_package_json_under_outer_unit_yields_nested_fact() {
    let dir = tempdir().expect("create temporary directory for test fixture");
    let outer = dir.path();
    write_package_json(outer);
    write_guardrail3_ts_toml(outer);

    let inner = outer.join("packages/inner");
    mkdir_p(&inner);
    write_package_json(&inner);
    write_guardrail3_ts_toml(&inner);

    let input = ingest_for_file_tree_checks(outer)
        .expect("ingest_for_file_tree_checks should succeed for adopted unit fixture");

    assertions::assert_nested_at(&input, "packages/inner/guardrail3-ts.toml");
}

#[test]
fn descendant_without_sibling_package_json_under_outer_unit_still_yields_nested_fact() {
    // reason: rule is inward-only on `guardrail3-ts.toml`; sibling-package.json is recorded but
    // the nesting fact fires regardless because the marker file alone breaks routing.
    let dir = tempdir().expect("create temporary directory for test fixture");
    let outer = dir.path();
    write_package_json(outer);
    write_guardrail3_ts_toml(outer);

    let inner = outer.join("packages/loose");
    mkdir_p(&inner);
    write_guardrail3_ts_toml(&inner);

    let input = ingest_for_file_tree_checks(outer)
        .expect("ingest_for_file_tree_checks should succeed for adopted unit fixture");

    assertions::assert_nested_at(&input, "packages/loose/guardrail3-ts.toml");
}

#[test]
fn no_descendant_guardrail3_ts_toml_yields_no_nested_facts() {
    let dir = tempdir().expect("create temporary directory for test fixture");
    let outer = dir.path();
    write_package_json(outer);
    write_guardrail3_ts_toml(outer);

    let inner = outer.join("packages/inner");
    mkdir_p(&inner);
    write_package_json(&inner);

    let input = ingest_for_file_tree_checks(outer)
        .expect("ingest_for_file_tree_checks should succeed for adopted unit fixture");

    assertions::assert_no_nested(&input);
}

#[test]
fn unit_root_without_marker_pair_errors() {
    let dir = tempdir().expect("create temporary directory for test fixture");
    let outer = dir.path();
    write_package_json(outer);
    // reason: missing `guardrail3-ts.toml` at the unit root should be rejected at ingestion.

    let result = ingest_for_file_tree_checks(outer);
    assert!(
        result.is_err(),
        "expected error for half-adopted unit root, got {result:?}"
    );
}

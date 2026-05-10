use g3ts_topology_file_tree_checks_assertions::run as run_assertions;
use g3ts_topology_ingestion_runtime::ingest_for_file_tree_checks;
use g3ts_topology_types::{
    G3TsTopologyFileTreeChecksInput, G3TsTopologyNestedGuardrail3TsTomlInput,
};
use tempfile::tempdir;

use super::helpers::{mkdir_p, write_guardrail3_ts_toml, write_package_json};

const NESTED_RULE_ID: &str = "g3ts-topology/no-nested-guardrail3-ts-toml";

#[test]
fn run_dispatches_nested_guardrail3_ts_toml_inputs() {
    let input = G3TsTopologyFileTreeChecksInput {
        unit_root_rel_dir: String::new(),
        unit_root_package_json_rel_path: "package.json".to_owned(),
        unit_root_guardrail3_ts_toml_rel_path: "guardrail3-ts.toml".to_owned(),
        descendant_guardrail3_ts_tomls: Vec::new(),
        input_failures: Vec::new(),
        nested_guardrail3_ts_tomls: vec![G3TsTopologyNestedGuardrail3TsTomlInput {
            rel_dir: "packages/inner".to_owned(),
            toml_rel_path: "packages/inner/guardrail3-ts.toml".to_owned(),
            parent_unit_rel: String::new(),
        }],
    };

    let results = run_assertions::run(&input);
    run_assertions::assert_rule_ids(&results, &[NESTED_RULE_ID]);
}

#[test]
fn descendant_adopted_unit_under_outer_unit_fires_rule_end_to_end() {
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
    let results = run_assertions::run(&input);

    run_assertions::assert_rule_fired(&results, NESTED_RULE_ID);
}

#[test]
fn sibling_adopted_units_outside_outer_do_not_fire_rule_end_to_end() {
    // reason: when ingesting from outer, only the outer subtree is walked. Siblings of the outer
    // unit are outside its tree and must not produce findings on the outer's run.
    let dir = tempdir().expect("create temporary directory for test fixture");
    let workspace = dir.path();

    let outer = workspace.join("apps/outer");
    mkdir_p(&outer);
    write_package_json(&outer);
    write_guardrail3_ts_toml(&outer);

    let sibling = workspace.join("apps/sibling");
    mkdir_p(&sibling);
    write_package_json(&sibling);
    write_guardrail3_ts_toml(&sibling);

    let input = ingest_for_file_tree_checks(&outer)
        .expect("ingest_for_file_tree_checks should succeed for adopted unit fixture");
    let results = run_assertions::run(&input);

    run_assertions::assert_rule_quiet(&results, NESTED_RULE_ID);
}

#[test]
fn unit_root_without_marker_pair_means_no_outer_unit_so_rule_does_not_fire() {
    // reason: outer dir has only `package.json` (no `guardrail3-ts.toml`), so it is not adopted
    // and the rule is not run from that directory at all. Ingestion rejects the unit root.
    let dir = tempdir().expect("create temporary directory for test fixture");
    let outer = dir.path();
    write_package_json(outer);

    let inner = outer.join("packages/inner");
    mkdir_p(&inner);
    write_package_json(&inner);
    write_guardrail3_ts_toml(&inner);

    let result = ingest_for_file_tree_checks(outer);
    assert!(
        result.is_err(),
        "expected error when outer is not adopted, got {result:?}"
    );
}

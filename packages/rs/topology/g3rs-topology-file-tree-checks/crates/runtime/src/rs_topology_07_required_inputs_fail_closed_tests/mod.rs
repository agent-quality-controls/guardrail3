use g3rs_topology_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use g3rs_topology_types::G3RsTopologyFileTreeInputFailure;
use guardrail3_check_types::G3Severity;

use crate::test_support::input_with_failures;

#[test]
fn empty_failure_list_stays_quiet() {
    let input = input_with_failures("[workspace]\nmembers = []\n", Vec::new(), Vec::new(), Vec::new());

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-FILETREE-07", &[]);
}

#[test]
fn descendant_manifest_failure_fires() {
    let input = input_with_failures(
        "[workspace]\nmembers = [\"bad\"]\n",
        Vec::new(),
        Vec::new(),
        vec![G3RsTopologyFileTreeInputFailure {
            rel_path: "bad/Cargo.toml".to_owned(),
            message: "parse failed".to_owned(),
        }],
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-07",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Rust topology required input failed closed"),
            file: Some("bad/Cargo.toml"),
            inventory: Some(false),
            message: Some("parse failed"),
        }],
    );
}

#[test]
fn multiple_failures_emit_one_result_each() {
    let input = input_with_failures(
        "[workspace]\nmembers = [\"bad\", \"worse\"]\n",
        Vec::new(),
        Vec::new(),
        vec![
            G3RsTopologyFileTreeInputFailure {
                rel_path: "bad/Cargo.toml".to_owned(),
                message: "parse failed".to_owned(),
            },
            G3RsTopologyFileTreeInputFailure {
                rel_path: "worse/Cargo.toml".to_owned(),
                message: "file is not readable".to_owned(),
            },
        ],
    );

    let results = crate::check(&input);
    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-07",
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("Rust topology required input failed closed"),
                file: Some("bad/Cargo.toml"),
                inventory: Some(false),
                message: Some("parse failed"),
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("Rust topology required input failed closed"),
                file: Some("worse/Cargo.toml"),
                inventory: Some(false),
                message: Some("file is not readable"),
            },
        ],
    );
}

#[test]
fn failures_coexist_with_other_topology_rules() {
    let input = input_with_failures(
        "[workspace]\nmembers = [\"good\", \"missing\"]\n",
        vec![
            ("good", Some(g3rs_topology_types::G3RsTopologyCargoManifestKind::Package)),
            ("nested", Some(g3rs_topology_types::G3RsTopologyCargoManifestKind::Workspace)),
        ],
        Vec::new(),
        vec![G3RsTopologyFileTreeInputFailure {
            rel_path: "missing/Cargo.toml".to_owned(),
            message: "parse failed".to_owned(),
        }],
    );

    let results = crate::check(&input);

    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-07")
            .count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-11")
            .count(),
        1
    );
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-FILETREE-12")
            .count(),
        1
    );
}

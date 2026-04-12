use g3rs_topology_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use g3rs_topology_types::G3RsTopologyCargoManifestKind;
use guardrail3_check_types::G3Severity;

use crate::test_support::input;

#[test]
fn nested_workspace_root_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            (
                "crates/api/nested",
                Some(G3RsTopologyCargoManifestKind::Workspace),
            ),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-11",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Nested workspace `crates/api/nested` is forbidden"),
            file: Some("crates/api/nested/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn nested_hybrid_workspace_root_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            (
                "crates/api/nested",
                Some(G3RsTopologyCargoManifestKind::Hybrid),
            ),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-11",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Nested workspace `crates/api/nested` is forbidden"),
            file: Some("crates/api/nested/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn nested_workspace_listed_in_members_still_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"crates/api/nested\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            (
                "crates/api/nested",
                Some(G3RsTopologyCargoManifestKind::Workspace),
            ),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-11",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Nested workspace `crates/api/nested` is forbidden"),
            file: Some("crates/api/nested/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn nested_workspace_excluded_from_parent_still_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\nexclude = [\"crates/api/nested\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            (
                "crates/api/nested",
                Some(G3RsTopologyCargoManifestKind::Workspace),
            ),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-FILETREE-11",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Nested workspace `crates/api/nested` is forbidden"),
            file: Some("crates/api/nested/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn package_child_does_not_fire_nested_workspace_rule() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-FILETREE-11", &[]);
}

use g3rs_topology_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use g3rs_topology_types::G3RsTopologyCargoManifestKind;
use guardrail3_check_types::G3Severity;

use crate::test_support::input;

#[test]
fn undeclared_child_root_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/extra", Some(G3RsTopologyCargoManifestKind::Package)),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-12",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Workspace child `crates/extra` must be declared explicitly"),
            file: Some("crates/extra/Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn extra_workspace_member_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-12",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("Workspace `.` has extra member `crates/ghost`"),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn exact_match_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = [\"crates/*\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/lib", Some(G3RsTopologyCargoManifestKind::Package)),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-12", &[]);
}

#[test]
fn dot_slash_member_path_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = [\"./crates/api\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-12", &[]);
}

#[test]
fn dot_slash_glob_member_path_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = [\"./crates/*\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/lib", Some(G3RsTopologyCargoManifestKind::Package)),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-12", &[]);
}

#[test]
fn missing_and_extra_are_both_reported() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"crates/ghost\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/core", Some(G3RsTopologyCargoManifestKind::Package)),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-12",
        &[
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("Workspace child `crates/core` must be declared explicitly"),
                file: Some("crates/core/Cargo.toml"),
                inventory: Some(false),
                message: None,
            },
            ExpectedRuleResult {
                severity: Some(G3Severity::Error),
                title: Some("Workspace `.` has extra member `crates/ghost`"),
                file: Some("Cargo.toml"),
                inventory: Some(false),
                message: None,
            },
        ],
    );
}

#[test]
fn nested_workspace_does_not_also_fire_membership_rule() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"crates/nested\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/nested", Some(G3RsTopologyCargoManifestKind::Workspace)),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-12", &[]);
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-11")
            .count(),
        1
    );
}

#[test]
fn hybrid_descendant_does_not_also_fire_membership_rule() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\", \"crates/nested\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/nested", Some(G3RsTopologyCargoManifestKind::Hybrid)),
        ],
        Vec::new(),
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-12", &[]);
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-11")
            .count(),
        1
    );
}

#[test]
fn parse_failed_descendant_does_not_also_fire_membership_rule() {
    let input = crate::test_support::input_with_failures(
        "[workspace]\nmembers = [\"crates/api\", \"crates/bad\"]\n",
        vec![
            ("crates/api", Some(G3RsTopologyCargoManifestKind::Package)),
            ("crates/bad", None),
        ],
        Vec::new(),
        vec![g3rs_topology_types::G3RsTopologyFileTreeInputFailure {
            rel_path: "crates/bad/Cargo.toml".to_owned(),
            message: "parse failed".to_owned(),
        }],
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-12", &[]);
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-07")
            .count(),
        1
    );
}

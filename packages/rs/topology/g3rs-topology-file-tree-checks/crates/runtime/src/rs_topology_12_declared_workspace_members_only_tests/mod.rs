use g3rs_topology_file_tree_checks_assertions::has_rule;
use g3rs_topology_types::G3RsTopologyCargoManifestKind;

use crate::test_support::{input, titles};

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

    assert!(has_rule(&results, "RS-TOPOLOGY-12"));
    assert!(
        titles(&results, "RS-TOPOLOGY-12")
            .iter()
            .any(|title| *title == "Workspace child `crates/extra` must be declared explicitly")
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

    assert!(
        titles(&results, "RS-TOPOLOGY-12")
            .iter()
            .any(|title| *title == "Workspace `.` has extra member `crates/ghost`")
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

    assert!(!has_rule(&results, "RS-TOPOLOGY-12"));
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

    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-TOPOLOGY-12")
            .count(),
        2
    );
    assert!(
        titles(&results, "RS-TOPOLOGY-12")
            .iter()
            .any(|title| title == "Workspace child `crates/core` must be declared explicitly")
    );
    assert!(
        titles(&results, "RS-TOPOLOGY-12")
            .iter()
            .any(|title| title == "Workspace `.` has extra member `crates/ghost`")
    );
}

use g3rs_topology_file_tree_checks_assertions::has_rule;
use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFileAttachment, G3RsTopologyWorkspaceFamilyFileKind,
};

use crate::test_support::{family_file, input, titles};

#[test]
fn nested_policy_file_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Clippy,
            "crates/api/nested/clippy.toml",
            G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel: "crates/api".to_owned(),
                owner_rel: "crates/api/nested".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert!(has_rule(&results, "RS-TOPOLOGY-16"));
}

#[test]
fn member_root_policy_file_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Clippy,
            "crates/api/clippy.toml",
            G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: "crates/api".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert!(
        titles(&results, "RS-TOPOLOGY-16")
            .iter()
            .any(|title| *title == "`clippy` file `crates/api/clippy.toml` is illegally placed")
    );
}

#[test]
fn root_level_fmt_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = []\n",
        Vec::new(),
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Fmt,
            "rustfmt.toml",
            G3RsTopologyWorkspaceFamilyFileKind::RustfmtToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: String::new(),
            },
        )],
    );

    let results = crate::check(&input);

    assert!(!has_rule(&results, "RS-TOPOLOGY-16"));
}

#[test]
fn root_level_cargo_config_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = []\n",
        Vec::new(),
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Clippy,
            ".cargo/config.toml",
            G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel: String::new(),
                owner_rel: ".cargo".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert!(!has_rule(&results, "RS-TOPOLOGY-16"));
}

#[test]
fn root_level_nextest_config_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = []\n",
        Vec::new(),
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Test,
            ".config/nextest.toml",
            G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel: String::new(),
                owner_rel: ".config".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert!(!has_rule(&results, "RS-TOPOLOGY-16"));
}

use g3rs_topology_file_tree_checks_assertions::{ExpectedRuleResult, assert_rule_results};
use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFileAttachment, G3RsTopologyWorkspaceFamilyFileKind,
};
use guardrail3_check_types::G3Severity;

use crate::test_support::{family_file, input};

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

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-16",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`clippy` file `crates/api/nested/clippy.toml` is illegally placed"),
            file: Some("crates/api/nested/clippy.toml"),
            inventory: Some(false),
            message: Some("`crates/api/nested/clippy.toml` is nested under `crates/api/nested`. Workspace-local `clippy` files must live directly at the workspace root `.` rather than in nested subdirectories."),
        }],
    );
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

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-16",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`clippy` file `crates/api/clippy.toml` is illegally placed"),
            file: Some("crates/api/clippy.toml"),
            inventory: Some(false),
            message: Some("`crates/api/clippy.toml` is attached to legal workspace member `crates/api`. Workspace-local `clippy` files must live at the workspace root `.` instead of inside a member crate."),
        }],
    );
}

#[test]
fn member_nextest_file_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Test,
            "crates/api/.config/nextest.toml",
            G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: "crates/api".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-16",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`test` file `crates/api/.config/nextest.toml` is illegally placed"),
            file: Some("crates/api/.config/nextest.toml"),
            inventory: Some(false),
            message: Some("`crates/api/.config/nextest.toml` is attached to legal workspace member `crates/api`. Workspace-local `test` files must live at the workspace root `.` instead of inside a member crate."),
        }],
    );
}

#[test]
fn illegal_child_root_branch_is_reported() {
    let input = input(
        "[workspace]\nmembers = []\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Deny,
            "crates/api/deny.toml",
            G3RsTopologyWorkspaceFamilyFileKind::DenyToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: "crates/api".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-16",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`deny` file `crates/api/deny.toml` is illegally placed"),
            file: Some("crates/api/deny.toml"),
            inventory: Some(false),
            message: Some("`crates/api/deny.toml` is attached to illegal child root `crates/api`. Workspace-local `deny` files must live at the workspace root `.`."),
        }],
    );
}

#[test]
fn non_member_root_branch_is_reported() {
    let input = input(
        "[workspace]\nmembers = []\n",
        Vec::new(),
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Release,
            "vendor/release-plz.toml",
            G3RsTopologyWorkspaceFamilyFileKind::ReleasePlzToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: "vendor".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-16",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`release` file `vendor/release-plz.toml` is illegally placed"),
            file: Some("vendor/release-plz.toml"),
            inventory: Some(false),
            message: Some("`vendor/release-plz.toml` is attached to non-member root `vendor`. Workspace-local `release` files must live at the workspace root `.`."),
        }],
    );
}

#[test]
fn misplaced_fmt_file_fires() {
    let input = input(
        "[workspace]\nmembers = [\"crates/api\"]\n",
        vec![("crates/api", Some(G3RsTopologyCargoManifestKind::Package))],
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Fmt,
            "crates/api/rustfmt.toml",
            G3RsTopologyWorkspaceFamilyFileKind::RustfmtToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: "crates/api".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert_rule_results(
        &results,
        "RS-TOPOLOGY-16",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("`fmt` file `crates/api/rustfmt.toml` is illegally placed"),
            file: Some("crates/api/rustfmt.toml"),
            inventory: Some(false),
            message: Some("fmt files must live at the validation root, not inside a workspace member or nested subdirectory."),
        }],
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

    assert_rule_results(&results, "RS-TOPOLOGY-16", &[]);
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

    assert_rule_results(&results, "RS-TOPOLOGY-16", &[]);
}

#[test]
fn root_level_cargo_config_legacy_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = []\n",
        Vec::new(),
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Clippy,
            ".cargo/config",
            G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel: String::new(),
                owner_rel: ".cargo".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-16", &[]);
}

#[test]
fn root_level_cargo_deny_stays_quiet() {
    let input = input(
        "[workspace]\nmembers = []\n",
        Vec::new(),
        vec![family_file(
            G3RsTopologyWorkspaceFamily::Deny,
            ".cargo/deny.toml",
            G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel: String::new(),
                owner_rel: ".cargo".to_owned(),
            },
        )],
    );

    let results = crate::check(&input);

    assert_rule_results(&results, "RS-TOPOLOGY-16", &[]);
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

    assert_rule_results(&results, "RS-TOPOLOGY-16", &[]);
}

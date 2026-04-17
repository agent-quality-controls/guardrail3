use g3rs_topology_file_tree_checks_assertions::rs_topology_16_workspace_local_file_placement::rule as assertions;
use g3rs_topology_types::{
    G3RsTopologyWorkspaceFamily,
};

use super::super::{IllegalFamilyFilePlacementInput, check};

#[test]
fn nested_policy_file_fires() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Clippy,
        rel_path: "crates/api/nested/clippy.toml".to_owned(),
        reason: "`crates/api/nested/clippy.toml` is nested under `crates/api/nested`. Workspace-local `clippy` files must live directly at the workspace root `.` rather than in nested subdirectories.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`clippy` file `crates/api/nested/clippy.toml` is illegally placed",
            "`crates/api/nested/clippy.toml` is nested under `crates/api/nested`. Workspace-local `clippy` files must live directly at the workspace root `.` rather than in nested subdirectories.",
            Some("crates/api/nested/clippy.toml"),
            false,
        ),
    );
}

#[test]
fn member_root_policy_file_fires() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Clippy,
        rel_path: "crates/api/clippy.toml".to_owned(),
        reason: "`crates/api/clippy.toml` is attached to legal workspace member `crates/api`. Workspace-local `clippy` files must live at the workspace root `.` instead of inside a member crate.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`clippy` file `crates/api/clippy.toml` is illegally placed",
            "`crates/api/clippy.toml` is attached to legal workspace member `crates/api`. Workspace-local `clippy` files must live at the workspace root `.` instead of inside a member crate.",
            Some("crates/api/clippy.toml"),
            false,
        ),
    );
}

#[test]
fn member_nextest_file_fires() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Test,
        rel_path: "crates/api/.config/nextest.toml".to_owned(),
        reason: "`crates/api/.config/nextest.toml` is attached to legal workspace member `crates/api`. Workspace-local `test` files must live at the workspace root `.` instead of inside a member crate.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`test` file `crates/api/.config/nextest.toml` is illegally placed",
            "`crates/api/.config/nextest.toml` is attached to legal workspace member `crates/api`. Workspace-local `test` files must live at the workspace root `.` instead of inside a member crate.",
            Some("crates/api/.config/nextest.toml"),
            false,
        ),
    );
}

#[test]
fn illegal_child_root_branch_is_reported() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Deny,
        rel_path: "crates/api/deny.toml".to_owned(),
        reason: "`crates/api/deny.toml` is attached to illegal child root `crates/api`. Workspace-local `deny` files must live at the workspace root `.`.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`deny` file `crates/api/deny.toml` is illegally placed",
            "`crates/api/deny.toml` is attached to illegal child root `crates/api`. Workspace-local `deny` files must live at the workspace root `.`.",
            Some("crates/api/deny.toml"),
            false,
        ),
    );
}

#[test]
fn non_member_root_branch_is_reported() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Release,
        rel_path: "vendor/release-plz.toml".to_owned(),
        reason: "`vendor/release-plz.toml` is attached to non-member root `vendor`. Workspace-local `release` files must live at the workspace root `.`.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`release` file `vendor/release-plz.toml` is illegally placed",
            "`vendor/release-plz.toml` is attached to non-member root `vendor`. Workspace-local `release` files must live at the workspace root `.`.",
            Some("vendor/release-plz.toml"),
            false,
        ),
    );
}

#[test]
fn misplaced_fmt_file_fires() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Fmt,
        rel_path: "crates/api/rustfmt.toml".to_owned(),
        reason: "fmt files must live at the validation root, not inside a workspace member or nested subdirectory.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`fmt` file `crates/api/rustfmt.toml` is illegally placed",
            "fmt files must live at the validation root, not inside a workspace member or nested subdirectory.",
            Some("crates/api/rustfmt.toml"),
            false,
        ),
    );
}

#[test]
fn fmt_message_is_preserved_verbatim() {
    let input = IllegalFamilyFilePlacementInput {
        family: G3RsTopologyWorkspaceFamily::Fmt,
        rel_path: "crates/api/rustfmt.toml".to_owned(),
        reason: "fmt files must live at the validation root, not inside a workspace member or nested subdirectory.".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "`fmt` file `crates/api/rustfmt.toml` is illegally placed",
            "fmt files must live at the validation root, not inside a workspace member or nested subdirectory.",
            Some("crates/api/rustfmt.toml"),
            false,
        ),
    );
}

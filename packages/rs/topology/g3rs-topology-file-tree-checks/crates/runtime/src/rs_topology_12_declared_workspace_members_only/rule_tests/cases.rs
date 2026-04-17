use g3rs_topology_file_tree_checks_assertions::rs_topology_12_declared_workspace_members_only::rule as assertions;

use super::super::{WorkspaceMemberIssueInput, WorkspaceMemberIssueKind, check};

#[test]
fn undeclared_child_root_fires() {
    let input = WorkspaceMemberIssueInput {
        rel_dir: "crates/extra".to_owned(),
        cargo_rel_path: "crates/extra/Cargo.toml".to_owned(),
        kind: WorkspaceMemberIssueKind::Undeclared {
            workspace_root_rel: String::new(),
        },
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace child `crates/extra` must be declared explicitly",
            "`crates/extra/Cargo.toml` sits under workspace `.`, but it is not a declared workspace member. Workspace membership must exactly match real child Rust roots. Add this crate's path to `[workspace] members` in `./Cargo.toml`.",
            Some("crates/extra/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn extra_workspace_member_fires() {
    let input = WorkspaceMemberIssueInput {
        rel_dir: String::new(),
        cargo_rel_path: "Cargo.toml".to_owned(),
        kind: WorkspaceMemberIssueKind::Extra {
            workspace_root_rel: String::new(),
            member_pattern: "crates/ghost".to_owned(),
        },
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `.` has extra member `crates/ghost`",
            "`Cargo.toml` declares workspace member `crates/ghost`, but it does not match a real owned child Rust root. Workspace membership must exactly match real child Rust roots. Remove `crates/ghost` from `[workspace] members`, or create the missing crate.",
            Some("Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn undeclared_issue_under_nested_workspace_mentions_parent_workspace() {
    let input = WorkspaceMemberIssueInput {
        rel_dir: "crates/core".to_owned(),
        cargo_rel_path: "crates/core/Cargo.toml".to_owned(),
        kind: WorkspaceMemberIssueKind::Undeclared {
            workspace_root_rel: "crates".to_owned(),
        },
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace child `crates/core` must be declared explicitly",
            "`crates/core/Cargo.toml` sits under workspace `crates`, but it is not a declared workspace member. Workspace membership must exactly match real child Rust roots. Add this crate's path to `[workspace] members` in `crates/Cargo.toml`.",
            Some("crates/core/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn extra_member_under_nested_workspace_mentions_parent_workspace() {
    let input = WorkspaceMemberIssueInput {
        rel_dir: "crates".to_owned(),
        cargo_rel_path: "crates/Cargo.toml".to_owned(),
        kind: WorkspaceMemberIssueKind::Extra {
            workspace_root_rel: "crates".to_owned(),
            member_pattern: "ghost".to_owned(),
        },
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `crates` has extra member `ghost`",
            "`crates/Cargo.toml` declares workspace member `ghost`, but it does not match a real owned child Rust root. Workspace membership must exactly match real child Rust roots. Remove `ghost` from `[workspace] members`, or create the missing crate.",
            Some("crates/Cargo.toml"),
            false,
        ),
    );
}

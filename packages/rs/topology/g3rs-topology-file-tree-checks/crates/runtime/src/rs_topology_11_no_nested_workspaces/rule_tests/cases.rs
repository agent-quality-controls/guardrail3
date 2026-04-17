use g3rs_topology_file_tree_checks_assertions::rs_topology_11_no_nested_workspaces::rule as assertions;

use super::super::{NestedWorkspaceInput, check};

#[test]
fn nested_workspace_root_fires() {
    let input = NestedWorkspaceInput {
        rel_dir: "crates/api/nested".to_owned(),
        cargo_rel_path: "crates/api/nested/Cargo.toml".to_owned(),
        parent_workspace_rel: String::new(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested workspace `crates/api/nested` is forbidden",
            "`crates/api/nested/Cargo.toml` declares a nested workspace under `.`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `.`.",
            Some("crates/api/nested/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn nested_hybrid_workspace_root_fires() {
    let input = NestedWorkspaceInput {
        rel_dir: "crates/api/nested".to_owned(),
        cargo_rel_path: "crates/api/nested/Cargo.toml".to_owned(),
        parent_workspace_rel: String::new(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested workspace `crates/api/nested` is forbidden",
            "`crates/api/nested/Cargo.toml` declares a nested workspace under `.`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `.`.",
            Some("crates/api/nested/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn nested_workspace_listed_in_members_still_fires() {
    let input = NestedWorkspaceInput {
        rel_dir: "crates/api/nested".to_owned(),
        cargo_rel_path: "crates/api/nested/Cargo.toml".to_owned(),
        parent_workspace_rel: String::new(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested workspace `crates/api/nested` is forbidden",
            "`crates/api/nested/Cargo.toml` declares a nested workspace under `.`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `.`.",
            Some("crates/api/nested/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn nested_workspace_excluded_from_parent_still_fires() {
    let input = NestedWorkspaceInput {
        rel_dir: "crates/api/nested".to_owned(),
        cargo_rel_path: "crates/api/nested/Cargo.toml".to_owned(),
        parent_workspace_rel: String::new(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested workspace `crates/api/nested` is forbidden",
            "`crates/api/nested/Cargo.toml` declares a nested workspace under `.`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `.`.",
            Some("crates/api/nested/Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn nested_workspace_under_non_root_parent_mentions_that_parent() {
    let input = NestedWorkspaceInput {
        rel_dir: "crates/api/nested".to_owned(),
        cargo_rel_path: "crates/api/nested/Cargo.toml".to_owned(),
        parent_workspace_rel: "crates/api".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Nested workspace `crates/api/nested` is forbidden",
            "`crates/api/nested/Cargo.toml` declares a nested workspace under `crates/api`. Cargo does not support nested workspaces. Remove the `[workspace]` section from this Cargo.toml, or move it so it is not nested under `crates/api`.",
            Some("crates/api/nested/Cargo.toml"),
            false,
        ),
    );
}

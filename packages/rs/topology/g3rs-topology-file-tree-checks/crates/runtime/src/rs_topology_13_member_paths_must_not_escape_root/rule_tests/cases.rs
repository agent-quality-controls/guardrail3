use g3rs_topology_file_tree_checks_assertions::rs_topology_13_member_paths_must_not_escape_root::rule as assertions;

use super::super::{EscapingWorkspaceMemberPathInput, check};

#[test]
fn escaping_member_path_fires() {
    let input = EscapingWorkspaceMemberPathInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_root_rel: String::new(),
        member_pattern: "../shared".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `.` uses escaping member path `../shared`",
            "`Cargo.toml` declares member pattern `../shared`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            Some("Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn absolute_member_path_fires() {
    let input = EscapingWorkspaceMemberPathInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_root_rel: String::new(),
        member_pattern: "/tmp/shared".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `.` uses escaping member path `/tmp/shared`",
            "`Cargo.toml` declares member pattern `/tmp/shared`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            Some("Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn windows_drive_absolute_member_path_fires() {
    let input = EscapingWorkspaceMemberPathInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_root_rel: String::new(),
        member_pattern: "C:/tmp/shared".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `.` uses escaping member path `C:/tmp/shared`",
            "`Cargo.toml` declares member pattern `C:/tmp/shared`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            Some("Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn unc_absolute_member_path_fires() {
    let input = EscapingWorkspaceMemberPathInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_root_rel: String::new(),
        member_pattern: "\\\\server\\share".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `.` uses escaping member path `\\\\server\\share`",
            "`Cargo.toml` declares member pattern `\\\\server\\share`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            Some("Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn backslash_absolute_member_path_fires() {
    let input = EscapingWorkspaceMemberPathInput {
        cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_root_rel: String::new(),
        member_pattern: "\\tmp\\shared".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `.` uses escaping member path `\\tmp\\shared`",
            "`Cargo.toml` declares member pattern `\\tmp\\shared`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            Some("Cargo.toml"),
            false,
        ),
    );
}

#[test]
fn escaping_member_under_nested_workspace_mentions_that_workspace() {
    let input = EscapingWorkspaceMemberPathInput {
        cargo_rel_path: "crates/Cargo.toml".to_owned(),
        workspace_root_rel: "crates".to_owned(),
        member_pattern: "../shared".to_owned(),
    };
    let mut results = Vec::new();

    check(&input, &mut results);

    assertions::assert_contains(
        &results,
        assertions::finding(
            assertions::Severity::Error,
            "Workspace `crates` uses escaping member path `../shared`",
            "`crates/Cargo.toml` declares member pattern `../shared`, which points outside the workspace directory. Workspace members must be relative subdirectory paths inside the workspace root, not absolute paths or `..` escapes. Change the pattern to a relative subdirectory path, or move the target crate inside the workspace.",
            Some("crates/Cargo.toml"),
            false,
        ),
    );
}

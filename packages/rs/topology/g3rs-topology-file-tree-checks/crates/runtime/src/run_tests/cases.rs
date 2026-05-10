use cargo_toml_parser::parse as parse_cargo_toml;
use g3rs_topology_file_tree_checks_assertions::run as assertions;
use g3rs_topology_types::{
    G3RsTopologyEscapingWorkspaceMemberPathInput, G3RsTopologyFileTreeChecksInput,
    G3RsTopologyFileTreeInputFailure, G3RsTopologyIllegalFamilyFilePlacementInput,
    G3RsTopologyNestedGuardrail3RsTomlInput, G3RsTopologyNestedWorkspaceInput,
    G3RsTopologyWorkspaceFamily, G3RsTopologyWorkspaceMemberIssueInput,
    G3RsTopologyWorkspaceMemberIssueKind,
};

use super::super::check;

#[test]
fn run_dispatches_precomputed_file_tree_inputs() {
    let input = G3RsTopologyFileTreeChecksInput {
        workspace_root_rel_dir: String::new(),
        workspace_root_cargo_rel_path: "Cargo.toml".to_owned(),
        workspace_manifest: parse_cargo_toml("[workspace]\nmembers = []\n")
            .expect("parse synthetic workspace manifest"),
        descendant_cargo_roots: Vec::new(),
        family_files: Vec::new(),
        input_failures: vec![G3RsTopologyFileTreeInputFailure {
            rel_path: "bad/Cargo.toml".to_owned(),
            message: "file is not readable".to_owned(),
        }],
        nested_workspaces: vec![G3RsTopologyNestedWorkspaceInput {
            rel_dir: "crates/api/nested".to_owned(),
            cargo_rel_path: "crates/api/nested/Cargo.toml".to_owned(),
            parent_workspace_rel: String::new(),
        }],
        nested_guardrail3_rs_tomls: vec![G3RsTopologyNestedGuardrail3RsTomlInput {
            rel_dir: "crates/api/inner".to_owned(),
            guardrail3_rs_toml_rel_path: "crates/api/inner/guardrail3-rs.toml".to_owned(),
            outer_adopted_unit_rel: String::new(),
        }],
        membership_issues: vec![G3RsTopologyWorkspaceMemberIssueInput {
            rel_dir: "crates/extra".to_owned(),
            cargo_rel_path: "crates/extra/Cargo.toml".to_owned(),
            kind: G3RsTopologyWorkspaceMemberIssueKind::Undeclared {
                workspace_root_rel: String::new(),
            },
        }],
        escaping_member_paths: vec![G3RsTopologyEscapingWorkspaceMemberPathInput {
            cargo_rel_path: "Cargo.toml".to_owned(),
            workspace_root_rel: String::new(),
            member_pattern: "../shared".to_owned(),
        }],
        illegal_family_files: vec![G3RsTopologyIllegalFamilyFilePlacementInput {
            family: G3RsTopologyWorkspaceFamily::Clippy,
            rel_path: "crates/api/clippy.toml".to_owned(),
            reason: "synthetic illegal file".to_owned(),
        }],
    };

    let results = check(&input);

    assertions::assert_precomputed_dispatch(&results);
}

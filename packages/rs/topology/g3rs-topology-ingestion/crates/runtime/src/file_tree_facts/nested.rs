use std::collections::BTreeSet;

use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyFileTreeChecksInput,
    G3RsTopologyNestedGuardrail3RsTomlInput, G3RsTopologyNestedWorkspaceInput,
    G3RsTopologyWorkspaceFamilyFile, G3RsTopologyWorkspaceFamilyFileAttachment,
    G3RsTopologyWorkspaceFamilyFileKind,
};

use super::paths::{nearest_ancestor_workspace, path_is_under};

/// `collect_nested_guardrail3_rs_toml_issues` function.
pub(super) fn collect_nested_guardrail3_rs_toml_issues(
    input: &G3RsTopologyFileTreeChecksInput,
) -> Vec<G3RsTopologyNestedGuardrail3RsTomlInput> {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    let root_marker_owner_rel = workspace_root_rel.to_owned();
    let root_is_adopted = input.family_files.iter().any(|file| {
        file.kind == G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml
            && marker_owner_rel(file) == root_marker_owner_rel
    });
    if !root_is_adopted {
        return Vec::new();
    }

    let mut seen = BTreeSet::new();
    input
        .family_files
        .iter()
        .filter(|file| file.kind == G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml)
        .filter_map(|file| {
            let owner_rel = marker_owner_rel(file);
            if owner_rel == root_marker_owner_rel {
                return None;
            }
            if !path_is_under(&owner_rel, workspace_root_rel) {
                return None;
            }
            if !seen.insert(file.rel_path.clone()) {
                return None;
            }
            Some(G3RsTopologyNestedGuardrail3RsTomlInput {
                rel_dir: owner_rel,
                guardrail3_rs_toml_rel_path: file.rel_path.clone(),
                outer_adopted_unit_rel: workspace_root_rel.to_owned(),
            })
        })
        .collect()
}

/// `marker_owner_rel` function.
pub(super) fn marker_owner_rel(file: &G3RsTopologyWorkspaceFamilyFile) -> String {
    match &file.attachment {
        G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel } => root_rel.clone(),
        G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot { owner_rel, .. } => {
            owner_rel.clone()
        }
    }
}

/// `collect_nested_workspace_issues` function.
pub(super) fn collect_nested_workspace_issues(
    input: &G3RsTopologyFileTreeChecksInput,
    workspace_roots: &BTreeSet<String>,
) -> Vec<G3RsTopologyNestedWorkspaceInput> {
    input
        .descendant_cargo_roots
        .iter()
        .filter(|root| {
            matches!(
                root.manifest_kind,
                Some(
                    G3RsTopologyCargoManifestKind::Workspace
                        | G3RsTopologyCargoManifestKind::Hybrid
                )
            )
        })
        .filter_map(|root| {
            nearest_ancestor_workspace(&root.rel_dir, workspace_roots).map(|parent_workspace_rel| {
                G3RsTopologyNestedWorkspaceInput {
                    rel_dir: root.rel_dir.clone(),
                    cargo_rel_path: root.cargo_rel_path.clone(),
                    parent_workspace_rel: parent_workspace_rel.to_owned(),
                }
            })
        })
        .collect()
}

use std::collections::BTreeSet;

use g3rs_topology_types::{
    G3RsTopologyIllegalFamilyFilePlacementInput, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFile, G3RsTopologyWorkspaceFamilyFileAttachment,
    G3RsTopologyWorkspaceFamilyFileKind,
};

use super::paths::{display_dir, join_rel};

/// `classify_illegal_family_file` function.
pub(super) fn classify_illegal_family_file(
    file: &G3RsTopologyWorkspaceFamilyFile,
    workspace_root_rel: &str,
    actual_children: &BTreeSet<String>,
    legal_member_roots: &BTreeSet<String>,
) -> Option<G3RsTopologyIllegalFamilyFilePlacementInput> {
    if file.kind == G3RsTopologyWorkspaceFamilyFileKind::CargoToml {
        return None;
    }

    let reason = if file.family == G3RsTopologyWorkspaceFamily::Fmt {
        match &file.attachment {
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel }
                if root_rel == workspace_root_rel =>
            {
                return None;
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { .. }
            | G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot { .. } => {
                "fmt files must live at the validation root, not inside a workspace member or nested subdirectory.".to_owned()
            }
        }
    } else if file_is_legal_root_sidecar(file, workspace_root_rel) {
        return None;
    } else {
        match &file.attachment {
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel }
                if root_rel == workspace_root_rel =>
            {
                return None;
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel }
                if legal_member_roots.contains(root_rel) =>
            {
                format!(
                    "`{}` is attached to legal workspace member `{}`. Workspace-local `{}` files must live at the workspace root `{}` instead of inside a member crate.",
                    file.rel_path,
                    display_dir(root_rel),
                    family_label(file.family),
                    display_dir(workspace_root_rel),
                )
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot { root_rel } => {
                let root_label = if actual_children.contains(root_rel) {
                    format!("illegal child root `{}`", display_dir(root_rel))
                } else {
                    format!("non-member root `{}`", display_dir(root_rel))
                };
                format!(
                    "`{}` is attached to {root_label}. Workspace-local `{}` files must live at the workspace root `{}`.",
                    file.rel_path,
                    family_label(file.family),
                    display_dir(workspace_root_rel),
                )
            }
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot { owner_rel, .. } => {
                format!(
                    "`{}` is nested under `{}`. Workspace-local `{}` files must live directly at the workspace root `{}` rather than in nested subdirectories.",
                    file.rel_path,
                    display_dir(owner_rel),
                    family_label(file.family),
                    display_dir(workspace_root_rel),
                )
            }
        }
    };

    Some(G3RsTopologyIllegalFamilyFilePlacementInput {
        family: file.family,
        rel_path: file.rel_path.clone(),
        reason,
    })
}

/// `file_is_legal_root_sidecar` function.
fn file_is_legal_root_sidecar(
    file: &G3RsTopologyWorkspaceFamilyFile,
    workspace_root_rel: &str,
) -> bool {
    match (&file.kind, &file.attachment) {
        (
            G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml
            | G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy
            | G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml
            | G3RsTopologyWorkspaceFamilyFileKind::MutantsToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel,
                owner_rel,
            },
        ) => root_rel == workspace_root_rel && owner_rel == &join_rel(workspace_root_rel, ".cargo"),
        (
            G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
            G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
                root_rel,
                owner_rel,
            },
        ) => {
            root_rel == workspace_root_rel && owner_rel == &join_rel(workspace_root_rel, ".config")
        }
        _ => false,
    }
}

/// `family_label` function.
const fn family_label(family: G3RsTopologyWorkspaceFamily) -> &'static str {
    match family {
        G3RsTopologyWorkspaceFamily::Toolchain => "toolchain",
        G3RsTopologyWorkspaceFamily::Fmt => "fmt",
        G3RsTopologyWorkspaceFamily::Clippy => "clippy",
        G3RsTopologyWorkspaceFamily::Deny => "deny",
        G3RsTopologyWorkspaceFamily::Cargo => "cargo",
        G3RsTopologyWorkspaceFamily::Deps => "deps",
        G3RsTopologyWorkspaceFamily::Garde => "garde",
        G3RsTopologyWorkspaceFamily::Release => "release",
        G3RsTopologyWorkspaceFamily::Test => "test",
    }
}

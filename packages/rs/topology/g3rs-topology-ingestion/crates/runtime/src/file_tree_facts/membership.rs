#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::BTreeSet;

use g3rs_topology_types::{G3RsTopologyCargoManifestKind, G3RsTopologyFileTreeChecksInput};
use glob::Pattern;

use super::paths::{join_rel, nearest_ancestor_workspace};

/// `ResolvedMemberPattern` struct.
#[derive(Debug, Clone)]
pub(super) struct ResolvedMemberPattern {
    /// `raw` item.
    pub(super) raw: String,
    /// `matched_unresolved_dirs` item.
    pub(super) matched_unresolved_dirs: Vec<String>,
    /// `resolved_child_dirs` item.
    pub(super) resolved_child_dirs: Vec<String>,
}

/// `collect_escaping_member_patterns` function.
pub(super) fn collect_escaping_member_patterns(
    input: &G3RsTopologyFileTreeChecksInput,
) -> Vec<String> {
    input
        .workspace_manifest
        .workspace
        .as_ref()
        .map(|workspace| {
            workspace
                .members
                .iter()
                .filter(|member| member_pattern_escapes_root(member))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// `collect_member_patterns` function.
pub(super) fn collect_member_patterns(
    input: &G3RsTopologyFileTreeChecksInput,
    actual_children: &[String],
) -> Vec<ResolvedMemberPattern> {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    let unresolved_descendant_rels = input
        .descendant_cargo_roots
        .iter()
        .filter(|root| root.manifest_kind.is_none())
        .map(|root| root.rel_dir.clone())
        .collect::<Vec<_>>();

    input
        .workspace_manifest
        .workspace
        .as_ref()
        .map(|workspace| {
            workspace
                .members
                .iter()
                .filter(|member| !member_pattern_escapes_root(member))
                .map(|member| ResolvedMemberPattern {
                    raw: member.clone(),
                    matched_unresolved_dirs: resolve_member_pattern(
                        workspace_root_rel,
                        member,
                        &unresolved_descendant_rels,
                    ),
                    resolved_child_dirs: resolve_member_pattern(
                        workspace_root_rel,
                        member,
                        actual_children,
                    ),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// `collect_actual_children` function.
pub(super) fn collect_actual_children(
    input: &G3RsTopologyFileTreeChecksInput,
    workspace_roots: &BTreeSet<String>,
) -> Vec<String> {
    let workspace_root_rel = input.workspace_root_rel_dir.as_str();
    input
        .descendant_cargo_roots
        .iter()
        .filter(|root| root.rel_dir != workspace_root_rel)
        .filter(|root| root.manifest_kind == Some(G3RsTopologyCargoManifestKind::Package))
        .filter(|root| {
            nearest_ancestor_workspace(&root.rel_dir, workspace_roots) == Some(workspace_root_rel)
        })
        .map(|root| root.rel_dir.clone())
        .collect()
}

/// `resolve_member_pattern` function.
fn resolve_member_pattern(
    workspace_root_rel: &str,
    member: &str,
    descendant_root_rels: &[String],
) -> Vec<String> {
    let normalized = normalize_member_pattern(member);
    let pattern = join_rel(workspace_root_rel, &normalized);
    if !contains_glob_meta(&normalized) {
        return descendant_root_rels
            .iter()
            .filter(|rel_dir| *rel_dir == &pattern)
            .cloned()
            .collect();
    }

    let Ok(pattern) = Pattern::new(&pattern) else {
        return Vec::new();
    };
    descendant_root_rels
        .iter()
        .filter(|rel_dir| pattern.matches(rel_dir.as_str()))
        .cloned()
        .collect()
}

/// `member_pattern_escapes_root` function.
pub(super) fn member_pattern_escapes_root(member: &str) -> bool {
    member.starts_with('/')
        || member.starts_with('\\')
        || has_windows_drive_absolute_prefix(member)
        || member.split(['/', '\\']).any(|segment| segment == "..")
}

/// `has_windows_drive_absolute_prefix` function.
fn has_windows_drive_absolute_prefix(member: &str) -> bool {
    let bytes = member.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'/' || bytes[2] == b'\\')
}

/// `normalize_member_pattern` function.
fn normalize_member_pattern(member: &str) -> String {
    member
        .split('/')
        .filter(|segment| !segment.is_empty() && *segment != ".")
        .collect::<Vec<_>>()
        .join("/")
}

/// `contains_glob_meta` function.
fn contains_glob_meta(member: &str) -> bool {
    member.contains('*') || member.contains('?') || member.contains('[')
}

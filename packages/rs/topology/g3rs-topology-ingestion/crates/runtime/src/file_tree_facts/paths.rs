use g3rs_topology_types::G3RsTopologyWorkspaceMemberIssueKind;
use std::collections::BTreeSet;

/// `nearest_ancestor_workspace` function.
pub(super) fn nearest_ancestor_workspace<'a>(
    rel_dir: &str,
    workspace_rels: &'a BTreeSet<String>,
) -> Option<&'a str> {
    workspace_rels
        .iter()
        .filter(|workspace_rel| {
            workspace_rel.as_str() != rel_dir && path_is_under(rel_dir, workspace_rel)
        })
        .max_by_key(|workspace_rel| workspace_rel.len())
        .map(String::as_str)
}

/// `path_is_under` function.
pub(super) fn path_is_under(rel_path: &str, parent_rel: &str) -> bool {
    parent_rel.is_empty()
        || rel_path == parent_rel
        || rel_path
            .strip_prefix(parent_rel)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// `join_rel` function.
pub(super) fn join_rel(parent: &str, child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else if child.is_empty() {
        parent.to_owned()
    } else {
        format!("{parent}/{child}")
    }
}

/// `display_dir` function.
pub(super) const fn display_dir(rel_dir: &str) -> &str {
    if rel_dir.is_empty() { "." } else { rel_dir }
}

/// `membership_issue_sort_key` function.
pub(super) fn membership_issue_sort_key(
    kind: &G3RsTopologyWorkspaceMemberIssueKind,
) -> (&'static str, String) {
    match kind {
        G3RsTopologyWorkspaceMemberIssueKind::Undeclared { workspace_root_rel } => {
            ("undeclared-member", workspace_root_rel.clone())
        }
        G3RsTopologyWorkspaceMemberIssueKind::Extra {
            workspace_root_rel,
            member_pattern,
        } => (
            "extra-member",
            format!("{workspace_root_rel}:{member_pattern}"),
        ),
    }
}

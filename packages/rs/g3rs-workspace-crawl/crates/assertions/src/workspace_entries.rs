use g3rs_workspace_crawl_types::{
    G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
};

pub fn assert_has_rel_path(entries: &[G3RsWorkspaceEntry], rel_path: &str) {
    let _ = crate::common::require_rel_path(entries, rel_path);
}

pub fn assert_entry(
    entry: &G3RsWorkspaceEntry,
    kind: G3RsWorkspaceEntryKind,
    ignore_state: G3RsWorkspaceIgnoreState,
    readable: bool,
) {
    assert_eq!(entry.kind, kind, "unexpected entry kind: {entry:#?}");
    assert_eq!(
        entry.ignore_state, ignore_state,
        "unexpected ignore state: {entry:#?}"
    );
    assert_eq!(
        entry.readable, readable,
        "unexpected readability: {entry:#?}"
    );
}

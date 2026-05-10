use std::path::PathBuf;

use g3_workspace_crawl_assertions::query as assertions;

#[test]
fn supports_basic_queries_over_the_crawl() {
    let crawl = crate::G3WorkspaceCrawl {
        root_abs_path: PathBuf::from("/tmp/demo"),
        entries: vec![
            new_entry("Cargo.toml", crate::G3WorkspaceEntryKind::File),
            new_entry("src", crate::G3WorkspaceEntryKind::Directory),
            new_entry("src/lib.rs", crate::G3WorkspaceEntryKind::File),
            new_entry("src/main.rs", crate::G3WorkspaceEntryKind::File),
            new_entry("README.md", crate::G3WorkspaceEntryKind::File),
        ],
    };

    assertions::assert_root_file_exists(&crawl, "Cargo.toml");
    assertions::assert_extension_count(&crawl, "rs", 2);
    assertions::assert_entry_kind(&crawl, "src", crate::G3WorkspaceEntryKind::Directory);
    assertions::assert_has_rel_path(&crawl.entries, "README.md");
}

fn new_entry(rel_path: &str, kind: crate::G3WorkspaceEntryKind) -> crate::G3WorkspaceEntry {
    crate::G3WorkspaceEntry {
        path: crate::G3WorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: PathBuf::from("/tmp/demo").join(rel_path),
        },
        kind,
        ignore_state: crate::G3WorkspaceIgnoreState::Included,
        readable: true,
    }
}

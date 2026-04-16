use std::path::PathBuf;

use g3rs_workspace_crawl_assertions::query as assertions;

#[test]
fn supports_basic_queries_over_the_crawl() {
    let crawl = crate::G3RsWorkspaceCrawl {
        root_abs_path: PathBuf::from("/tmp/demo"),
        entries: vec![
            file("Cargo.toml"),
            dir("src"),
            file("src/lib.rs"),
            file("src/main.rs"),
            file("README.md"),
        ],
    };

    assertions::assert_root_file_exists(&crawl, "Cargo.toml");
    assertions::assert_extension_count(&crawl, "rs", 2);
    assertions::assert_entry_kind(&crawl, "src", crate::G3RsWorkspaceEntryKind::Directory);
    assertions::assert_has_rel_path(&crawl.entries, "README.md");
}

fn file(rel_path: &str) -> crate::G3RsWorkspaceEntry {
    entry(rel_path, crate::G3RsWorkspaceEntryKind::File)
}

fn dir(rel_path: &str) -> crate::G3RsWorkspaceEntry {
    entry(rel_path, crate::G3RsWorkspaceEntryKind::Directory)
}

fn entry(rel_path: &str, kind: crate::G3RsWorkspaceEntryKind) -> crate::G3RsWorkspaceEntry {
    crate::G3RsWorkspaceEntry {
        path: crate::G3RsWorkspacePath {
            rel_path: rel_path.to_owned(),
            abs_path: PathBuf::from("/tmp/demo").join(rel_path),
        },
        kind,
        ignore_state: crate::G3RsWorkspaceIgnoreState::Included,
        readable: true,
    }
}

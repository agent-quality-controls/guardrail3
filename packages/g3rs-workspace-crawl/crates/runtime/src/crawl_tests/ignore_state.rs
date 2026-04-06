use std::{fs, path::Path};

use g3rs_workspace_crawl_assertions::workspace_entries::{assert_entry, assert_has_rel_path};
use g3rs_workspace_crawl_types::{G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState};
use tempfile::tempdir;

#[test]
fn marks_gitignored_files_as_ignored() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();

    write(root.join(".gitignore"), "target/\nignored.txt\n");
    write(root.join("ignored.txt"), "nope\n");
    fs::create_dir_all(root.join("target")).expect("create ignored target directory");
    write(root.join("target/output.txt"), "compiled\n");
    write(root.join("visible.txt"), "ok\n");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    assert_has_rel_path(&crawl.entries, "ignored.txt");
    assert_entry(
        crate::support::ignored(&crawl.entries, "ignored.txt").expect("ignored file entry"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
    assert_entry(
        crate::support::ignored(&crawl.entries, "target").expect("ignored dir entry"),
        G3RsWorkspaceEntryKind::Directory,
        G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
    assert_entry(
        crawl.entry("visible.txt").expect("visible file entry"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

fn write(path: impl AsRef<Path>, content: &str) {
    fs::write(path, content).expect("write ignore-state fixture file");
}

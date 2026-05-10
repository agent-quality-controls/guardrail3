#![allow(
    clippy::disallowed_methods,
    reason = "fixture-driven filesystem tests need direct std::fs calls in test bodies"
)]

use std::fs;

use g3_workspace_crawl_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{git_init, write_fixture as write};

#[test]
fn entries_are_sorted_by_rel_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("z.rs"), "// z");
    write(root.join("a.rs"), "// a");
    write(root.join("m/b.rs"), "// b");

    let crawl = crate::run::crawl(root).expect("crawl should succeed for sort-order test");

    let rel_paths: Vec<&str> = crawl
        .entries
        .iter()
        .map(|e| e.path.rel_path.as_str())
        .filter(|p| !p.starts_with(".git"))
        .collect();

    assert_eq!(
        rel_paths,
        vec!["a.rs", "m", "m/b.rs", "z.rs"],
        "crawl entries should be sorted by rel_path in lexicographic order"
    );
}

#[cfg(unix)]
#[test]
fn symlinks_are_skipped() {
    use std::os::unix::fs::symlink;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("real.txt"), "real content");
    symlink(root.join("real.txt"), root.join("link.txt"))
        .expect("create symlink fixture pointing to real.txt");

    let crawl = crate::run::crawl(root).expect("crawl should succeed with symlinks present");

    assertions::assert_crawl_entry_exists(&crawl, "real.txt");
    assertions::assert_crawl_entry_absent(&crawl, "link.txt");
}

#[cfg(unix)]
#[test]
fn unreadable_file_has_readable_false() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("secret.txt"), "classified");
    write(root.join("normal.txt"), "public");

    let permissions = fs::Permissions::from_mode(0o000);
    fs::set_permissions(root.join("secret.txt"), permissions)
        .expect("chmod 000 should succeed on fixture file");

    let crawl = crate::run::crawl(root).expect("crawl should succeed even with unreadable files");

    assertions::assert_crawl_entry(
        &crawl,
        "secret.txt",
        crate::G3WorkspaceEntryKind::File,
        crate::G3WorkspaceIgnoreState::Included,
        false,
    );

    assertions::assert_crawl_entry(
        &crawl,
        "normal.txt",
        crate::G3WorkspaceEntryKind::File,
        crate::G3WorkspaceIgnoreState::Included,
        true,
    );
}

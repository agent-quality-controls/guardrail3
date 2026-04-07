use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn entries_are_sorted_by_rel_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("z.rs"), "// z");
    write(root.join("a.rs"), "// a");
    write(root.join("m/b.rs"), "// b");

    let crawl = crate::crawl(root).expect("crawl should succeed for sort-order test");

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

    let crawl = crate::crawl(root).expect("crawl should succeed with symlinks present");

    assert!(
        crawl.entry("real.txt").is_some(),
        "regular file real.txt should appear in crawl"
    );
    assert!(
        crawl.entry("link.txt").is_none(),
        "symlink link.txt should be skipped because follow_links is false"
    );
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

    let crawl = crate::crawl(root).expect("crawl should succeed even with unreadable files");

    let secret = crawl
        .entry("secret.txt")
        .expect("unreadable file secret.txt should still appear in crawl entries");
    assert!(
        !secret.readable,
        "secret.txt should have readable=false after chmod 000"
    );

    let normal = crawl
        .entry("normal.txt")
        .expect("normal.txt should appear in crawl entries");
    assert!(
        normal.readable,
        "normal.txt should have readable=true with default permissions"
    );
}

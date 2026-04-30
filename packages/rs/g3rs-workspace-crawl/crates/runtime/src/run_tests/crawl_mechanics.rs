use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_workspace_crawl_assertions::run as assertions;
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

fn write_root_manifest(root: &Path) {
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
}

#[test]
fn rejects_non_workspace_root_even_when_nested_rust_workspaces_exist() {
    let temp_dir = tempdir().expect("create temporary repository root");
    let root = temp_dir.path();
    git_init(root);

    write(
        root.join("apps/service/Cargo.toml"),
        "[package]\nname = \"service\"\n",
    );
    write(root.join("apps/service/src/lib.rs"), "");

    let error = crate::run::crawl(root).expect_err("repo root without Cargo.toml should fail");

    assert!(
        matches!(
            error,
            crate::run::G3RsWorkspaceCrawlError::MissingWorkspaceManifest(_)
        ),
        "expected missing root Cargo.toml error, got {error:?}",
    );
}

#[test]
fn crawl_any_root_accepts_non_rust_project_roots() {
    let temp_dir = tempdir().expect("create temporary TypeScript app root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("package.json"), "{}\n");
    write(root.join("src/index.ts"), "export {};\n");

    let crawl = crate::run::crawl_any_root(root).expect("crawl any project root should succeed");

    assertions::assert_crawl_entry_exists(&crawl, "package.json");
    assertions::assert_crawl_entry_exists(&crawl, "src/index.ts");
}

#[test]
fn entries_are_sorted_by_rel_path() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);
    write_root_manifest(root);

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
        vec!["Cargo.toml", "a.rs", "m", "m/b.rs", "z.rs"],
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
    write_root_manifest(root);

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
    write_root_manifest(root);

    write(root.join("secret.txt"), "classified");
    write(root.join("normal.txt"), "public");

    let permissions = fs::Permissions::from_mode(0o000);
    fs::set_permissions(root.join("secret.txt"), permissions)
        .expect("chmod 000 should succeed on fixture file");

    let crawl = crate::run::crawl(root).expect("crawl should succeed even with unreadable files");

    assertions::assert_crawl_entry(
        &crawl,
        "secret.txt",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        false,
    );

    assertions::assert_crawl_entry(
        &crawl,
        "normal.txt",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

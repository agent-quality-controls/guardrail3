use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_workspace_crawl_assertions::workspace_entries::{assert_entry, assert_has_rel_path};
use g3rs_workspace_crawl_types::{G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState};
use tempfile::tempdir;

/// Initialize a git repo at the given path so the ignore crate's WalkBuilder
/// can find .gitignore files and compute ignore state.
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
fn marks_gitignored_files_as_included_via_recovery() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "Cargo.lock\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(root.join("Cargo.lock"), "# lock\n");
    write(root.join("src/lib.rs"), "");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    // Cargo.lock is ignored but recoverable — should appear as Ignored
    assert_has_rel_path(&crawl.entries, "Cargo.lock");
    assert_entry(
        crawl.entry("Cargo.lock").expect("Cargo.lock should be recovered from ignored space because it is on the recovery list"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Ignored,
        true,
    );

    // Cargo.toml is not ignored
    assert_entry(
        crawl.entry("Cargo.toml").expect("Cargo.toml should be present as an included entry in the crawl result"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn ignored_non_recoverable_files_do_not_appear() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "*.log\n");
    write(root.join("debug.log"), "some log\n");
    write(root.join("src/lib.rs"), "");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    // debug.log is ignored and not on the recovery list — absent
    assert!(
        crawl.entry("debug.log").is_none(),
        "ignored non-recoverable file should not appear in crawl"
    );
}

#[test]
fn nested_gitignore_is_respected() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join("src/lib.rs"), "");
    write(root.join("src/.gitignore"), "*.tmp\n");
    write(root.join("src/temp.tmp"), "junk");
    write(root.join("root.tmp"), "also tmp but not ignored by nested rule");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    // src/temp.tmp is ignored by src/.gitignore — not recoverable, absent
    assert!(
        crawl.entry("src/temp.tmp").is_none(),
        "file ignored by nested .gitignore should not appear"
    );

    // root.tmp is NOT ignored (the nested .gitignore only applies to src/)
    assert_entry(
        crawl.entry("root.tmp").expect("root.tmp should be included"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn ancestor_gitignore_is_respected() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let repo_root = temp_dir.path();
    git_init(repo_root);

    // Parent .gitignore at repo root ignores *.generated
    write(repo_root.join(".gitignore"), "*.generated\n");

    // Workspace is a subdirectory of the repo
    let workspace = repo_root.join("packages/demo");
    fs::create_dir_all(&workspace).expect("create workspace dir");
    write(workspace.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    write(workspace.join("src/lib.rs"), "");
    write(workspace.join("output.generated"), "generated file");

    let crawl = crate::crawl(&workspace).expect("crawl should succeed");

    // output.generated is ignored by ancestor .gitignore — not recoverable, absent
    assert!(
        crawl.entry("output.generated").is_none(),
        "file ignored by ancestor .gitignore should not appear"
    );

    // Cargo.toml is included
    assert_entry(
        crawl.entry("Cargo.toml").expect("Cargo.toml should be present as an included entry in the crawl result"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn negation_pattern_unignores_file() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "*.log\n!important.log\n");
    write(root.join("debug.log"), "ignored log");
    write(root.join("important.log"), "keep this");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    // important.log is unignored by negation
    assert_entry(
        crawl
            .entry("important.log")
            .expect("important.log should be included via negation"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );

    // debug.log is ignored and not recoverable — absent
    assert!(
        crawl.entry("debug.log").is_none(),
        "debug.log should not appear"
    );
}

#[test]
fn hidden_dotfiles_are_included_normally() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".rustfmt.toml"), "edition = \"2024\"\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    assert_entry(
        crawl.entry(".clippy.toml").expect(".clippy.toml should be included as a normal dotfile entry"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
    assert_entry(
        crawl.entry(".rustfmt.toml").expect(".rustfmt.toml should be included as a normal dotfile entry"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
}

#[test]
fn banned_directories_are_excluded_from_recovery() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    write(root.join(".gitignore"), "target/\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");
    // Cargo.toml inside target/ should NOT be recovered
    fs::create_dir_all(root.join("target")).expect("create target dir");
    write(
        root.join("target/Cargo.toml"),
        "[package]\nname = \"build-artifact\"\n",
    );

    let crawl = crate::crawl(root).expect("crawl should succeed");

    assert!(
        crawl.entry("target/Cargo.toml").is_none(),
        "Cargo.toml inside banned target/ should not be recovered"
    );
    assert!(
        crawl.entry("target").is_none(),
        "banned target/ directory should not appear"
    );
}

#[test]
fn recovery_finds_ignored_config_in_non_banned_directory() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);

    // .cargo/ is ignored but not banned
    write(root.join(".gitignore"), ".cargo/\n");
    fs::create_dir_all(root.join(".cargo")).expect("create .cargo dir");
    write(root.join(".cargo/config.toml"), "[build]\njobs = 4\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\n");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    // .cargo/config.toml is ignored but recoverable
    assert_has_rel_path(&crawl.entries, ".cargo/config.toml");
    assert_entry(
        crawl
            .entry(".cargo/config.toml")
            .expect(".cargo/config.toml should be recovered from ignored .cargo/ directory because it is on the recovery list"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Ignored,
        true,
    );
}

use std::fs;

use g3rs_toolchain_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{crawl, git_init, write};

#[test]
fn filetree_ingests_root_toolchain_paths() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rust-toolchain"), "stable\n");

    let output = super::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

    assert_eq!(
        output.toolchain_toml_rel_path.as_deref(),
        Some("rust-toolchain.toml")
    );
    assert_eq!(
        output.legacy_toolchain_rel_path.as_deref(),
        Some("rust-toolchain")
    );
}

#[test]
fn filetree_ignores_descendant_toolchain_files() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("nested/rust-toolchain.toml"),
        "[toolchain]\nchannel = \"beta\"\n",
    );
    write(root.join("nested/rust-toolchain"), "stable\n");

    let output = super::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed");

    assert_eq!(output.toolchain_toml_rel_path, None);
    assert_eq!(output.legacy_toolchain_rel_path, None);
}

#[test]
fn pipeline_reports_missing_modern_toolchain_file() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let input = super::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed without toolchain files");
    let results = g3rs_toolchain_filetree_checks::check(&input);
    assertions::assert_missing_modern_toolchain(&results);
}

#[test]
fn pipeline_reports_legacy_only_as_warn_plus_missing_modern() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rust-toolchain"), "stable\n");

    let input = super::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed with legacy toolchain only");
    let results = g3rs_toolchain_filetree_checks::check(&input);
    assertions::assert_legacy_only_without_modern(&results);
}

#[test]
fn pipeline_reports_both_toolchain_files_conflict() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.join("rust-toolchain"), "stable\n");

    let input = super::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should succeed when both files exist");
    let results = g3rs_toolchain_filetree_checks::check(&input);
    assertions::assert_both_toolchain_files_present(&results);
}

#[test]
fn pipeline_reports_root_files_even_when_contents_are_malformed() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rust-toolchain.toml"), "{{{{bad toml");
    write(root.join("rust-toolchain"), "not parsed anyway\n");

    let input = super::ingest_for_file_tree_checks(&crawl(root))
        .expect("filetree ingestion should ignore malformed file contents");
    let results = g3rs_toolchain_filetree_checks::check(&input);
    assertions::assert_both_toolchain_files_present(&results);
}

#[test]
fn filetree_uses_unreadable_root_entries_without_reading_them() {
    use g3rs_workspace_crawl::{
        G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
        G3RsWorkspacePath,
    };

    let crawl = G3RsWorkspaceCrawl {
        root_abs_path: std::path::PathBuf::from("/synthetic/workspace"),
        entries: vec![
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: "rust-toolchain.toml".to_owned(),
                    abs_path: std::path::PathBuf::from("/synthetic/workspace/rust-toolchain.toml"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: false,
            },
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: "rust-toolchain".to_owned(),
                    abs_path: std::path::PathBuf::from("/synthetic/workspace/rust-toolchain"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: false,
            },
        ],
    };

    let input = super::ingest_for_file_tree_checks(&crawl)
        .expect("filetree ingestion should not fail on unreadable entries");
    let results = g3rs_toolchain_filetree_checks::check(&input);
    assertions::assert_both_toolchain_files_present(&results);
}

#[test]
fn filetree_still_reports_root_file_after_delete_after_crawl() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );

    let crawl = crawl(root);
    fs::remove_file(root.join("rust-toolchain.toml"))
        .expect("should delete rust-toolchain.toml after crawl");

    let input = super::ingest_for_file_tree_checks(&crawl)
        .expect("filetree ingestion should not reread the filesystem");
    let results = g3rs_toolchain_filetree_checks::check(&input);
    assertions::assert_modern_toolchain_exists(&results);
}

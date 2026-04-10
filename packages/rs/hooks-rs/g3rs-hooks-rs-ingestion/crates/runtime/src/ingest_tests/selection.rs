use std::fs;

use g3rs_hooks_rs_ingestion_types::G3RsHooksRsIngestionError;
use g3rs_workspace_crawl::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
    G3RsWorkspacePath,
};
use tempfile::tempdir;

fn write(path: impl AsRef<std::path::Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture");
}

fn unreadable_file_crawl(root: &std::path::Path) -> G3RsWorkspaceCrawl {
    G3RsWorkspaceCrawl {
        root_abs_path: root.to_path_buf(),
        entries: vec![G3RsWorkspaceEntry {
            path: G3RsWorkspacePath {
                rel_path: ".githooks/pre-commit".to_owned(),
                abs_path: root.join(".githooks/pre-commit"),
            },
            kind: G3RsWorkspaceEntryKind::File,
            ignore_state: G3RsWorkspaceIgnoreState::Included,
            readable: false,
        }],
    }
}

#[test]
fn prefers_githooks_pre_commit_over_hooks_pre_commit() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, ".githooks/pre-commit");
    assert!(inputs[0].content.contains("cargo fmt --check"));
}

#[test]
fn falls_back_to_hooks_pre_commit_when_githooks_script_is_absent() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, "hooks/pre-commit");
    assert!(inputs[0].content.contains("cargo test --workspace"));
}

#[test]
fn returns_empty_when_no_supported_pre_commit_exists() {
    let temp_dir = tempdir().expect("create temp dir");
    let crawl = g3rs_workspace_crawl::crawl(temp_dir.path()).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    assert!(inputs.is_empty());
}

#[test]
fn returns_unreadable_when_selected_pre_commit_cannot_be_read() {
    let temp_dir = tempdir().expect("create temp dir");
    let crawl = unreadable_file_crawl(temp_dir.path());

    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksRsIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, temp_dir.path().join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

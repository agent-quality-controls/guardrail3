use std::fs;

use g3rs_hooks_shared_ingestion_types::{G3RsHookScriptKind, G3RsHooksSharedIngestionError};
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
fn prefers_githooks_pre_commit_and_includes_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");
    write(root.join("hooks/pre-commit"), "echo fallback\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "cargo fmt --check\n");
    write(root.join(".githooks/pre-commit.d/20-extra.sh"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let rel_paths = inputs
        .iter()
        .map(|input| input.rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        rel_paths,
        vec![
            ".githooks/pre-commit",
            ".githooks/pre-commit.d/10-rust.sh",
            ".githooks/pre-commit.d/20-extra.sh",
        ]
    );
    assert_eq!(inputs[0].kind, G3RsHookScriptKind::PreCommit);
    assert!(inputs[0].has_modular_dir);
    assert_eq!(inputs[1].kind, G3RsHookScriptKind::Modular);
    assert_eq!(inputs[2].kind, G3RsHookScriptKind::Modular);
}

#[test]
fn falls_back_to_hooks_pre_commit_when_githooks_script_is_absent() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join("hooks/pre-commit"), "echo fallback\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, "hooks/pre-commit");
    assert_eq!(inputs[0].kind, G3RsHookScriptKind::PreCommit);
    assert!(!inputs[0].has_modular_dir);
}

#[test]
fn returns_unreadable_when_modular_script_cannot_be_read() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();
    let unreadable_path = root.join(".githooks/pre-commit.d/10-rust.sh");

    let crawl = G3RsWorkspaceCrawl {
        root_abs_path: root.to_path_buf(),
        entries: vec![
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: ".githooks/pre-commit".to_owned(),
                    abs_path: root.join(".githooks/pre-commit"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: true,
            },
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: ".githooks/pre-commit.d".to_owned(),
                    abs_path: root.join(".githooks/pre-commit.d"),
                },
                kind: G3RsWorkspaceEntryKind::Directory,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: true,
            },
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: ".githooks/pre-commit.d/10-rust.sh".to_owned(),
                    abs_path: unreadable_path.clone(),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: false,
            },
        ],
    };

    fs::create_dir_all(root.join(".githooks")).expect("create githooks dir");
    fs::write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n")
        .expect("write pre-commit");

    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksSharedIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, unreadable_path);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn returns_unreadable_when_selected_pre_commit_cannot_be_read() {
    let temp_dir = tempdir().expect("create temp dir");
    let crawl = unreadable_file_crawl(temp_dir.path());

    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksSharedIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, temp_dir.path().join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

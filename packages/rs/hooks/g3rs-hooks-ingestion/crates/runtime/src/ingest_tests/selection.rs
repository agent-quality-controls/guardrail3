use std::fs;
use std::process::Command;

use g3rs_hooks_ingestion_types::{G3RsHookScriptKind, G3RsHooksIngestionError};
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

fn git_init(path: &std::path::Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn repo_root(temp_dir: &tempfile::TempDir) -> &std::path::Path {
    let root = temp_dir.path();
    git_init(root);
    root
}

fn git_config_hooks_path(path: &std::path::Path, hooks_path: &str) {
    let status = Command::new("git")
        .args(["config", "core.hooksPath", hooks_path])
        .current_dir(path)
        .status()
        .expect("git config should succeed");
    assert!(status.success(), "git config should exit successfully");
}

fn break_git_dir(path: &std::path::Path) {
    fs::rename(path.join(".git"), path.join(".git-real")).expect("rename git dir");
    fs::write(path.join(".git"), "gitdir: /missing/hooks-test-gitdir\n")
        .expect("write broken gitdir file");
}

#[cfg(unix)]
fn make_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt as _;

    let mut permissions = fs::metadata(path).expect("metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("set executable bit");
}

fn unreadable_file_crawl(root: &std::path::Path) -> G3RsWorkspaceCrawl {
    git_init(root);
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
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, ".githooks/pre-commit");
    assert_eq!(inputs[0].kind, G3RsHookScriptKind::PreCommit);
    assert!(inputs[0]
        .parsed
        .source_lines
        .iter()
        .any(|line| line.raw.contains("cargo fmt --check")));
    assert!(!inputs[0].has_modular_dir);
    assert!(!inputs[0].is_workspace_project);
}

#[test]
fn falls_back_to_hooks_pre_commit_when_githooks_script_is_absent() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, "hooks/pre-commit");
    assert_eq!(inputs[0].kind, G3RsHookScriptKind::PreCommit);
    assert!(inputs[0]
        .parsed
        .source_lines
        .iter()
        .any(|line| line.raw.contains("cargo test --workspace")));
    assert!(!inputs[0].has_modular_dir);
    assert!(!inputs[0].is_workspace_project);
}

#[test]
fn honors_core_hooks_path_when_hooks_pre_commit_is_configured() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "cargo clippy -- -D warnings\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, "hooks/pre-commit");
    assert!(!inputs[0].has_modular_dir);
}

#[test]
fn honors_normalized_core_hooks_path_variants() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "./hooks/");

    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let file_tree = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, "hooks/pre-commit");
    assert_eq!(
        file_tree.pre_commit.as_ref().map(|fact| fact.rel_path.as_str()),
        Some("hooks/pre-commit")
    );
    assert_eq!(file_tree.hooks_path, Some("./hooks/".to_owned()));
}

#[test]
fn keeps_non_compat_hooks_path_outside_owned_hook_surface() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "custom-hooks");

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    write(root.join("hooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let source_inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let config_input =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect("ingestion should succeed");
    let file_tree_input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");

    assert!(source_inputs.is_empty());
    assert!(config_input.selected_hook.is_none());
    assert!(file_tree_input.pre_commit.is_none());
    assert_eq!(file_tree_input.hooks_path, Some("custom-hooks".to_owned()));
}

#[test]
fn prefers_githooks_pre_commit_and_includes_direct_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

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
fn fallback_hook_does_not_activate_githooks_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("hooks/pre-commit"), "cargo fmt --check\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0].rel_path, "hooks/pre-commit");
    assert!(!inputs[0].has_modular_dir);
}

#[test]
fn ignores_nested_files_under_pre_commit_d() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "cargo fmt --check\n");
    write(
        root.join(".githooks/pre-commit.d/nested/20-hidden.sh"),
        "cargo test --workspace\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let rel_paths = inputs
        .iter()
        .map(|input| input.rel_path.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        rel_paths,
        vec![".githooks/pre-commit", ".githooks/pre-commit.d/10-rust.sh"]
    );
}

#[test]
fn marks_inputs_as_workspace_projects_when_root_manifest_has_workspace_section() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join(".githooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 1);
    assert!(inputs[0].is_workspace_project);
}

#[test]
fn fails_closed_when_root_manifest_is_malformed_for_hooked_repo() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace\n");
    write(root.join(".githooks/pre-commit"), "cargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");

    match error {
        G3RsHooksIngestionError::ParseFailed { path, .. } => {
            assert_eq!(path, root.join("Cargo.toml"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn ignores_malformed_root_manifest_when_no_hook_exists() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should stay quiet");

    assert!(inputs.is_empty());
}

#[test]
fn source_ingestion_fails_closed_when_git_hooks_path_lookup_errors() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    break_git_dir(root);
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");

    match error {
        G3RsHooksIngestionError::ParseFailed { path, .. } => {
            assert_eq!(path, root.join(".git/config"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
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
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, temp_dir.path().join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn fails_closed_when_selected_pre_commit_disappears_after_crawl() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    fs::remove_file(root.join(".githooks/pre-commit")).expect("remove selected hook");

    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, root.join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn returns_unreadable_when_modular_script_cannot_be_read() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
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
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, unreadable_path);
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn fails_closed_when_selected_modular_script_disappears_after_crawl() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "cargo fmt --check\n");
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    fs::remove_file(root.join(".githooks/pre-commit.d/10-rust.sh"))
        .expect("remove modular script");

    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, root.join(".githooks/pre-commit.d/10-rust.sh"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn fails_closed_when_root_manifest_disappears_after_crawl_for_hooked_repo() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join(".githooks/pre-commit"), "cargo test --workspace\n");
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    fs::remove_file(root.join("Cargo.toml")).expect("remove root manifest");

    let error = crate::ingest_for_source_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, root.join("Cargo.toml"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn config_ingestion_selects_effective_hook_and_detects_installed_tools() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    let bin_dir = root.join("bin");

    write(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");
    write(root.join("hooks/pre-commit"), "cargo fmt --check\n");
    write(bin_dir.join("g3rs"), "#!/usr/bin/env bash\n");
    write(bin_dir.join("gitleaks"), "#!/usr/bin/env bash\n");
    #[cfg(unix)]
    {
        make_executable(&bin_dir.join("g3rs"));
        make_executable(&bin_dir.join("gitleaks"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let path_env = std::env::join_paths([bin_dir.as_path()]).expect("join path");
    let input = crate::run::ingest_for_config_checks_with_path(&crawl, Some(path_env.as_os_str()))
        .expect("ingestion should succeed");

    let selected_hook = input.selected_hook.expect("selected hook");
    assert_eq!(selected_hook.rel_path, ".githooks/pre-commit");
    assert!(selected_hook
        .parsed
        .source_lines
        .iter()
        .any(|line| line.raw.contains("g3rs validate --path")));
    assert_eq!(input.installed_tools, vec!["g3rs".to_owned(), "gitleaks".to_owned()]);
}

#[test]
fn config_ingestion_fails_closed_when_selected_pre_commit_cannot_be_read() {
    let temp_dir = tempdir().expect("create temp dir");
    let crawl = unreadable_file_crawl(temp_dir.path());

    let error =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, temp_dir.path().join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn config_ingestion_fails_closed_when_selected_pre_commit_disappears_after_crawl() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    fs::remove_file(root.join(".githooks/pre-commit")).expect("remove selected hook");

    let error =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, root.join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn config_ingestion_honors_core_hooks_path_and_path_qualified_tools() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");
    write(
        root.join("hooks/pre-commit"),
        "/opt/bin/gitleaks protect --staged --no-banner\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect("ingestion should succeed");

    let selected_hook = input.selected_hook.expect("selected hook");
    assert_eq!(selected_hook.rel_path, "hooks/pre-commit");
}

#[test]
fn config_ingestion_fails_closed_when_git_hooks_path_lookup_errors() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "g3rs validate --path .\n");
    break_git_dir(root);
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error =
        crate::run::ingest_for_config_checks_with_path(&crawl, None).expect_err("should fail closed");

    match error {
        G3RsHooksIngestionError::ParseFailed { path, .. } => {
            assert_eq!(path, root.join(".git/config"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn config_ingestion_stays_quiet_without_hook() {
    let temp_dir = tempdir().expect("create temp dir");
    let crawl = g3rs_workspace_crawl::crawl(temp_dir.path()).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks_with_path(&crawl, None)
        .expect("ingestion should succeed");

    assert!(input.selected_hook.is_none());
    assert!(input.installed_tools.is_empty());
}

#[test]
fn file_tree_ingestion_collects_hook_layout_and_trust_surface() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, ".githooks");

    write(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\nrun-parts .githooks/pre-commit.d\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(
        root.join(".guardrail3/overrides/pre-commit.d/90-local.sh"),
        "#!/usr/bin/env bash\necho override\n",
    );
    write(root.join(".husky/pre-commit"), "echo shadow\n");
    #[cfg(unix)]
    {
        make_executable(&root.join(".githooks/pre-commit"));
        make_executable(&root.join(".githooks/pre-commit.d/10-rust.sh"));
    }

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");

    let pre_commit = input.pre_commit.expect("pre-commit fact");
    assert_eq!(pre_commit.rel_path, ".githooks/pre-commit");
    assert_eq!(pre_commit.line_count, 2);
    assert_eq!(pre_commit.byte_count, 53);
    assert_eq!(pre_commit.executable, Some(true));
    assert!(input.has_modular_dir);
    assert_eq!(input.modular_scripts.len(), 1);
    assert_eq!(input.modular_scripts[0].rel_path, ".githooks/pre-commit.d/10-rust.sh");
    assert_eq!(
        input.local_override_scripts,
        vec!["90-local.sh".to_owned()]
    );
    assert_eq!(input.hooks_path, Some(".githooks".to_owned()));
    assert_eq!(input.trust_risks, vec![".husky/pre-commit".to_owned()]);
}

#[test]
fn file_tree_ingestion_collects_modular_layout_for_hooks_compat_path() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\nrun-parts .githooks/pre-commit.d\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo test --workspace\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(input.pre_commit.as_ref().map(|fact| fact.rel_path.as_str()), Some("hooks/pre-commit"));
    assert!(input.has_modular_dir);
    assert_eq!(input.modular_scripts.len(), 1);
    assert_eq!(input.modular_scripts[0].rel_path, ".githooks/pre-commit.d/10-rust.sh");
}

#[test]
fn file_tree_ingestion_reports_git_hook_shadow_when_hooks_path_is_wrong() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(root.join(".git/hooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(input.pre_commit.as_ref().map(|fact| fact.rel_path.as_str()), Some("hooks/pre-commit"));
    assert_eq!(input.trust_risks, vec![".git/hooks/pre-commit".to_owned()]);
}

#[test]
fn file_tree_ingestion_does_not_report_git_hook_shadow_for_hooks_path_compat_mode() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);
    git_config_hooks_path(root, "hooks");

    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    write(root.join(".git/hooks/pre-commit"), "#!/usr/bin/env bash\nexit 0\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(input.pre_commit.as_ref().map(|fact| fact.rel_path.as_str()), Some("hooks/pre-commit"));
    assert!(input.trust_risks.is_empty());
}

#[test]
fn file_tree_ingestion_fails_closed_when_git_hooks_path_lookup_errors() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "#!/usr/bin/env bash\ncargo fmt --check\n");
    break_git_dir(root);
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let error = crate::ingest_for_file_tree_checks(&crawl).expect_err("should fail closed");

    match error {
        G3RsHooksIngestionError::ParseFailed { path, .. } => {
            assert_eq!(path, root.join(".git/config"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn file_tree_ingestion_fails_closed_when_selected_pre_commit_cannot_be_read() {
    let temp_dir = tempdir().expect("create temp dir");
    let crawl = unreadable_file_crawl(temp_dir.path());

    let error = crate::ingest_for_file_tree_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, temp_dir.path().join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn file_tree_ingestion_fails_closed_when_selected_pre_commit_disappears_after_crawl() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    fs::remove_file(root.join(".githooks/pre-commit")).expect("remove selected hook");

    let error = crate::ingest_for_file_tree_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, root.join(".githooks/pre-commit"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn file_tree_ingestion_fails_closed_when_selected_modular_script_disappears_after_crawl() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");
    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "cargo fmt --check\n");
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    fs::remove_file(root.join(".githooks/pre-commit.d/10-rust.sh"))
        .expect("remove modular script");

    let error = crate::ingest_for_file_tree_checks(&crawl).expect_err("should fail closed");
    match error {
        G3RsHooksIngestionError::Unreadable { path, .. } => {
            assert_eq!(path, root.join(".githooks/pre-commit.d/10-rust.sh"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

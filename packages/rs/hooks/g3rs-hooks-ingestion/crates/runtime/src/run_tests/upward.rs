use std::fs;

use g3rs_hooks_types::G3RsHookScriptKind;
use tempfile::tempdir;

use super::helpers::{git_init, write_fixture};

/// Workspace-local hook (existing behavior): when `.githooks/pre-commit`
/// lives inside the crawled workspace, ingestion finds it and the rel_path
/// is workspace-relative.
#[test]
fn workspace_local_hook_is_ingested() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    let workspace = repo.join("apps/some-app");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join(".githooks/pre-commit"), "echo local\n");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let pre_commit = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::PreCommit)
        .expect("pre-commit should be ingested from workspace-local path");
    assert_eq!(pre_commit.rel_path, ".githooks/pre-commit");
    assert!(pre_commit.exists);
}

/// Ancestor hook: when `.githooks/pre-commit` does not live inside the
/// crawled workspace but lives at an ancestor of the workspace, ingestion
/// walks upward to find it. The rel_path reflects the ancestor-relative
/// location, not the workspace.
#[test]
fn ancestor_hook_is_ingested_via_upward_walk() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    write_fixture(repo.join(".githooks/pre-commit"), "echo ancestor\n");

    let workspace = repo.join("apps/sub-rs");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let pre_commit = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::PreCommit)
        .expect("pre-commit should be ingested from ancestor via upward walk");
    assert_eq!(pre_commit.rel_path, ".githooks/pre-commit");
    assert!(pre_commit.exists);
}

/// No hook anywhere: when neither the workspace nor any ancestor contains
/// `.githooks/pre-commit`, ingestion reports no pre-commit input. The
/// scripts/g3rs/verify missing-fact still ships, matching prior behavior.
#[test]
fn missing_hook_anywhere_reports_no_pre_commit_input() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    let workspace = repo.join("apps/no-hook");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert!(
        inputs
            .iter()
            .all(|input| input.kind != G3RsHookScriptKind::PreCommit),
        "no pre-commit input should be produced when no hook exists workspace-local or upward"
    );
}

/// Workspace-local verifier (existing behavior): when
/// `scripts/g3rs/verify` lives inside the crawled workspace, ingestion
/// finds it and the rel_path is workspace-relative.
#[test]
fn workspace_local_verifier_is_ingested() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    let workspace = repo.join("apps/some-app");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join("scripts/g3rs/verify"), "echo local\n");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let verifier = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::G3RsVerifier)
        .expect("verifier should be ingested from workspace-local path");
    assert_eq!(verifier.rel_path, "scripts/g3rs/verify");
    assert!(verifier.exists);
}

/// Ancestor verifier: when `scripts/g3rs/verify` does not live inside the
/// crawled workspace but lives at an ancestor of the workspace, ingestion
/// walks upward to find it. The rel_path reflects the ancestor-relative
/// location, not the workspace.
#[test]
fn ancestor_verifier_is_ingested_via_upward_walk() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    write_fixture(repo.join("scripts/g3rs/verify"), "echo ancestor\n");

    let workspace = repo.join("apps/sub-rs");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let verifier = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::G3RsVerifier)
        .expect("verifier should be ingested from ancestor via upward walk");
    assert_eq!(verifier.rel_path, "scripts/g3rs/verify");
    assert!(verifier.exists);
}

/// No verifier anywhere: when neither the workspace nor any ancestor
/// contains `scripts/g3rs/verify`, ingestion reports a missing verifier
/// fact (existing behavior).
#[test]
fn missing_verifier_anywhere_reports_not_exists() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    let workspace = repo.join("apps/no-verifier");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let verifier = inputs
        .iter()
        .find(|input| input.kind == G3RsHookScriptKind::G3RsVerifier)
        .expect("verifier missing-fact should still ship when not found anywhere");
    assert_eq!(verifier.rel_path, "scripts/g3rs/verify");
    assert!(!verifier.exists);
}

/// File-tree ingestion also walks upward for the modular hook directory and
/// surfaces direct script children of the ancestor `.githooks/pre-commit.d/`.
#[test]
fn ancestor_modular_dir_is_ingested_via_upward_walk() {
    let temp_dir = tempdir().expect("create temp dir");
    let repo = temp_dir.path();
    git_init(repo);

    write_fixture(repo.join(".githooks/pre-commit"), "echo ancestor\n");
    write_fixture(
        repo.join(".githooks/pre-commit.d/10-rust.sh"),
        "echo modular\n",
    );

    let workspace = repo.join("apps/sub-rs");
    fs::create_dir_all(workspace.as_path()).expect("create workspace dir");
    write_fixture(workspace.join("Cargo.toml"), "[workspace]\n");

    let crawl =
        g3_workspace_crawl::crawl_any_root(workspace.as_path()).expect("crawl should succeed");
    let input = super::super::ingest_for_file_tree_checks(&crawl)
        .expect("file-tree ingestion should succeed");

    assert!(input.active);
    assert!(input.has_modular_dir);
    let pre_commit = input
        .pre_commit
        .as_ref()
        .expect("pre-commit fact should be ingested via upward walk");
    assert_eq!(pre_commit.rel_path, ".githooks/pre-commit");
    let modular_paths = input
        .modular_scripts
        .iter()
        .map(|fact| fact.rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(modular_paths, vec![".githooks/pre-commit.d/10-rust.sh"]);
}

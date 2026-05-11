use g3rs_hooks_types::G3RsHookScriptKind;
use tempfile::tempdir;

use super::helpers::{repo_root, write_fixture};

#[test]
fn ingests_pre_commit_and_g3rs_verifier_only_for_source_checks() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "scripts/g3rs/verify\n");
    write_fixture(
        root.join("scripts/g3rs/verify"),
        "g3rs validate --path \"$SCOPE\"\n",
    );
    write_fixture(
        root.join("scripts/g3ts/verify"),
        "g3ts validate --path \"$SCOPE\"\n",
    );
    write_fixture(root.join("scripts/guardrails/verify"), "echo shared\n");
    write_fixture(
        root.join(".githooks/pre-commit.d/10-rust.sh"),
        "cargo fmt\n",
    );
    write_fixture(root.join("hooks/pre-commit"), "cargo test\n");

    let crawl = g3_workspace_crawl::crawl_any_root(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    let rel_paths = inputs
        .iter()
        .map(|input| input.rel_path.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        rel_paths,
        vec![".githooks/pre-commit", "scripts/g3rs/verify"]
    );
    assert_eq!(inputs[0].kind, G3RsHookScriptKind::PreCommit);
    assert_eq!(inputs[1].kind, G3RsHookScriptKind::G3RsVerifier);
    assert!(inputs.iter().all(|input| input.exists));
}

#[test]
fn emits_missing_g3rs_verifier_fact_without_ingesting_g3ts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join(".githooks/pre-commit"), "scripts/g3rs/verify\n");
    write_fixture(
        root.join("scripts/g3ts/verify"),
        "g3ts validate --path \"$SCOPE\"\n",
    );

    let crawl = g3_workspace_crawl::crawl_any_root(root).expect("crawl should succeed");
    let inputs = super::super::ingest_for_source_checks(&crawl).expect("ingestion should succeed");

    assert_eq!(inputs.len(), 2);
    assert_eq!(inputs[1].rel_path, "scripts/g3rs/verify");
    assert_eq!(inputs[1].kind, G3RsHookScriptKind::G3RsVerifier);
    assert!(!inputs[1].exists);
    assert!(
        inputs
            .iter()
            .all(|input| input.rel_path != "scripts/g3ts/verify")
    );
}

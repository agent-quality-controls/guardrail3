use std::fs;

use tempfile::tempdir;

fn write(path: impl AsRef<std::path::Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture");
}

#[test]
fn pipeline_reports_dispatcher_findings_for_real_pre_commit_script() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    fs::create_dir_all(root.join(".githooks/pre-commit.d")).expect("create modular dir");
    write(root.join(".githooks/pre-commit"), "run-parts .githooks/pre-commit.d\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_shared_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "HOOK-SHARED-04"
                && result.file() == Some(".githooks/pre-commit")
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "HOOK-SHARED-19"
                && result.file() == Some(".githooks/pre-commit")
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_runs_shared_source_checks_on_modular_scripts() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join(".githooks/pre-commit.d/10-rust.sh"), "echo cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_shared_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "HOOK-SHARED-11"
                && result.file() == Some(".githooks/pre-commit.d/10-rust.sh")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

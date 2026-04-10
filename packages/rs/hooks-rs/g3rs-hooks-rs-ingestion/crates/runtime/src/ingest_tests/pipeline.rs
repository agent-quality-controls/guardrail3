use std::fs;

use tempfile::tempdir;

fn write(path: impl AsRef<std::path::Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture");
}

#[test]
fn pipeline_reports_fmt_step_when_real_command_exists() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join(".githooks/pre-commit"), "cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_rs_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "HOOK-RS-01"
                && result.file() == Some(".githooks/pre-commit")
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_echoed_fmt_text_as_missing_step() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join(".githooks/pre-commit"), "echo \"cargo fmt --check\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_rs_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "HOOK-RS-01"
                && result.file() == Some(".githooks/pre-commit")
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_works_through_hooks_pre_commit_fallback() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join("hooks/pre-commit"), "cargo fmt --check\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_rs_source_checks::check)
        .collect::<Vec<_>>();

    assert!(
        results.iter().any(|result| {
            result.id() == "HOOK-RS-01"
                && result.file() == Some("hooks/pre-commit")
                && result.inventory()
        }),
        "{results:#?}"
    );
}

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

    let dispatcher_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-04")
        .collect::<Vec<_>>();
    let syntax_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-19")
        .collect::<Vec<_>>();

    assert_eq!(dispatcher_results.len(), 1, "{results:#?}");
    assert_eq!(syntax_results.len(), 1, "{results:#?}");
    assert!(
        dispatcher_results.iter().any(|result| {
            result.id() == "HOOK-SHARED-04"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher pattern present"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        syntax_results.iter().any(|result| {
            result.id() == "HOOK-SHARED-19"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher uses real executable syntax"
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

    let shebang_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-11")
        .collect::<Vec<_>>();

    assert_eq!(shebang_results.len(), 1, "{results:#?}");
    assert!(
        shebang_results.iter().any(|result| {
            result.id() == "HOOK-SHARED-11"
                && result.file() == Some(".githooks/pre-commit.d/10-rust.sh")
                && result.title() == "hook shebang missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_works_through_hooks_pre_commit_fallback() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join("hooks/pre-commit"), "#!/usr/bin/env bash\nset -e\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_shared_source_checks::check)
        .collect::<Vec<_>>();

    let shebang_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-11")
        .collect::<Vec<_>>();

    assert_eq!(shebang_results.len(), 1, "{results:#?}");
    assert!(
        shebang_results.iter().any(|result| {
            result.file() == Some("hooks/pre-commit")
                && result.title() == "valid hook shebang present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

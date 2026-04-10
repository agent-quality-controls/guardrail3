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

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-RS-01")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.id() == "HOOK-RS-01"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "cargo fmt --check step present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_hook_rs_10_quiet_for_single_crate_repo() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write(root.join(".githooks/pre-commit"), "cargo test\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_rs_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-RS-10")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.title() == "cargo test workspace scope not required" && result.inventory()
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

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-RS-01")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.id() == "HOOK-RS-01"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "cargo fmt --check step missing"
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

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-RS-01")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.id() == "HOOK-RS-01"
                && result.file() == Some("hooks/pre-commit")
                && result.title() == "cargo fmt --check step present"
                && result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_rs_config_trigger_for_guardrail3_rs_toml() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(
        root.join(".githooks/pre-commit"),
        "if echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    guardrail3 rs validate --staged .\nfi\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_rs_source_checks::check)
        .collect::<Vec<_>>();

    let rule_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-RS-16")
        .collect::<Vec<_>>();

    assert_eq!(rule_results.len(), 1, "{results:#?}");
    assert!(
        rule_results.iter().any(|result| {
            result.title() == "Rust config changes trigger hook validation" && result.inventory()
        }),
        "{results:#?}"
    );
}

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
        .flat_map(g3rs_hooks_source_checks::check)
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
        .flat_map(g3rs_hooks_source_checks::check)
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
        .flat_map(g3rs_hooks_source_checks::check)
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
        .flat_map(g3rs_hooks_source_checks::check)
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
        "if echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs rs validate --staged .\nfi\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
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
        .flat_map(g3rs_hooks_source_checks::check)
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
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "dispatcher pattern present"
                && result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        syntax_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
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
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let shebang_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-11")
        .collect::<Vec<_>>();

    assert_eq!(shebang_results.len(), 1, "{results:#?}");
    assert!(
        shebang_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit.d/10-rust.sh")
                && result.title() == "hook shebang missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_inert_g3rs_text_quiet_when_wrapped_command_executes() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(
        root.join(".githooks/pre-commit"),
        "# g3rs rs validate --staged .\nenv -i g3rs rs validate --staged .\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let inert_text_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-18")
        .collect::<Vec<_>>();

    assert!(inert_text_results.is_empty(), "{results:#?}");
}

#[test]
fn pipeline_reports_fail_open_wrapper_on_called_function() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = temp_dir.path();

    write(
        root.join(".githooks/pre-commit"),
        "run_tests() {\n    cargo test --workspace\n}\nrun_tests || true\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>();

    let fail_open_results = results
        .iter()
        .filter(|result| result.id() == "HOOK-SHARED-21")
        .collect::<Vec<_>>();

    assert_eq!(fail_open_results.len(), 1, "{results:#?}");
    assert!(
        fail_open_results.iter().any(|result| {
            result.file() == Some(".githooks/pre-commit")
                && result.title() == "critical hook command is fail-open"
                && result.line() == Some(4)
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

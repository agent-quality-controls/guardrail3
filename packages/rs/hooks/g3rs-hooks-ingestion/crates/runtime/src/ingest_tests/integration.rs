use std::fs;
use std::path::Path;
use std::process::Command;

use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
    assert!(status.success(), "git init should exit successfully");
}

fn repo_root(temp_dir: &tempfile::TempDir) -> &Path {
    let root = temp_dir.path();
    git_init(root);
    root
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture");
}

fn hook_results(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::ingest_for_source_checks(&crawl).expect("hook ingestion should succeed");
    inputs
        .iter()
        .flat_map(g3rs_hooks_source_checks::check)
        .collect::<Vec<_>>()
}

fn code_results(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        g3rs_code_ingestion::ingest_for_source_checks(&crawl).expect("code ingestion should succeed");
    inputs
        .iter()
        .flat_map(g3rs_code_source_checks::check)
        .collect::<Vec<_>>()
}

fn clippy_results(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        g3rs_clippy_ingestion::ingest_for_config_checks(&crawl).expect("clippy ingestion should succeed");
    g3rs_clippy_config_checks::check(&input)
}

#[test]
fn integration_reports_hook_breakage_when_rust_hook_is_misconfigured() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\n",
    );

    let results = hook_results(root);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-09"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "Rust guardrail validate step missing"
                && !result.inventory()
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-15"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "Rust config-change trigger coverage incomplete"
                && !result.inventory()
        }),
        "{results:#?}"
    );
}

#[test]
fn integration_aligns_hook_validation_step_with_source_family_breakage() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ng3rs validate --path .\n",
    );
    write(root.join("src/lib.rs"), "pub fn demo() { todo!(\"broken\"); }\n");

    let hook_results = hook_results(root);
    let code_results = code_results(root);

    assert!(
        hook_results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-09"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "Rust guardrail validate step present"
                && result.inventory()
        }),
        "{hook_results:#?}"
    );
    assert!(
        code_results.iter().any(|result| {
            result.id() == "RS-CODE-SOURCE-13"
                && result.file() == Some("src/lib.rs")
                && !result.inventory()
        }),
        "{code_results:#?}"
    );
}

#[test]
fn integration_aligns_hook_config_trigger_with_config_family_breakage() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );
    write(root.join("clippy.toml"), "too-many-lines-threshold = 1\n");

    let hook_results = hook_results(root);
    let clippy_results = clippy_results(root);

    assert!(
        hook_results.iter().any(|result| {
            result.id() == "RS-HOOKS-SOURCE-15"
                && result.file() == Some(".githooks/pre-commit")
                && result.title() == "Rust config changes trigger hook validation"
                && result.inventory()
        }),
        "{hook_results:#?}"
    );
    assert!(
        clippy_results.iter().any(|result| {
            result.id() == "RS-CLIPPY-CONFIG-03"
                && result.file() == Some("clippy.toml")
                && !result.inventory()
        }),
        "{clippy_results:#?}"
    );
}

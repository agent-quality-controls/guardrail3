use std::path::Path;

use g3rs_hooks_ingestion_assertions::run as assertions;
use guardrail3_check_types::G3CheckResult;
use tempfile::tempdir;

use super::helpers::{repo_root, write_fixture};

fn hook_results(root: &Path) -> Vec<G3CheckResult> {
    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        super::super::ingest_for_source_checks(&crawl).expect("hook ingestion should succeed");
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

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ncargo fmt --check\ncargo clippy -- -D warnings\ncargo deny check\ncargo test --workspace\n",
    );

    let results = hook_results(root);

    assertions::assert_present(
        &results,
        "RS-HOOKS-SOURCE-09",
        "Rust guardrail validate step missing",
        Some(".githooks/pre-commit"),
        false,
    );
    assertions::assert_present(
        &results,
        "RS-HOOKS-SOURCE-15",
        "Rust config-change trigger coverage incomplete",
        Some(".githooks/pre-commit"),
        false,
    );
}

#[test]
fn integration_aligns_hook_validation_step_with_source_family_breakage() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\ng3rs validate --path .\n",
    );
    write_fixture(root.join("src/lib.rs"), "pub fn demo() { todo!(\"broken\"); }\n");

    let hook_results = hook_results(root);
    let code_results = code_results(root);

    assertions::assert_present(
        &hook_results,
        "RS-HOOKS-SOURCE-09",
        "Rust guardrail validate step present",
        Some(".githooks/pre-commit"),
        true,
    );
    assertions::assert_present(
        &code_results,
        "RS-CODE-SOURCE-13",
        "todo! macro",
        Some("src/lib.rs"),
        false,
    );
}

#[test]
fn integration_aligns_hook_config_trigger_with_config_family_breakage() {
    let temp_dir = tempdir().expect("create temp dir");
    let root = repo_root(&temp_dir);

    write_fixture(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write_fixture(
        root.join(".githooks/pre-commit"),
        "#!/usr/bin/env bash\nset -e\nif echo \"$STAGED_FILES\" | grep -qE '(guardrail3-rs\\.toml|clippy\\.toml|\\.clippy\\.toml|deny\\.toml|\\.deny\\.toml|rustfmt\\.toml|\\.rustfmt\\.toml|rust-toolchain\\.toml)$'; then\n    g3rs validate --path .\nfi\n",
    );
    write_fixture(root.join("clippy.toml"), "too-many-lines-threshold = 1\n");

    let hook_results = hook_results(root);
    let clippy_results = clippy_results(root);

    assertions::assert_present(
        &hook_results,
        "RS-HOOKS-SOURCE-15",
        "Rust config changes trigger hook validation",
        Some(".githooks/pre-commit"),
        true,
    );
    assertions::assert_present(
        &clippy_results,
        "RS-CLIPPY-CONFIG-03",
        "too-many-lines-threshold wrong value",
        Some("clippy.toml"),
        false,
    );
}

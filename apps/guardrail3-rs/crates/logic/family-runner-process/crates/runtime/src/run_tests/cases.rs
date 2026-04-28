use std::fs;
use std::path::{Path, PathBuf};

use guardrail3_rs_app_types::SupportedFamily;
use guardrail3_rs_family_runner_process_assertions::run as assertions;

use super::super::{run, rust_hook_requirements};

#[test]
fn rust_hook_requirements_include_every_family_contract() {
    assertions::assert_rust_hook_contract_owners(rust_hook_requirements());
}

#[test]
fn hooks_runner_injects_family_contracts_into_source_checks() {
    let root = temp_workspace("g3rs-hooks-runner-contracts");
    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(
        root.join(".githooks/pre-commit"),
        "#!/bin/sh\ng3rs validate --path . --family hooks\n",
    );
    write(
        root.join(".git/config"),
        "[core]\nrepositoryformatversion = 0\nfilemode = true\nbare = false\n",
    );
    write(root.join(".git/HEAD"), "ref: refs/heads/main\n");

    let crawl =
        g3rs_workspace_crawl::crawl(root.as_path()).expect("test workspace crawl should succeed");
    let results = run(SupportedFamily::Hooks, &crawl).expect("hooks family should run");

    assertions::assert_hooks_runner_injects_contracts(&results);

    remove_workspace(root.as_path());
}

#[allow(clippy::disallowed_methods)] // reason: tests create and clean temporary workspaces through std fs/env APIs.
fn temp_workspace(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(root.as_path()).expect("stale test workspace should be removed");
    }
    fs::create_dir_all(root.as_path()).expect("test workspace should be created");
    root
}

#[allow(clippy::disallowed_methods)] // reason: tests write local fixture files without going through production filesystem ports.
fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("test fixture parent should be created");
    }
    fs::write(path, content).expect("test fixture should be written");
}

#[allow(clippy::disallowed_methods)] // reason: tests must remove temporary workspace directories they created.
fn remove_workspace(root: &Path) {
    fs::remove_dir_all(root).expect("test workspace should be removed");
}

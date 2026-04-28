use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use guardrail3_rs_app_types::SupportedFamily;

use super::{run, rust_hook_requirements};

#[test]
fn rust_hook_requirements_include_every_family_contract() {
    let owners = rust_hook_requirements()
        .into_iter()
        .map(|requirement| requirement.owner_family)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        owners,
        BTreeSet::from([
            "apparch".to_owned(),
            "arch".to_owned(),
            "cargo".to_owned(),
            "clippy".to_owned(),
            "code".to_owned(),
            "deny".to_owned(),
            "deps".to_owned(),
            "fmt".to_owned(),
            "garde".to_owned(),
            "release".to_owned(),
            "test".to_owned(),
            "toolchain".to_owned(),
            "topology".to_owned(),
        ])
    );
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

    assert!(
        results.iter().any(|result| {
            result.id() == "g3rs-hooks/required-contract-command-present"
                && !result.inventory()
                && result.message().contains("Owner families:")
        }),
        "hooks runner should inject family hook contracts into source checks"
    );

    remove_workspace(root.as_path());
}

#[allow(clippy::disallowed_methods)]
fn temp_workspace(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(root.as_path()).expect("stale test workspace should be removed");
    }
    fs::create_dir_all(root.as_path()).expect("test workspace should be created");
    root
}

#[allow(clippy::disallowed_methods)]
fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("test fixture parent should be created");
    }
    fs::write(path, content).expect("test fixture should be written");
}

#[allow(clippy::disallowed_methods)]
fn remove_workspace(root: &Path) {
    fs::remove_dir_all(root).expect("test workspace should be removed");
}

use std::fs;
use std::path::Path;

use guardrail3_rs_validate_command_assertions::marker_pairs::{
    assert_incomplete_adoption_marker_pair, assert_no_marker_pair_findings,
};
use tempfile::tempdir;

use super::super::check_repo;

#[expect(
    clippy::disallowed_methods,
    reason = "test fixture creation is isolated here so production code keeps centralized filesystem access"
)]
fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create fixture parent");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn marker_pairs_ignore_behavior_fixtures() {
    let temp_dir = tempdir().expect("create temporary repo");
    let root = temp_dir.path();
    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(
        root.join("behavior/fixtures/g3rs/demo/repo/guardrail3-rs.toml"),
        "profile = \"library\"\n",
    );

    let results = check_repo(root);

    assert_no_marker_pair_findings(&results);
}

#[test]
fn marker_pairs_still_report_real_incomplete_adoption() {
    let temp_dir = tempdir().expect("create temporary repo");
    let root = temp_dir.path();
    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(
        root.join("packages/demo/guardrail3-rs.toml"),
        "profile = \"library\"\n",
    );

    let results = check_repo(root);

    assert_incomplete_adoption_marker_pair(&results, "packages/demo/guardrail3-rs.toml");
}

#[test]
fn marker_pairs_report_workspace_without_guardrail_config() {
    let temp_dir = tempdir().expect("create temporary repo");
    let root = temp_dir.path();
    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(
        root.join("packages/demo/Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );

    let results = check_repo(root);

    assert_incomplete_adoption_marker_pair(&results, "packages/demo/Cargo.toml");
}

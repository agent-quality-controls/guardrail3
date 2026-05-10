#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures need direct filesystem access to build temp workspaces"
)]

use std::fs;

use g3rs_deny_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{git_init, write};

#[test]
fn pipeline_reports_missing_deny_config() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_missing_deny_config(&results);
}

#[test]
fn pipeline_inventories_selected_root_deny_config() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_selected_root_deny_config(&results);
}

#[test]
fn pipeline_reports_same_root_conflicts() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join(".deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(
        root.join(".cargo/deny.toml"),
        "[advisories]\nyanked = \"warn\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_same_root_conflicts(&results);
}

#[test]
fn pipeline_reports_selected_deny_parse_failures() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_selected_deny_parse_failures(&results);
}

#[test]
fn pipeline_reports_rust_policy_parse_failures_without_hiding_selected_coverage() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join("guardrail3-rs.toml"), "profile = \"invalid\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_rust_policy_parse_failures(&results);
}

#[test]
fn pipeline_reports_unreadable_selected_deny_file() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    let deny_path = root.join("deny.toml");
    write(&deny_path, "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let mut permissions = fs::metadata(&deny_path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&deny_path, permissions).expect("chmod fixture unreadable");

    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_unreadable_selected_deny_file(&results);
}

#[test]
fn pipeline_reports_unreadable_rust_policy() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    let guardrail_path = root.join("guardrail3-rs.toml");
    write(&guardrail_path, "profile = \"service\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let mut permissions = fs::metadata(&guardrail_path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&guardrail_path, permissions).expect("chmod fixture unreadable");

    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_unreadable_rust_policy(&results);
}

#[test]
fn pipeline_reports_shadowed_root_parse_failures() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    write(root.join(".deny.toml"), "[advisories");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_shadowed_root_parse_failures(&results);
}

#[test]
fn pipeline_reports_shadowed_root_unreadable_failures() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "[advisories]\nyanked = \"warn\"\n");
    let dot_deny_path = root.join(".deny.toml");
    write(&dot_deny_path, "[advisories]\nyanked = \"warn\"\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let mut permissions = fs::metadata(&dot_deny_path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(&dot_deny_path, permissions).expect("chmod fixture unreadable");

    let input =
        crate::run::ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deny_filetree_checks::check(&input);
    assertions::assert_shadowed_root_unreadable_failures(&results);
}

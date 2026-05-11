use g3rs_deps_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{git_init, write, write_executable};

#[test]
fn pipeline_reports_missing_dependency_allowlist_for_library() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\nresolver = \"2\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"library\"\n");
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_deps_config_checks::check)
        .collect::<Vec<_>>();

    assertions::assert_pipeline_missing_dependency_allowlist_for_library(&results);
}

#[test]
fn pipeline_reports_workspace_tool_presence_in_deps_config_lane() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\"]\nresolver = \"2\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let tools_dir = root.join("bin");
    write_executable(tools_dir.join("cargo-deny"), "#!/bin/sh\nexit 0\n");
    write_executable(tools_dir.join("cargo-machete"), "#!/bin/sh\nexit 0\n");
    write_executable(tools_dir.join("gitleaks"), "#!/bin/sh\nexit 0\n");

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs =
        crate::run::ingest_for_config_checks_with_path(&crawl, Some(tools_dir.as_os_str()))
            .expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_deps_config_checks::check)
        .collect::<Vec<_>>();

    assertions::assert_pipeline_workspace_tool_presence(&results);
}

#[test]
fn pipeline_emits_one_workspace_tool_result_set_even_with_multiple_crates() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/core\", \"crates/support\"]\nresolver = \"2\"\n",
    );
    write(root.join("guardrail3-rs.toml"), "profile = \"service\"\n");
    write(
        root.join("crates/core/Cargo.toml"),
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );
    write(
        root.join("crates/support/Cargo.toml"),
        "[package]\nname = \"support\"\nversion = \"0.1.0\"\n",
    );

    let crawl = g3_workspace_crawl::crawl(root).expect("crawl should succeed");
    let inputs = crate::run::ingest_for_config_checks_with_path(&crawl, None)
        .expect("ingestion should succeed");
    let results = inputs
        .iter()
        .flat_map(g3rs_deps_config_checks::check)
        .collect::<Vec<_>>();

    assertions::assert_pipeline_workspace_tool_absence(&results);
}

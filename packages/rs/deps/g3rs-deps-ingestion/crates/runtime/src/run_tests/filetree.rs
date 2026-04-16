use g3rs_deps_ingestion_assertions::run as assertions;
use g3rs_workspace_crawl::crawl;

use crate::run::ingest_for_file_tree_checks;

use super::helpers::{temp_workspace, write_file};

#[test]
fn filetree_entrypoint_ingests_root_lockfile_surface() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "target/\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input =
        ingest_for_file_tree_checks(&crawl).expect("deps filetree ingestion should succeed");

    assert_eq!(input.cargo_lock_rel_path, "Cargo.lock");
    assert!(input.cargo_lock_exists);
    assert!(!input.cargo_lock_ignored);
}

#[test]
fn pipeline_reports_missing_lockfile_as_error_for_service_profile() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assertions::assert_filetree_missing_lockfile_for_service(&results);
}

#[test]
fn pipeline_reports_missing_lockfile_as_info_for_library_profile() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assertions::assert_filetree_missing_lockfile_for_library(&results);
}

#[test]
fn pipeline_reports_ignored_lockfile() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "Cargo.lock\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assertions::assert_filetree_ignored_lockfile(&results);
}

#[test]
fn pipeline_respects_unignore_after_ignore() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "Cargo.lock\n!Cargo.lock\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assertions::assert_filetree_unignored_lockfile(&results);
}

#[test]
fn pipeline_uses_last_gitignore_match() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core"]
        "#,
    );
    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "service"
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        r#"
            [package]
            name = "core"
            version = "0.1.0"
        "#,
    );
    write_file(workspace.path(), "Cargo.lock", "# lockfile\n");
    write_file(workspace.path(), ".gitignore", "!Cargo.lock\nCargo.lock\n");

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let input = ingest_for_file_tree_checks(&crawl).expect("filetree ingestion should succeed");
    let results = g3rs_deps_filetree_checks::check(&input);

    assertions::assert_filetree_ignored_lockfile(&results);
}

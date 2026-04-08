use g3rs_workspace_crawl::crawl;

use crate::run::{IngestionError, ingest_for_ast_checks, ingest_for_config_checks, ingest_for_file_tree_checks};

use super::{temp_workspace, write_file};

#[test]
fn missing_guardrail_rs_file_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        "[workspace]\nmembers = [\"packages/*\"]\n",
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl).expect_err("missing guardrail3-rs.toml should fail");
    assert!(matches!(err, IngestionError::Guardrail3RsTomlNotFound));
}

#[test]
fn ast_and_file_tree_entrypoints_stay_stubbed() {
    let workspace = temp_workspace();
    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");

    assert!(matches!(
        ingest_for_ast_checks(&crawl),
        Err(IngestionError::AstIngestionNotImplemented)
    ));
    assert!(matches!(
        ingest_for_file_tree_checks(&crawl),
        Err(IngestionError::FileTreeIngestionNotImplemented)
    ));
}

#[test]
fn missing_declared_workspace_member_fails_ingestion() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [workspace]
            members = ["packages/core", "packages/missing"]
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
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl).expect_err("missing declared member should fail ingestion");
    assert!(matches!(
        err,
        IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("packages/missing")
    ));
}

#[test]
fn empty_allowlist_stays_present_while_missing_allowlist_stays_absent() {
    let workspace = temp_workspace();
    write_file(
        workspace.path(),
        "Cargo.toml",
        r#"
            [package]
            name = "root-crate"
            version = "0.1.0"

            [workspace]
            members = ["packages/*"]
        "#,
    );
    write_file(
        workspace.path(),
        "packages/core/Cargo.toml",
        "[package]\nname = \"core\"\nversion = \"0.1.0\"\n",
    );

    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
            allowed_deps = []
        "#,
    );
    let workspace_crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_for_config_checks(&workspace_crawl).expect("empty allowlist should ingest");
    assert!(
        inputs.iter().all(|input| input.allowlist_present),
        "empty allowlist should still be marked as present: {inputs:#?}"
    );
    assert!(
        inputs.iter().all(|input| input.allowed_deps.is_empty()),
        "empty allowlist should stay empty: {inputs:#?}"
    );

    write_file(
        workspace.path(),
        "guardrail3-rs.toml",
        r#"
            profile = "library"
        "#,
    );
    let workspace_crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let inputs = ingest_for_config_checks(&workspace_crawl).expect("missing allowlist should ingest");
    assert!(
        inputs.iter().all(|input| !input.allowlist_present),
        "missing allowlist should stay absent: {inputs:#?}"
    );
    assert!(
        inputs.iter().all(|input| input.allowed_deps.is_empty()),
        "missing allowlist should still have an empty vector payload: {inputs:#?}"
    );
}

#[test]
fn in_workspace_non_member_path_dependency_fails_ingestion() {
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

            [dependencies]
            helper = { path = "../../vendor/helper" }
        "#,
    );
    write_file(
        workspace.path(),
        "vendor/helper/Cargo.toml",
        "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n",
    );

    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");
    let err = ingest_for_config_checks(&crawl).expect_err("in-workspace non-member path should fail");
    assert!(matches!(
        err,
        IngestionError::NormalizationFailed { reason, .. }
            if reason.contains("in-workspace non-member")
    ));
}

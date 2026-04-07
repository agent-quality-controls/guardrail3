use g3rs_workspace_crawl::crawl;

use crate::run::{IngestionError, ingest_ast, ingest_config, ingest_file_tree};

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
    let err = ingest_config(&crawl).expect_err("missing guardrail3-rs.toml should fail");
    assert!(matches!(err, IngestionError::Guardrail3RsTomlNotFound));
}

#[test]
fn ast_and_file_tree_entrypoints_stay_stubbed() {
    let workspace = temp_workspace();
    let crawl = crawl(workspace.path()).expect("workspace crawl should succeed");

    assert!(matches!(
        ingest_ast(&crawl),
        Err(IngestionError::AstIngestionNotImplemented)
    ));
    assert!(matches!(
        ingest_file_tree(&crawl),
        Err(IngestionError::FileTreeIngestionNotImplemented)
    ));
}

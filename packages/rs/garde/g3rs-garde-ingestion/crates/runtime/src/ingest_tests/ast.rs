use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, content).expect("write fixture file");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed")
}

#[test]
fn ingests_root_scoped_ast_input() {
    let temp = tempdir().expect("create tempdir");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    write(root.join("src/lib.rs"), "use garde::Validate;\n#[derive(Validate)] struct Input;\n");
    write(root.join("tests/ignored.rs"), "#[test]\nfn ignored() {}\n");

    let crawl = crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");

    assert_eq!(input.guardrail_toml.rel_path, "guardrail3.toml");
    assert_eq!(input.source_files.len(), 1, "{input:#?}");
    assert_eq!(input.source_files[0].rel_path, "src/lib.rs");
}

#[test]
fn pipeline_reports_malformed_source_via_garde_10() {
    let temp = tempdir().expect("create tempdir");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write(root.join("guardrail3.toml"), "[profile]\nname = \"service\"\n");
    write(root.join("src/lib.rs"), "fn broken( {\n");

    let crawl = crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10" && result.file() == Some("src/lib.rs")
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_reports_malformed_guardrail_via_garde_10() {
    let temp = tempdir().expect("create tempdir");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write(root.join("guardrail3.toml"), "[[broken");
    write(
        root.join("src/lib.rs"),
        "use sqlx::query_as;\nfn load() { let _ = query_as!(User, \"select 1\"); }\n",
    );

    let crawl = crawl(root);
    let input = crate::ingest_for_ast_checks(&crawl).expect("AST ingestion should succeed");
    let results = g3rs_garde_ast_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-10" && result.file() == Some("guardrail3.toml")
        }),
        "{results:#?}"
    );
}

#[test]
fn ast_ingestion_fails_when_guardrail_is_missing() {
    let temp = tempdir().expect("create tempdir");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );
    write(root.join("src/lib.rs"), "fn ok() {}\n");

    let crawl = crawl(root);
    let result = crate::ingest_for_ast_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::GuardrailTomlNotFound)),
        "{result:#?}"
    );
}

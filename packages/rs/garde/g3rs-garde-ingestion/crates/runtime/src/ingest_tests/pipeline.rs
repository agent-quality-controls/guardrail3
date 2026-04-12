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
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

#[test]
fn pipeline_reports_missing_garde_dependency() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-01" && result.title() == "garde dependency missing"
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_warns_when_clippy_is_missing_for_garde_root() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-02"
                && result.title() == "cannot verify core garde method bans"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-03"
                && result.title() == "cannot verify garde extractor bans"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-04"
                && result.title() == "cannot verify reqwest garde ban"
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-05"
                && result.title() == "cannot verify additional garde method bans"
        }),
        "{results:#?}"
    );
}

#[test]
fn pipeline_keeps_ban_rules_quiet_when_garde_is_missing() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\nversion = \"0.1.0\"\n",
    );
    write(root.join("clippy.toml"), "disallowed-methods = []\ndisallowed-types = []\n");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-01" && result.title() == "garde dependency missing"
        }),
        "{results:#?}"
    );
    assert!(
        results
            .iter()
            .all(|result| !matches!(result.id(), "RS-GARDE-CONFIG-02" | "RS-GARDE-CONFIG-03" | "RS-GARDE-CONFIG-04" | "RS-GARDE-CONFIG-05")),
        "{results:#?}"
    );
}

#[test]
fn pipeline_warns_when_clippy_is_invalid_for_garde_root() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = []\n[workspace.dependencies]\ngarde = \"0.22\"\n",
    );
    write(root.join("clippy.toml"), "{{{{not valid toml}}}}");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_garde_config_checks::check(&input);

    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-02"
                && result.title() == "cannot verify core garde method bans"
                && result.file() == Some("clippy.toml")
        }),
        "{results:#?}"
    );
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-GARDE-CONFIG-05"
                && result.title() == "cannot verify additional garde method bans"
                && result.file() == Some("clippy.toml")
        }),
        "{results:#?}"
    );
}

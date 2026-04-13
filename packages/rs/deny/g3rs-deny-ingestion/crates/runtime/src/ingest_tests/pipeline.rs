use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_deny_config_checks_assertions::rs_deny_config_05_graph_no_default_features as graph_no_default_features_assertions;
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
fn pipeline_reports_missing_graph_section() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_deny_config_checks::check(&input);

    graph_no_default_features_assertions::assert_findings(
        &results,
        &[graph_no_default_features_assertions::error(
            "[graph] section missing",
            "`deny.toml` must contain `[graph]` coverage settings.",
            "deny.toml",
            false,
        )],
    );
}

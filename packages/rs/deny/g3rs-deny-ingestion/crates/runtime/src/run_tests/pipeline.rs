use g3rs_deny_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{git_init, write};

#[test]
fn pipeline_reports_missing_graph_section() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("deny.toml"), "");

    let crawl = g3rs_workspace_crawl::crawl(root).expect("crawl should succeed");
    let input = crate::run::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_deny_config_checks::check(&input);
    assertions::assert_missing_graph_section(&results);
}

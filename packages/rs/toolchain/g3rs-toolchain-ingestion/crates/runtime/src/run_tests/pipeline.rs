use g3rs_toolchain_ingestion_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::{crawl, git_init, write};

#[test]
fn pipeline_reports_nightly_toolchain_channel() {
    let temp = tempdir().expect("create temporary workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"nightly\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );

    let crawl = crawl(root);
    let input = super::ingest_for_config_checks(&crawl).expect("ingestion should succeed");
    let results = g3rs_toolchain_config_checks::check(&input);
    assertions::assert_nightly_toolchain_channel(&results);
}

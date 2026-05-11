use std::fs;

use g3_workspace_crawl::{G3WorkspaceCrawl, crawl};
use g3rs_arch_config_checks::check as check_config;
use g3rs_arch_types::G3RsArchConfigChecksInput;
use guardrail3_check_types::G3CheckResult;
use tempfile::{TempDir, tempdir};

pub(super) fn temp_workspace_root() -> TempDir {
    tempdir().expect("create temporary workspace root for arch config ingestion test")
}

/// Write a fixture file. The centralized fs ban targets production code paths; these
/// test-only helpers materialize real on-disk inputs to exercise the workspace crawl.
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper materializes real on-disk files to exercise the workspace crawl; the centralized fs bans target production code paths only"
)]
pub(super) fn write_file(root: &TempDir, rel: &str, content: &str) {
    fs::write(root.path().join(rel), content)
        .expect("write fixture file for arch config ingestion test");
}

/// Materialize a fixture directory. See [`write_file`].
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper materializes real on-disk directories to exercise the workspace crawl; the centralized fs bans target production code paths only"
)]
pub(super) fn make_dir(root: &TempDir, rel: &str) {
    fs::create_dir_all(root.path().join(rel))
        .expect("create fixture directory for arch config ingestion test");
}

fn crawl_workspace(root: &TempDir) -> G3WorkspaceCrawl {
    crawl(root.path()).expect("crawl fixture workspace for arch config ingestion test")
}

pub(super) fn config_inputs(root: &TempDir) -> Vec<G3RsArchConfigChecksInput> {
    crate::config::ingest_for_config_checks(&crawl_workspace(root))
        .expect("ingest config checks from fixture workspace for arch config ingestion test")
}

pub(super) fn config_results(root: &TempDir) -> Vec<G3CheckResult> {
    let inputs = config_inputs(root);
    let first = inputs
        .first()
        .expect("config ingestion should produce at least one input");
    check_config(first)
}

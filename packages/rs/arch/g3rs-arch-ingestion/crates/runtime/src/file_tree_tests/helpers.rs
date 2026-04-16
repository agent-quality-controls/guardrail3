use std::fs;

use g3rs_arch_file_tree_checks::check as check_file_tree;
use g3rs_arch_types::G3RsArchFileTreeChecksInput;
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, crawl};
use guardrail3_check_types::G3CheckResult;
use tempfile::{TempDir, tempdir};

pub(super) fn temp_workspace_root() -> TempDir {
    tempdir().expect("create temporary workspace root for arch file tree ingestion test")
}

pub(super) fn write_file(root: &TempDir, rel: &str, content: &str) {
    fs::write(root.path().join(rel), content)
        .expect("write fixture file for arch file tree ingestion test");
}

pub(super) fn make_dir(root: &TempDir, rel: &str) {
    fs::create_dir_all(root.path().join(rel))
        .expect("create fixture directory for arch file tree ingestion test");
}

fn crawl_workspace(root: &TempDir) -> G3RsWorkspaceCrawl {
    crawl(root.path()).expect("crawl fixture workspace for arch file tree ingestion test")
}

pub(super) fn file_tree_input(root: &TempDir) -> G3RsArchFileTreeChecksInput {
    crate::file_tree::ingest_for_file_tree_checks(&crawl_workspace(root))
        .expect("ingest file tree checks from fixture workspace for arch file tree ingestion test")
}

pub(super) fn file_tree_results(root: &TempDir) -> Vec<G3CheckResult> {
    let input = file_tree_input(root);
    check_file_tree(&input)
}

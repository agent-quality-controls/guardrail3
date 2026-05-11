#![allow(
    clippy::disallowed_methods,
    reason = "test fixture builder writes synthetic workspace files via std::fs::write/create_dir_all into a tempdir; the fs.rs boundary applies to runtime code, not to per-test fixture authoring"
)]

use std::path::Path;

use g3_workspace_crawl::{G3WorkspaceCrawl, crawl};
use g3rs_apparch_types::G3RsApparchConfigChecksInput;
use tempfile::{TempDir, tempdir};

pub(super) fn temp_workspace() -> TempDir {
    tempdir().expect("create temporary workspace root for config ingestion test")
}

pub(super) fn crawl_workspace(root: &Path) -> G3WorkspaceCrawl {
    crawl(root).expect("crawl test workspace for config ingestion")
}

pub(super) fn config_input(root: &Path) -> G3RsApparchConfigChecksInput {
    super::super::ingest_for_config_checks(&crawl_workspace(root))
        .expect("ingest config input from test workspace")
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .expect("create fixture parent directory for config ingestion test");
    }
    std::fs::write(path, content).expect("write config ingestion test fixture");
}

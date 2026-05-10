#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures need direct filesystem and process access to build temp workspaces"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

pub(super) fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    // Tests intentionally exercise workspaces with and without a root `Cargo.toml`
    // (the fmt rules emit `Missing` state for the absent case). Use `crawl_any_root`
    // so the crawl boundary does not reject manifestless fixtures.
    g3rs_workspace_crawl::crawl_any_root(root)
        .expect("crawl should succeed on valid test workspace")
}

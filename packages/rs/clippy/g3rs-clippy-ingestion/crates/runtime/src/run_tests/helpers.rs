use std::fs;
use std::path::Path;
use std::process::Command;

/// Initialize a git repository under `path` so the workspace crawl recognizes it.
///
/// Tests for ingestion need real on-disk fixtures including git metadata to
/// exercise the crawler end-to-end; the centralized fs/process bans apply to
/// production code paths only, so this test-only helper opts out with reason.
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper materializes real on-disk git+files to exercise the workspace crawl; the centralized fs/process bans target production code paths only"
)]
pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

/// Write a test fixture file, materializing any missing parent directories.
///
/// See [`git_init`] for the rationale on the disallowed-methods opt-out.
#[expect(
    clippy::disallowed_methods,
    reason = "test-only fixture helper materializes real on-disk git+files to exercise the workspace crawl; the centralized fs/process bans target production code paths only"
)]
pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

/// Crawl the workspace rooted at `root`.
///
/// Materializes a minimal `Cargo.toml` if one is absent so the crawler's
/// workspace-manifest anchor requirement is satisfied for ingestion fixtures
/// that focus on clippy/policy/cargo-config files only.
pub(super) fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    let cargo_toml = root.join("Cargo.toml");
    if !cargo_toml.exists() {
        write(&cargo_toml, "[workspace]\nmembers = []\nresolver = \"2\"\n");
    }
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

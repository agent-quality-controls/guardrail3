/// Select required config entries from a workspace crawl.
use g3rs_workspace_crawl::{G3RsWorkspaceCrawl, G3RsWorkspaceEntry};

/// Find the workspace-root `rustfmt.toml` in the crawl result.
///
/// Only `rustfmt.toml` is accepted. The dot-prefixed `.rustfmt.toml` variant
/// is a policy violation handled by the app layer and is never ingested here.
pub(crate) fn select_rustfmt_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("rustfmt.toml")
}

/// Find the workspace-root `Cargo.toml` in the crawl result.
pub(crate) fn select_cargo_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("Cargo.toml")
}

/// Find the workspace-root `rust-toolchain.toml` in the crawl result.
pub(crate) fn select_toolchain_toml(crawl: &G3RsWorkspaceCrawl) -> Option<&G3RsWorkspaceEntry> {
    crawl.root_file("rust-toolchain.toml")
}

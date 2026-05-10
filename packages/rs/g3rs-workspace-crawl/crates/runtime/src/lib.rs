//! Workspace crawl runtime: discovers files in a Cargo workspace honouring
//! gitignore semantics with targeted recovery for ignored-but-relevant files.

/// Two-phase crawl implementation.
mod crawl;
/// Centralized filesystem boundary.
mod fs;
/// Query helpers over a `G3RsWorkspaceCrawl`.
mod query;
/// Recovery list (banned dirs and ignored-but-relevant files).
mod recovery;
/// Public crawl entry points and error type.
mod run;
/// Internal entry construction helpers.
mod support;

#[cfg(feature = "crawl")]
pub use g3rs_workspace_crawl_types::{
    G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
    G3RsWorkspacePath,
};
#[cfg(feature = "crawl")]
pub use query::{entry, files_with_extension, root_file};
#[cfg(feature = "crawl")]
pub use run::{G3RsWorkspaceCrawlError, crawl, crawl_any_root};

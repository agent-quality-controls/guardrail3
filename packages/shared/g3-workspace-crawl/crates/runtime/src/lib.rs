//! Runtime crate that crawls one workspace root and produces inventory entries.

/// Filesystem traversal entry point invoked by the crawl driver.
mod crawl;
/// Centralized filesystem boundary for crawl primitives.
mod fs;
/// Read-only query helpers over a completed crawl.
mod query;
/// Best-effort recovery rules for partially readable workspaces.
mod recovery;
/// Public crawl entry-point and error surface.
mod run;
/// Support utilities shared between crawl and recovery.
mod support;

#[cfg(feature = "crawl")]
pub use g3_workspace_crawl_types::{
    G3WorkspaceCrawl, G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState,
    G3WorkspacePath,
};
#[cfg(feature = "crawl")]
pub use query::{entry, files_with_extension, root_file};
#[cfg(feature = "crawl")]
pub use run::{G3WorkspaceCrawlError, crawl};

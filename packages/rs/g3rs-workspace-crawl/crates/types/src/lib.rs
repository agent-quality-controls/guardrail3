//! Shared types for the g3rs workspace crawl shared package.

/// Crawl-tree aggregate type definitions.
mod crawl;
/// Per-entry workspace path and metadata definitions.
mod entry;

#[cfg(feature = "api")]
pub use crawl::G3RsWorkspaceCrawl;
#[cfg(feature = "api")]
pub use entry::{
    G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState, G3RsWorkspacePath,
};

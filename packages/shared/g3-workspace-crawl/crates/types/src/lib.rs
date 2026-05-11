//! Shared types for the g3rs workspace crawl shared package.

/// Crawl-tree aggregate type definitions.
mod crawl;
/// Per-entry workspace path and metadata definitions.
mod entry;

#[cfg(feature = "api")]
pub use crawl::G3WorkspaceCrawl;
#[cfg(feature = "api")]
pub use entry::{G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, G3WorkspacePath};

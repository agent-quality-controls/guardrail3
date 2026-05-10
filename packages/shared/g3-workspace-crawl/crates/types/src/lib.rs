//! Shared types for the g3-workspace-crawl ingestion outputs.

/// Crawl-result aggregate types for workspace tree traversal.
mod crawl;
/// Entry-level types describing one filesystem node visited by the crawler.
mod entry;

#[cfg(feature = "api")]
pub use crawl::G3WorkspaceCrawl;
#[cfg(feature = "api")]
pub use entry::{G3WorkspaceEntry, G3WorkspaceEntryKind, G3WorkspaceIgnoreState, G3WorkspacePath};

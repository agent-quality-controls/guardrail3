//! Reusable assertion helpers for `g3-workspace-crawl` callers.

#[cfg(feature = "crawl")]
use g3_workspace_crawl_runtime as _;
#[cfg(feature = "crawl")]
pub mod query;
#[cfg(feature = "crawl")]
pub mod run;

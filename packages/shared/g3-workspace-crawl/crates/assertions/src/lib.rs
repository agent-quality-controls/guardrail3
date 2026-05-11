//! Test-time assertions for the workspace crawl runtime.

#[cfg(feature = "crawl")]
use g3_workspace_crawl_runtime as _;
#[cfg(feature = "crawl")]
pub mod query;
#[cfg(feature = "crawl")]
pub mod run;

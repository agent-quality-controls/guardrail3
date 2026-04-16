#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold stays focused on shared crawl proof helpers"
)]

#[cfg(feature = "crawl")]
use g3rs_workspace_crawl_runtime as _;
#[cfg(feature = "crawl")]
pub mod crawl;

#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold will gain ingestion-specific helpers later"
)]

#[cfg(feature = "ingest")]
use g3rs_deps_ingestion_runtime as _;

#[cfg(feature = "ingest")]
pub mod run;

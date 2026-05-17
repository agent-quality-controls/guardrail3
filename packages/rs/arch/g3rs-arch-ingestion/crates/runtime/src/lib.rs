//! Internal crate.

/// config module.
mod config;
/// error module.
mod error;
/// file tree module.
mod file_tree;
mod fs;
/// run module.
mod run;
/// source module.
mod source;
/// source-tier facade-surface analysis.
mod source_facade;
/// source-tier `#[path]` attribute collection.
mod source_path_attr;
/// shared syn helpers for source-tier ingestion.
mod source_syn_helpers;
/// view module.
mod view;
/// workspace module.
mod workspace;

#[cfg(feature = "ingest")]
pub use run::{
    G3RsArchIngestionError, ingest_for_config_checks, ingest_for_file_tree_checks,
    ingest_for_source_checks,
};

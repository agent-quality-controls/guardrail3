//! Assertion helpers for arch-ingestion tests; wraps `g3rs_arch_ingestion_runtime` outputs
//! with snapshot-friendly accessors for config, file-tree, and source surfaces.

#[cfg(feature = "ingest")]
use g3rs_arch_ingestion_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "ingest")]
pub mod config;
#[cfg(feature = "ingest")]
pub mod file_tree;
#[cfg(feature = "ingest")]
pub mod source;

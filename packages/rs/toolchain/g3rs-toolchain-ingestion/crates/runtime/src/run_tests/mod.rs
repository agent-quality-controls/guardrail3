pub(super) use super::{IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks};

/// Covers the smallest happy-path and failure-path ingest cases.
mod basic;
/// Covers dependency-specific ingest scenarios.
mod deps;
/// Covers file tree projection and root-file selection cases.
mod filetree;
/// Shared fixture setup used across run sidecar tests.
mod helpers;
/// Covers end-to-end ingestion wiring across all derived outputs.
mod pipeline;
/// Covers ingest behavior against real repo fixture layouts.
mod real_workspaces;

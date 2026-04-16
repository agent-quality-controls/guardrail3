#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::wildcard_enum_match_arm,
    clippy::disallowed_methods,
    reason = "test code uses expect/panic for assertions and direct fs access for fixtures"
)]

/// Covers the smallest happy-path and failure-path ingest cases.
mod basic;
/// Covers dependency-specific ingest scenarios.
mod deps;
/// Covers file tree projection and root-file selection cases.
mod filetree;
/// Covers end-to-end ingestion wiring across all derived outputs.
mod pipeline;
/// Covers ingest behavior against real repo fixture layouts.
mod real_workspaces;

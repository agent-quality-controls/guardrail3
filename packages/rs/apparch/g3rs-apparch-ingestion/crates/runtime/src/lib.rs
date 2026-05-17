//! Runtime crate for g3rs-apparch ingestion: builds typed apparch facts from a workspace.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::type_complexity,
    clippy::wildcard_enum_match_arm,
    reason = "ingestion modules carry the full typed apparch model with many private helpers and tuple-shaped state; doc-per-private-item adds no signal over the module-level docs. type_complexity arises from the typed workspace tuples that thread through ingestion stages. wildcard_enum_match_arm fires on policy-state matches where new policy variants must default to the existing fallback path."
)]

/// Filesystem boundary helpers.
mod fs;
/// Top-level ingestion orchestration and per-stage submodules.
mod run;
/// Read-only projection helpers over typed apparch facts.
mod view;

#[cfg(feature = "ingest")]
pub use run::{G3RsApparchIngestionError, ingest_for_config_checks, ingest_for_source_checks};

#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive dep `siphasher` resolves to 0.3.11 (via swc_common in g3ts-arch-ingestion's SWC-based parser) and 1.0.2 (via other dependents); both versions are pinned by upstream crates this app does not own"
)]

/// Runs the structure-oriented family group against one workspace crawl.
mod run;

#[cfg(feature = "api")]
pub use run::run;

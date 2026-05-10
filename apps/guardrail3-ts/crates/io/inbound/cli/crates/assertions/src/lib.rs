#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive dep `siphasher` resolves to 0.3.11 (via swc_common in g3ts-arch-ingestion's SWC-based parser pulled in via the g3ts CLI dependency) and 1.0.2 (via other dependents); both versions are pinned by upstream crates this app does not own"
)]

#[cfg(feature = "checks")]
use g3ts as _;

#[cfg(feature = "checks")]
pub mod cli;
#[cfg(feature = "checks")]
pub mod run;

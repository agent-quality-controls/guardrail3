#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive: hashbrown and siphasher pulled at different versions by upstream parser crates; pinning here would break cross-workspace ingestion contract"
)]

use g3ts_astro_media_ingestion_runtime as _;

#[cfg(feature = "api")]
pub mod eslint;

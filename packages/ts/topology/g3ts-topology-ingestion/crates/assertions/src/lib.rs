//! Assertions consumed by tests of the g3ts topology ingestion runtime.

// Anchor dependency: assertions must depend on the runtime they assert against,
// so internal and external tests share the same proof surface.
use g3ts_topology_ingestion_runtime as _;

#[cfg(feature = "checks")]
pub mod run;

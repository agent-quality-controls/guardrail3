//! Facade crate re-exporting the g3ts topology ingestion API.

#[cfg(feature = "api")]
pub use g3ts_topology_ingestion_runtime::{
    G3TsTopologyIngestionError, ingest_for_file_tree_checks,
};
#[cfg(feature = "api")]
pub use g3ts_topology_ingestion_types as types;

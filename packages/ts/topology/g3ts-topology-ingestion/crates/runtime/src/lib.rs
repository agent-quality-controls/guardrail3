//! Runtime for the g3ts topology ingestion family.

/// Walks an adopted TS unit's tree and produces topology facts.
mod run;

#[cfg(feature = "ingest")]
pub use g3ts_topology_ingestion_types::G3TsTopologyIngestionError;
#[cfg(feature = "ingest")]
pub use run::ingest_for_file_tree_checks;

#[cfg(test)]
use g3ts_topology_ingestion_assertions as _;
#[cfg(test)]
use tempfile as _;

/// Owns the internal ingestion error type so runtime and facade share one contract.
mod error;

pub use g3rs_topology_types::{
    G3RsTopologyFileTreeChecksInput,
};

#[cfg(feature = "api")]
pub use error::G3RsTopologyIngestionError;

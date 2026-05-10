//! Public types shared by the g3ts topology ingestion runtime and its consumers.

#[cfg(feature = "api")]
mod error;

#[cfg(feature = "api")]
pub use error::G3TsTopologyIngestionError;

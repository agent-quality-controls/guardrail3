/// Owns the internal ingestion error type so runtime and facade share one contract.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsTopologyIngestionError;

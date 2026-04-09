/// Error types for fmt ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsFmtIngestionError;

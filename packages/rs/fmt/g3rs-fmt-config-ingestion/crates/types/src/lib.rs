/// Error types for fmt config ingestion.
mod error;

#[cfg(feature = "api")]
pub use error::G3RsFmtConfigIngestionError;
